#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationKind(pub i32);
impl ActivationKind {
    pub const Launch: Self = Self(0i32);
    pub const Search: Self = Self(1i32);
    pub const ShareTarget: Self = Self(2i32);
    pub const File: Self = Self(3i32);
    pub const Protocol: Self = Self(4i32);
    pub const FileOpenPicker: Self = Self(5i32);
    pub const FileSavePicker: Self = Self(6i32);
    pub const CachedFileUpdater: Self = Self(7i32);
    pub const ContactPicker: Self = Self(8i32);
    pub const Device: Self = Self(9i32);
    pub const PrintTaskSettings: Self = Self(10i32);
    pub const CameraSettings: Self = Self(11i32);
    pub const RestrictedLaunch: Self = Self(12i32);
    pub const AppointmentsProvider: Self = Self(13i32);
    pub const Contact: Self = Self(14i32);
    pub const LockScreenCall: Self = Self(15i32);
    pub const VoiceCommand: Self = Self(16i32);
    pub const LockScreen: Self = Self(17i32);
    pub const PickerReturned: Self = Self(1000i32);
    pub const WalletAction: Self = Self(1001i32);
    pub const PickFileContinuation: Self = Self(1002i32);
    pub const PickSaveFileContinuation: Self = Self(1003i32);
    pub const PickFolderContinuation: Self = Self(1004i32);
    pub const WebAuthenticationBrokerContinuation: Self = Self(1005i32);
    pub const WebAccountProvider: Self = Self(1006i32);
    pub const ComponentUI: Self = Self(1007i32);
    pub const ProtocolForResults: Self = Self(1009i32);
    pub const ToastNotification: Self = Self(1010i32);
    pub const Print3DWorkflow: Self = Self(1011i32);
    pub const DialReceiver: Self = Self(1012i32);
    pub const DevicePairing: Self = Self(1013i32);
    pub const UserDataAccountsProvider: Self = Self(1014i32);
    pub const FilePickerExperience: Self = Self(1015i32);
    pub const LockScreenComponent: Self = Self(1016i32);
    pub const ContactPanel: Self = Self(1017i32);
    pub const PrintWorkflowForegroundTask: Self = Self(1018i32);
    pub const GameUIProvider: Self = Self(1019i32);
    pub const StartupTask: Self = Self(1020i32);
    pub const CommandLineLaunch: Self = Self(1021i32);
    pub const BarcodeScannerProvider: Self = Self(1022i32);
    pub const PrintSupportJobUI: Self = Self(1023i32);
    pub const PrintSupportSettingsUI: Self = Self(1024i32);
    pub const PhoneCallActivation: Self = Self(1025i32);
    pub const VpnForeground: Self = Self(1026i32);
}
impl ::core::marker::Copy for ActivationKind {}
impl ::core::clone::Clone for ActivationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ActivationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivationKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ActivationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationExecutionState(pub i32);
impl ApplicationExecutionState {
    pub const NotRunning: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspended: Self = Self(2i32);
    pub const Terminated: Self = Self(3i32);
    pub const ClosedByUser: Self = Self(4i32);
}
impl ::core::marker::Copy for ApplicationExecutionState {}
impl ::core::clone::Clone for ApplicationExecutionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationExecutionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ApplicationExecutionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationExecutionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationExecutionState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ApplicationExecutionState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ApplicationExecutionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppointmentsProviderAddAppointmentActivatedEventArgs(::windows_core::IUnknown);
impl AppointmentsProviderAddAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddAppointmentOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentsProvider::AddAppointmentOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderAddAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs;{a2861367-cee5-4e4d-9ed7-41c34ec18b02})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentsProviderAddAppointmentActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderAddAppointmentActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderAddAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderAddAppointmentActivatedEventArgs {}
#[repr(transparent)]
pub struct AppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows_core::IUnknown);
impl AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAppointmentOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderRemoveAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs;{751f3ab8-0b8e-451c-9f15-966e699bac25})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentsProviderRemoveAppointmentActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderRemoveAppointmentActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[repr(transparent)]
pub struct AppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows_core::IUnknown);
impl AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceAppointmentOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderReplaceAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs;{1551b7d4-a981-4067-8a62-0524e4ade121})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentsProviderReplaceAppointmentActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderReplaceAppointmentActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[repr(transparent)]
pub struct AppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows_core::IUnknown);
impl AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InstanceStartDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    pub fn LocalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LocalId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RoamingId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RoamingId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderShowAppointmentDetailsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs;{3958f065-9841-4ca5-999b-885198b9ef2a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[repr(transparent)]
pub struct AppointmentsProviderShowTimeFrameActivatedEventArgs(::windows_core::IUnknown);
impl AppointmentsProviderShowTimeFrameActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TimeToShow(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeToShow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderShowTimeFrameActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs;{9baeaba6-0e0b-49aa-babc-12b1dc774986})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAppointmentsProviderShowTimeFrameActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderShowTimeFrameActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[repr(transparent)]
pub struct BackgroundActivatedEventArgs(::windows_core::IUnknown);
impl BackgroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> ::windows_core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TaskInstance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Background::IBackgroundTaskInstance>(result__)
        }
    }
}
impl ::core::clone::Clone for BackgroundActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundActivatedEventArgs {}
impl ::core::fmt::Debug for BackgroundActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs;{ab14bee0-e760-440e-a91c-44796de3a92d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBackgroundActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs";
}
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BackgroundActivatedEventArgs> for IBackgroundActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: BackgroundActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BackgroundActivatedEventArgs> for IBackgroundActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &BackgroundActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundActivatedEventArgs> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBackgroundActivatedEventArgs> for &BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IBackgroundActivatedEventArgs> {
        ::core::convert::TryInto::<IBackgroundActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BackgroundActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundActivatedEventArgs {}
#[repr(transparent)]
pub struct BarcodeScannerPreviewActivatedEventArgs(::windows_core::IUnknown);
impl BarcodeScannerPreviewActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn ConnectionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerPreviewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerPreviewActivatedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerPreviewActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerPreviewActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs;{6772797c-99bf-4349-af22-e4123560371c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IBarcodeScannerPreviewActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs";
}
impl ::core::convert::From<BarcodeScannerPreviewActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: BarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerPreviewActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: BarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBarcodeScannerPreviewActivatedEventArgs> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IBarcodeScannerPreviewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IBarcodeScannerPreviewActivatedEventArgs> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IBarcodeScannerPreviewActivatedEventArgs> {
        ::core::convert::TryInto::<IBarcodeScannerPreviewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerPreviewActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerPreviewActivatedEventArgs {}
#[repr(transparent)]
pub struct CachedFileUpdaterActivatedEventArgs(::windows_core::IUnknown);
impl CachedFileUpdaterActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> ::windows_core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CachedFileUpdaterUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
}
impl ::core::clone::Clone for CachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CachedFileUpdaterActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CachedFileUpdaterActivatedEventArgs {}
impl ::core::fmt::Debug for CachedFileUpdaterActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileUpdaterActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs;{d06eb1c7-3805-4ecb-b757-6cf15e26fef3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICachedFileUpdaterActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs";
}
impl ::core::convert::From<CachedFileUpdaterActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CachedFileUpdaterActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICachedFileUpdaterActivatedEventArgs> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ICachedFileUpdaterActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICachedFileUpdaterActivatedEventArgs> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ICachedFileUpdaterActivatedEventArgs> {
        ::core::convert::TryInto::<ICachedFileUpdaterActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CachedFileUpdaterActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterActivatedEventArgs {}
#[repr(transparent)]
pub struct CameraSettingsActivatedEventArgs(::windows_core::IUnknown);
impl CameraSettingsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn VideoDeviceController(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn VideoDeviceExtension(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceExtension)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for CameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CameraSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for CameraSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs;{fb67a508-2dad-490a-9170-dca036eb114b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICameraSettingsActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs";
}
impl ::core::convert::From<CameraSettingsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CameraSettingsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: CameraSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &CameraSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CameraSettingsActivatedEventArgs> for ICameraSettingsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: CameraSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CameraSettingsActivatedEventArgs> for ICameraSettingsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &CameraSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICameraSettingsActivatedEventArgs> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ICameraSettingsActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICameraSettingsActivatedEventArgs> for &CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ICameraSettingsActivatedEventArgs> {
        ::core::convert::TryInto::<ICameraSettingsActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CameraSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CameraSettingsActivatedEventArgs {}
#[repr(transparent)]
pub struct CommandLineActivatedEventArgs(::windows_core::IUnknown);
impl CommandLineActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Operation(&self) -> ::windows_core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CommandLineActivationOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for CommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CommandLineActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommandLineActivatedEventArgs {}
impl ::core::fmt::Debug for CommandLineActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommandLineActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CommandLineActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs;{4506472c-006a-48eb-8afb-d07ab25e3366})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ICommandLineActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs";
}
impl ::core::convert::From<CommandLineActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: CommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommandLineActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: CommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for ICommandLineActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for ICommandLineActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommandLineActivatedEventArgs> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ICommandLineActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ICommandLineActivatedEventArgs> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ICommandLineActivatedEventArgs> {
        ::core::convert::TryInto::<ICommandLineActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CommandLineActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CommandLineActivatedEventArgs {}
#[repr(transparent)]
pub struct CommandLineActivationOperation(::windows_core::IUnknown);
impl CommandLineActivationOperation {
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CurrentDirectoryPath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentDirectoryPath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetExitCode(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExitCode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ExitCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CommandLineActivationOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CommandLineActivationOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommandLineActivationOperation {}
impl ::core::fmt::Debug for CommandLineActivationOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommandLineActivationOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CommandLineActivationOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivationOperation;{994b2841-c59e-4f69-bcfd-b61ed4e622eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperation_Vtbl;
    const IID: ::windows_core::GUID = <ICommandLineActivationOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CommandLineActivationOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivationOperation";
}
impl ::core::convert::From<CommandLineActivationOperation> for ::windows_core::IUnknown {
    fn from(value: CommandLineActivationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivationOperation> for ::windows_core::IUnknown {
    fn from(value: &CommandLineActivationOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CommandLineActivationOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CommandLineActivationOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommandLineActivationOperation> for ::windows_core::IInspectable {
    fn from(value: CommandLineActivationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivationOperation> for ::windows_core::IInspectable {
    fn from(value: &CommandLineActivationOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CommandLineActivationOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CommandLineActivationOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CommandLineActivationOperation {}
unsafe impl ::core::marker::Sync for CommandLineActivationOperation {}
#[repr(transparent)]
pub struct ContactCallActivatedEventArgs(::windows_core::IUnknown);
impl ContactCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactCallActivatedEventArgs {}
impl ::core::fmt::Debug for ContactCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContactCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs;{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IContactCallActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs";
}
impl ::core::convert::From<ContactCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IContactCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IContactCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactCallActivatedEventArgs> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactCallActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactCallActivatedEventArgs> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactCallActivatedEventArgs> {
        ::core::convert::TryInto::<IContactCallActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactCallActivatedEventArgs {}
#[repr(transparent)]
pub struct ContactMapActivatedEventArgs(::windows_core::IUnknown);
impl ContactMapActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> ::windows_core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::ContactAddress>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactMapActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactMapActivatedEventArgs {}
impl ::core::fmt::Debug for ContactMapActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactMapActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContactMapActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs;{b32bf870-eee7-4ad2-aaf1-a87effcf00a4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IContactMapActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs";
}
impl ::core::convert::From<ContactMapActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactMapActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IContactMapActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IContactMapActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactMapActivatedEventArgs> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactMapActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactMapActivatedEventArgs> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactMapActivatedEventArgs> {
        ::core::convert::TryInto::<IContactMapActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactMapActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMapActivatedEventArgs {}
#[repr(transparent)]
pub struct ContactMessageActivatedEventArgs(::windows_core::IUnknown);
impl ContactMessageActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactMessageActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactMessageActivatedEventArgs {}
impl ::core::fmt::Debug for ContactMessageActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactMessageActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs;{de598db2-0e03-43b0-bf56-bcc40b3162df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IContactMessageActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs";
}
impl ::core::convert::From<ContactMessageActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactMessageActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IContactMessageActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IContactMessageActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactMessageActivatedEventArgs> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactMessageActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactMessageActivatedEventArgs> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactMessageActivatedEventArgs> {
        ::core::convert::TryInto::<IContactMessageActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactMessageActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMessageActivatedEventArgs {}
#[repr(transparent)]
pub struct ContactPanelActivatedEventArgs(::windows_core::IUnknown);
impl ContactPanelActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> ::windows_core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContactPanel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::ContactPanel>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPanelActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPanelActivatedEventArgs {}
impl ::core::fmt::Debug for ContactPanelActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPanelActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs;{52bb63e4-d3d4-4b63-8051-4af2082cab80})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IContactPanelActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs";
}
impl ::core::convert::From<ContactPanelActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactPanelActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IContactPanelActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IContactPanelActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactPanelActivatedEventArgs> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactPanelActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactPanelActivatedEventArgs> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactPanelActivatedEventArgs> {
        ::core::convert::TryInto::<IContactPanelActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactPanelActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPanelActivatedEventArgs {}
#[repr(transparent)]
pub struct ContactPickerActivatedEventArgs(::windows_core::IUnknown);
impl ContactPickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> ::windows_core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContactPickerUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Provider::ContactPickerUI>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPickerActivatedEventArgs {}
impl ::core::fmt::Debug for ContactPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPickerActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs;{ce57aae7-6449-45a7-971f-d113be7a8936})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IContactPickerActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs";
}
impl ::core::convert::From<ContactPickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactPickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPickerActivatedEventArgs> for IContactPickerActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPickerActivatedEventArgs> for IContactPickerActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactPickerActivatedEventArgs> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactPickerActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactPickerActivatedEventArgs> for &ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactPickerActivatedEventArgs> {
        ::core::convert::TryInto::<IContactPickerActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPickerActivatedEventArgs {}
#[repr(transparent)]
pub struct ContactPostActivatedEventArgs(::windows_core::IUnknown);
impl ContactPostActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPostActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPostActivatedEventArgs {}
impl ::core::fmt::Debug for ContactPostActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPostActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContactPostActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs;{b35a3c67-f1e7-4655-ad6e-4857588f552f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IContactPostActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs";
}
impl ::core::convert::From<ContactPostActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactPostActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IContactPostActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IContactPostActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactPostActivatedEventArgs> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactPostActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactPostActivatedEventArgs> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactPostActivatedEventArgs> {
        ::core::convert::TryInto::<IContactPostActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactPostActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPostActivatedEventArgs {}
#[repr(transparent)]
pub struct ContactVideoCallActivatedEventArgs(::windows_core::IUnknown);
impl ContactVideoCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactVideoCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactVideoCallActivatedEventArgs {}
impl ::core::fmt::Debug for ContactVideoCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactVideoCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs;{61079db8-e3e7-4b4f-858d-5c63a96ef684})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IContactVideoCallActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs";
}
impl ::core::convert::From<ContactVideoCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactVideoCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactVideoCallActivatedEventArgs> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactVideoCallActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactVideoCallActivatedEventArgs> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactVideoCallActivatedEventArgs> {
        ::core::convert::TryInto::<IContactVideoCallActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactVideoCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactVideoCallActivatedEventArgs {}
#[repr(transparent)]
pub struct DeviceActivatedEventArgs(::windows_core::IUnknown);
impl DeviceActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn DeviceInformationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for DeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceActivatedEventArgs {}
impl ::core::fmt::Debug for DeviceActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DeviceActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DeviceActivatedEventArgs;{cd50b9a9-ce10-44d2-8234-c355a073ef33})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDeviceActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DeviceActivatedEventArgs";
}
impl ::core::convert::From<DeviceActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IDeviceActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IDeviceActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDeviceActivatedEventArgs> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IDeviceActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDeviceActivatedEventArgs> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IDeviceActivatedEventArgs> {
        ::core::convert::TryInto::<IDeviceActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DeviceActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceActivatedEventArgs {}
#[repr(transparent)]
pub struct DevicePairingActivatedEventArgs(::windows_core::IUnknown);
impl DevicePairingActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
}
impl ::core::clone::Clone for DevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DevicePairingActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePairingActivatedEventArgs {}
impl ::core::fmt::Debug for DevicePairingActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs;{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDevicePairingActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs";
}
impl ::core::convert::From<DevicePairingActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DevicePairingActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IDevicePairingActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IDevicePairingActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDevicePairingActivatedEventArgs> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IDevicePairingActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDevicePairingActivatedEventArgs> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IDevicePairingActivatedEventArgs> {
        ::core::convert::TryInto::<IDevicePairingActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DevicePairingActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePairingActivatedEventArgs {}
#[repr(transparent)]
pub struct DialReceiverActivatedEventArgs(::windows_core::IUnknown);
impl DialReceiverActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn AppName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for DialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DialReceiverActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialReceiverActivatedEventArgs {}
impl ::core::fmt::Debug for DialReceiverActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialReceiverActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs;{fb777ed7-85ee-456e-a44d-85d730e70aed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDialReceiverActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs";
}
impl ::core::convert::From<DialReceiverActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DialReceiverActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDialReceiverActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IDialReceiverActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDialReceiverActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IDialReceiverActivatedEventArgs> {
        ::core::convert::TryInto::<IDialReceiverActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DialReceiverActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DialReceiverActivatedEventArgs {}
#[repr(transparent)]
pub struct FileActivatedEventArgs(::windows_core::IUnknown);
impl FileActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IFileActivatedEventArgsWithCallerPackageFamilyName>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows_core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows_core::Interface::cast::<IFileActivatedEventArgsWithNeighboringFiles>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringFilesQuery)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for FileActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileActivatedEventArgs {}
impl ::core::fmt::Debug for FileActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FileActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileActivatedEventArgs;{bb2afc33-93b1-42ed-8b26-236dd9c78496})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IFileActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileActivatedEventArgs";
}
impl ::core::convert::From<FileActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: FileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &FileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: FileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &FileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileActivatedEventArgs> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileActivatedEventArgs> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileActivatedEventArgs> {
        ::core::convert::TryInto::<IFileActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Error = ::windows_core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> {
        ::core::convert::TryInto::<IFileActivatedEventArgsWithCallerPackageFamilyName>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows_core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileActivatedEventArgsWithNeighboringFiles> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileActivatedEventArgsWithNeighboringFiles> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileActivatedEventArgsWithNeighboringFiles> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileActivatedEventArgsWithNeighboringFiles> {
        ::core::convert::TryInto::<IFileActivatedEventArgsWithNeighboringFiles>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileActivatedEventArgs {}
#[repr(transparent)]
pub struct FileOpenPickerActivatedEventArgs(::windows_core::IUnknown);
impl FileOpenPickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> ::windows_core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FileOpenPickerUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IFileOpenPickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for FileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileOpenPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileOpenPickerActivatedEventArgs {}
impl ::core::fmt::Debug for FileOpenPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOpenPickerActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs;{72827082-5525-4bf2-bc09-1f5095d4964d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IFileOpenPickerActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs";
}
impl ::core::convert::From<FileOpenPickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: FileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileOpenPickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: FileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileOpenPickerActivatedEventArgs> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileOpenPickerActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileOpenPickerActivatedEventArgs> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileOpenPickerActivatedEventArgs> {
        ::core::convert::TryInto::<IFileOpenPickerActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows_core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileOpenPickerActivatedEventArgs2> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileOpenPickerActivatedEventArgs2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileOpenPickerActivatedEventArgs2> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileOpenPickerActivatedEventArgs2> {
        ::core::convert::TryInto::<IFileOpenPickerActivatedEventArgs2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileOpenPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileOpenPickerActivatedEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FileOpenPickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl FileOpenPickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for FileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for FileOpenPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for FileOpenPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOpenPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for FileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs;{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for FileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IFileOpenPickerContinuationEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for FileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FileOpenPickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: FileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FileOpenPickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: FileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IFileOpenPickerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IFileOpenPickerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IFileOpenPickerContinuationEventArgs> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileOpenPickerContinuationEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IFileOpenPickerContinuationEventArgs> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileOpenPickerContinuationEventArgs> {
        ::core::convert::TryInto::<IFileOpenPickerContinuationEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for FileOpenPickerContinuationEventArgs {}
#[repr(transparent)]
pub struct FileSavePickerActivatedEventArgs(::windows_core::IUnknown);
impl FileSavePickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> ::windows_core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FileSavePickerUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EnterpriseId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for FileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileSavePickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileSavePickerActivatedEventArgs {}
impl ::core::fmt::Debug for FileSavePickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileSavePickerActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs;{81c19cf1-74e6-4387-82eb-bb8fd64b4346})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IFileSavePickerActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs";
}
impl ::core::convert::From<FileSavePickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: FileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileSavePickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: FileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileSavePickerActivatedEventArgs> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileSavePickerActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileSavePickerActivatedEventArgs> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileSavePickerActivatedEventArgs> {
        ::core::convert::TryInto::<IFileSavePickerActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows_core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileSavePickerActivatedEventArgs2> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileSavePickerActivatedEventArgs2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileSavePickerActivatedEventArgs2> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileSavePickerActivatedEventArgs2> {
        ::core::convert::TryInto::<IFileSavePickerActivatedEventArgs2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileSavePickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileSavePickerActivatedEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FileSavePickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl FileSavePickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn File(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for FileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for FileSavePickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for FileSavePickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileSavePickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for FileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs;{2c846fe1-3bad-4f33-8c8b-e46fae824b4b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for FileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IFileSavePickerContinuationEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for FileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FileSavePickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: FileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FileSavePickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: FileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IFileSavePickerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IFileSavePickerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IFileSavePickerContinuationEventArgs> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileSavePickerContinuationEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IFileSavePickerContinuationEventArgs> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFileSavePickerContinuationEventArgs> {
        ::core::convert::TryInto::<IFileSavePickerContinuationEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FolderPickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl FolderPickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn Folder(&self) -> ::windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for FolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for FolderPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for FolderPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for FolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs;{51882366-9f4b-498f-beb0-42684f6e1c29})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for FolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IFolderPickerContinuationEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for FolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FolderPickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: FolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<FolderPickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: FolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IFolderPickerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IFolderPickerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IFolderPickerContinuationEventArgs> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFolderPickerContinuationEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IFolderPickerContinuationEventArgs> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IFolderPickerContinuationEventArgs> {
        ::core::convert::TryInto::<IFolderPickerContinuationEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for FolderPickerContinuationEventArgs {}
#[repr(transparent)]
pub struct IActivatedEventArgs(::windows_core::IUnknown);
impl IActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivatedEventArgs {}
impl ::core::fmt::Debug for IActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{cf651713-cd08-4fd8-b697-a281b6544e2e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IActivatedEventArgs {
    type Vtable = IActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf651713_cd08_4fd8_b697_a281b6544e2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationKind) -> ::windows_core::HRESULT,
    pub PreviousExecutionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationExecutionState) -> ::windows_core::HRESULT,
    pub SplashScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IActivatedEventArgsWithUser(::windows_core::IUnknown);
impl IActivatedEventArgsWithUser {
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IActivatedEventArgsWithUser> for ::windows_core::IUnknown {
    fn from(value: IActivatedEventArgsWithUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgsWithUser> for ::windows_core::IUnknown {
    fn from(value: &IActivatedEventArgsWithUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IActivatedEventArgsWithUser> for ::windows_core::IInspectable {
    fn from(value: IActivatedEventArgsWithUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgsWithUser> for ::windows_core::IInspectable {
    fn from(value: &IActivatedEventArgsWithUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IActivatedEventArgsWithUser> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IActivatedEventArgsWithUser) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IActivatedEventArgsWithUser> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IActivatedEventArgsWithUser) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IActivatedEventArgsWithUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivatedEventArgsWithUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivatedEventArgsWithUser {}
impl ::core::fmt::Debug for IActivatedEventArgsWithUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivatedEventArgsWithUser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IActivatedEventArgsWithUser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{1cf09b9e-9962-4936-80ff-afc8e8ae5c8c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IActivatedEventArgsWithUser {
    type Vtable = IActivatedEventArgsWithUser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1cf09b9e_9962_4936_80ff_afc8e8ae5c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsWithUser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(transparent)]
pub struct IApplicationViewActivatedEventArgs(::windows_core::IUnknown);
impl IApplicationViewActivatedEventArgs {
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IApplicationViewActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IApplicationViewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApplicationViewActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IApplicationViewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IApplicationViewActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IApplicationViewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IApplicationViewActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IApplicationViewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IApplicationViewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IApplicationViewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IApplicationViewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IApplicationViewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IApplicationViewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IApplicationViewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationViewActivatedEventArgs {}
impl ::core::fmt::Debug for IApplicationViewActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationViewActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IApplicationViewActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{930cef4b-b829-40fc-88f4-8513e8a64738}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IApplicationViewActivatedEventArgs {
    type Vtable = IApplicationViewActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x930cef4b_b829_40fc_88f4_8513e8a64738);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CurrentlyShownApplicationViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAppointmentsProviderActivatedEventArgs(::windows_core::IUnknown);
impl IAppointmentsProviderActivatedEventArgs {
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IAppointmentsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IAppointmentsProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentsProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IAppointmentsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IAppointmentsProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IAppointmentsProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAppointmentsProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IAppointmentsProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAppointmentsProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{3364c405-933c-4e7d-a034-500fb8dcd9f3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAppointmentsProviderActivatedEventArgs {
    type Vtable = IAppointmentsProviderActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3364c405_933c_4e7d_a034_500fb8dcd9f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs(::windows_core::IUnknown);
impl IAppointmentsProviderAddAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddAppointmentOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentsProvider::AddAppointmentOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderAddAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{a2861367-cee5-4e4d-9ed7-41c34ec18b02}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2861367_cee5_4e4d_9ed7_41c34ec18b02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub AddAppointmentOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    AddAppointmentOperation: usize,
}
#[repr(transparent)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows_core::IUnknown);
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAppointmentOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderRemoveAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{751f3ab8-0b8e-451c-9f15-966e699bac25}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x751f3ab8_0b8e_451c_9f15_966e699bac25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub RemoveAppointmentOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    RemoveAppointmentOperation: usize,
}
#[repr(transparent)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows_core::IUnknown);
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceAppointmentOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderReplaceAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{1551b7d4-a981-4067-8a62-0524e4ade121}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1551b7d4_a981_4067_8a62_0524e4ade121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub ReplaceAppointmentOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    ReplaceAppointmentOperation: usize,
}
#[repr(transparent)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows_core::IUnknown);
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InstanceStartDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    pub fn LocalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LocalId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn RoamingId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RoamingId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{3958f065-9841-4ca5-999b-885198b9ef2a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3958f065_9841_4ca5_999b_885198b9ef2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs(::windows_core::IUnknown);
impl IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn TimeToShow(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeToShow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderShowTimeFrameActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{9baeaba6-0e0b-49aa-babc-12b1dc774986}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9baeaba6_0e0b_49aa_babc_12b1dc774986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub TimeToShow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToShow: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
}
#[repr(transparent)]
pub struct IBackgroundActivatedEventArgs(::windows_core::IUnknown);
impl IBackgroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> ::windows_core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TaskInstance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Background::IBackgroundTaskInstance>(result__)
        }
    }
}
impl ::core::convert::From<IBackgroundActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IBackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IBackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IBackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IBackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundActivatedEventArgs {}
impl ::core::fmt::Debug for IBackgroundActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IBackgroundActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ab14bee0-e760-440e-a91c-44796de3a92d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IBackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab14bee0_e760_440e_a91c_44796de3a92d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Background")]
    pub TaskInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    TaskInstance: usize,
}
#[repr(transparent)]
pub struct IBarcodeScannerPreviewActivatedEventArgs(::windows_core::IUnknown);
impl IBarcodeScannerPreviewActivatedEventArgs {
    pub fn ConnectionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IBarcodeScannerPreviewActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBarcodeScannerPreviewActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBarcodeScannerPreviewActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBarcodeScannerPreviewActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IBarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IBarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBarcodeScannerPreviewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBarcodeScannerPreviewActivatedEventArgs {}
impl ::core::fmt::Debug for IBarcodeScannerPreviewActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBarcodeScannerPreviewActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{6772797c-99bf-4349-af22-e4123560371c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IBarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6772797c_99bf_4349_af22_e4123560371c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerPreviewActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ConnectionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICachedFileUpdaterActivatedEventArgs(::windows_core::IUnknown);
impl ICachedFileUpdaterActivatedEventArgs {
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> ::windows_core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CachedFileUpdaterUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ICachedFileUpdaterActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICachedFileUpdaterActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ICachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICachedFileUpdaterActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICachedFileUpdaterActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ICachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ICachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ICachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for ICachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICachedFileUpdaterActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICachedFileUpdaterActivatedEventArgs {}
impl ::core::fmt::Debug for ICachedFileUpdaterActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICachedFileUpdaterActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d06eb1c7-3805-4ecb-b757-6cf15e26fef3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd06eb1c7_3805_4ecb_b757_6cf15e26fef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Provider")]
    pub CachedFileUpdaterUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    CachedFileUpdaterUI: usize,
}
#[repr(transparent)]
pub struct ICameraSettingsActivatedEventArgs(::windows_core::IUnknown);
impl ICameraSettingsActivatedEventArgs {
    pub fn VideoDeviceController(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn VideoDeviceExtension(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceExtension)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ICameraSettingsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICameraSettingsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ICameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICameraSettingsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICameraSettingsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ICameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ICameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ICameraSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ICameraSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for ICameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICameraSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICameraSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for ICameraSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICameraSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{fb67a508-2dad-490a-9170-dca036eb114b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb67a508_2dad_490a_9170_dca036eb114b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraSettingsActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub VideoDeviceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub VideoDeviceExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ICommandLineActivatedEventArgs(::windows_core::IUnknown);
impl ICommandLineActivatedEventArgs {
    pub fn Operation(&self) -> ::windows_core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CommandLineActivationOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ICommandLineActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommandLineActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ICommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICommandLineActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommandLineActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ICommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ICommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ICommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ICommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for ICommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommandLineActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommandLineActivatedEventArgs {}
impl ::core::fmt::Debug for ICommandLineActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommandLineActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICommandLineActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{4506472c-006a-48eb-8afb-d07ab25e3366}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4506472c_006a_48eb_8afb_d07ab25e3366);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommandLineActivationOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x994b2841_c59e_4f69_bcfd_b61ed4e622eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivationOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CurrentDirectoryPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(transparent)]
pub struct IContactActivatedEventArgs(::windows_core::IUnknown);
impl IContactActivatedEventArgs {
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IContactActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IContactActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IContactActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContactActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContactActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IContactActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IContactActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IContactActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IContactActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IContactActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IContactActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IContactActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactActivatedEventArgs {}
impl ::core::fmt::Debug for IContactActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IContactActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d627a1c4-c025-4c41-9def-f1eafad075e7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IContactActivatedEventArgs {
    type Vtable = IContactActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd627a1c4_c025_4c41_9def_f1eafad075e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IContactCallActivatedEventArgs(::windows_core::IUnknown);
impl IContactCallActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for &IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactCallActivatedEventArgs {}
impl ::core::fmt::Debug for IContactCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IContactCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2df14c7_30eb_41c6_b3bc_5b1694f9dab3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCallActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[repr(transparent)]
pub struct IContactMapActivatedEventArgs(::windows_core::IUnknown);
impl IContactMapActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> ::windows_core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::ContactAddress>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactMapActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactMapActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactMapActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactMapActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for &IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactMapActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactMapActivatedEventArgs {}
impl ::core::fmt::Debug for IContactMapActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactMapActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IContactMapActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{b32bf870-eee7-4ad2-aaf1-a87effcf00a4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb32bf870_eee7_4ad2_aaf1_a87effcf00a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMapActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Address: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[repr(transparent)]
pub struct IContactMessageActivatedEventArgs(::windows_core::IUnknown);
impl IContactMessageActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactMessageActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactMessageActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactMessageActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactMessageActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for &IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactMessageActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactMessageActivatedEventArgs {}
impl ::core::fmt::Debug for IContactMessageActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactMessageActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{de598db2-0e03-43b0-bf56-bcc40b3162df}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde598db2_0e03_43b0_bf56_bcc40b3162df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMessageActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[repr(transparent)]
pub struct IContactPanelActivatedEventArgs(::windows_core::IUnknown);
impl IContactPanelActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> ::windows_core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContactPanel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::ContactPanel>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
impl ::core::convert::From<IContactPanelActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPanelActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactPanelActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPanelActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactPanelActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPanelActivatedEventArgs {}
impl ::core::fmt::Debug for IContactPanelActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactPanelActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{52bb63e4-d3d4-4b63-8051-4af2082cab80}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52bb63e4_d3d4_4b63_8051_4af2082cab80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub ContactPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    ContactPanel: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[repr(transparent)]
pub struct IContactPickerActivatedEventArgs(::windows_core::IUnknown);
impl IContactPickerActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> ::windows_core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContactPickerUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Provider::ContactPickerUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IContactPickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactPickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPickerActivatedEventArgs {}
impl ::core::fmt::Debug for IContactPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactPickerActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ce57aae7-6449-45a7-971f-d113be7a8936}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce57aae7_6449_45a7_971f_d113be7a8936);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub ContactPickerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts_Provider"))]
    ContactPickerUI: usize,
}
#[repr(transparent)]
pub struct IContactPostActivatedEventArgs(::windows_core::IUnknown);
impl IContactPostActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactPostActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPostActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactPostActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactPostActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for &IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactPostActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPostActivatedEventArgs {}
impl ::core::fmt::Debug for IContactPostActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactPostActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IContactPostActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{b35a3c67-f1e7-4655-ad6e-4857588f552f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb35a3c67_f1e7_4655_ad6e_4857588f552f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPostActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[repr(transparent)]
pub struct IContactVideoCallActivatedEventArgs(::windows_core::IUnknown);
impl IContactVideoCallActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IContactVideoCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactVideoCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactVideoCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactVideoCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContactActivatedEventArgs> for &IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactVideoCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactVideoCallActivatedEventArgs {}
impl ::core::fmt::Debug for IContactVideoCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactVideoCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{61079db8-e3e7-4b4f-858d-5c63a96ef684}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61079db8_e3e7_4b4f_858d_5c63a96ef684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactVideoCallActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[repr(transparent)]
pub struct IContactsProviderActivatedEventArgs(::windows_core::IUnknown);
impl IContactsProviderActivatedEventArgs {
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IContactsProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IContactsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactsProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IContactsProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContactsProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IContactsProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContactsProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IContactsProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContactsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContactsProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContactsProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IContactsProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContactsProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactsProviderActivatedEventArgs {}
impl ::core::fmt::Debug for IContactsProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactsProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IContactsProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{4580dca8-5750-4916-aa52-c0829521eb94}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IContactsProviderActivatedEventArgs {
    type Vtable = IContactsProviderActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4580dca8_5750_4916_aa52_c0829521eb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactsProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IContinuationActivatedEventArgs(::windows_core::IUnknown);
impl IContinuationActivatedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IContinuationActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IContinuationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContinuationActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IContinuationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IContinuationActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IContinuationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContinuationActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IContinuationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IContinuationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IContinuationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContinuationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IContinuationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IContinuationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContinuationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContinuationActivatedEventArgs {}
impl ::core::fmt::Debug for IContinuationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContinuationActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IContinuationActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e58106b5-155f-4a94-a742-c7e08f4e188c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IContinuationActivatedEventArgs {
    type Vtable = IContinuationActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe58106b5_155f_4a94_a742_c7e08f4e188c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuationActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
}
#[repr(transparent)]
pub struct IDeviceActivatedEventArgs(::windows_core::IUnknown);
impl IDeviceActivatedEventArgs {
    pub fn DeviceInformationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IDeviceActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IDeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDeviceActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IDeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IDeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IDeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IDeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IDeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDeviceActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceActivatedEventArgs {}
impl ::core::fmt::Debug for IDeviceActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IDeviceActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{cd50b9a9-ce10-44d2-8234-c355a073ef33}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IDeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd50b9a9_ce10_44d2_8234_c355a073ef33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceInformationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDevicePairingActivatedEventArgs(::windows_core::IUnknown);
impl IDevicePairingActivatedEventArgs {
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IDevicePairingActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDevicePairingActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IDevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDevicePairingActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDevicePairingActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IDevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IDevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IDevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IDevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IDevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDevicePairingActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDevicePairingActivatedEventArgs {}
impl ::core::fmt::Debug for IDevicePairingActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDevicePairingActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IDevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IDevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba0d1e4_ecc6_4148_94ed_f4b37ec05b3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
#[repr(transparent)]
pub struct IDialReceiverActivatedEventArgs(::windows_core::IUnknown);
impl IDialReceiverActivatedEventArgs {
    pub fn AppName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IDialReceiverActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDialReceiverActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IDialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDialReceiverActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDialReceiverActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IDialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IDialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IDialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs> for &IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IDialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDialReceiverActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDialReceiverActivatedEventArgs {}
impl ::core::fmt::Debug for IDialReceiverActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDialReceiverActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IDialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{fb777ed7-85ee-456e-a44d-85d730e70aed}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IDialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb777ed7_85ee_456e_a44d_85d730e70aed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AppName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFileActivatedEventArgs(::windows_core::IUnknown);
impl IFileActivatedEventArgs {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IFileActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IFileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IFileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IFileActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileActivatedEventArgs {}
impl ::core::fmt::Debug for IFileActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IFileActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{bb2afc33-93b1-42ed-8b26-236dd9c78496}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IFileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb2afc33_93b1_42ed_8b26_236dd9c78496);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    Files: usize,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName(::windows_core::IUnknown);
impl IFileActivatedEventArgsWithCallerPackageFamilyName {
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows_core::IUnknown {
    fn from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows_core::IUnknown {
    fn from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows_core::IInspectable {
    fn from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows_core::IInspectable {
    fn from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithCallerPackageFamilyName> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithCallerPackageFamilyName> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileActivatedEventArgsWithCallerPackageFamilyName {}
impl ::core::fmt::Debug for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileActivatedEventArgsWithCallerPackageFamilyName").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{2d60f06b-d25f-4d25-8653-e1c5e1108309}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Vtable = IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d60f06b_d25f_4d25_8653_e1c5e1108309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFileActivatedEventArgsWithNeighboringFiles(::windows_core::IUnknown);
impl IFileActivatedEventArgsWithNeighboringFiles {
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows_core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringFilesQuery)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = &::windows_core::Interface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithNeighboringFiles> for ::windows_core::IUnknown {
    fn from(value: IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithNeighboringFiles> for ::windows_core::IUnknown {
    fn from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithNeighboringFiles> for ::windows_core::IInspectable {
    fn from(value: IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithNeighboringFiles> for ::windows_core::IInspectable {
    fn from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithNeighboringFiles> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IFileActivatedEventArgsWithNeighboringFiles) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithNeighboringFiles> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithNeighboringFiles> for IFileActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IFileActivatedEventArgsWithNeighboringFiles) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithNeighboringFiles> for IFileActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows_core::Param<'a, IFileActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IFileActivatedEventArgs> for &IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows_core::Param<'a, IFileActivatedEventArgs> {
        ::core::convert::TryInto::<IFileActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IFileActivatedEventArgsWithNeighboringFiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileActivatedEventArgsWithNeighboringFiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileActivatedEventArgsWithNeighboringFiles {}
impl ::core::fmt::Debug for IFileActivatedEventArgsWithNeighboringFiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileActivatedEventArgsWithNeighboringFiles").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IFileActivatedEventArgsWithNeighboringFiles {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{433ba1a4-e1e2-48fd-b7fc-b5d6eee65033}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IFileActivatedEventArgsWithNeighboringFiles {
    type Vtable = IFileActivatedEventArgsWithNeighboringFiles_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x433ba1a4_e1e2_48fd_b7fc_b5d6eee65033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithNeighboringFiles_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Search")]
    pub NeighboringFilesQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    NeighboringFilesQuery: usize,
}
#[repr(transparent)]
pub struct IFileOpenPickerActivatedEventArgs(::windows_core::IUnknown);
impl IFileOpenPickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> ::windows_core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FileOpenPickerUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IFileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IFileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IFileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IFileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IFileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileOpenPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOpenPickerActivatedEventArgs {}
impl ::core::fmt::Debug for IFileOpenPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOpenPickerActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IFileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{72827082-5525-4bf2-bc09-1f5095d4964d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IFileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72827082_5525_4bf2_bc09_1f5095d4964d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileOpenPickerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileOpenPickerUI: usize,
}
#[repr(transparent)]
pub struct IFileOpenPickerActivatedEventArgs2(::windows_core::IUnknown);
impl IFileOpenPickerActivatedEventArgs2 {
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs2> for ::windows_core::IUnknown {
    fn from(value: IFileOpenPickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs2> for ::windows_core::IUnknown {
    fn from(value: &IFileOpenPickerActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs2> for ::windows_core::IInspectable {
    fn from(value: IFileOpenPickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs2> for ::windows_core::IInspectable {
    fn from(value: &IFileOpenPickerActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFileOpenPickerActivatedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileOpenPickerActivatedEventArgs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOpenPickerActivatedEventArgs2 {}
impl ::core::fmt::Debug for IFileOpenPickerActivatedEventArgs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOpenPickerActivatedEventArgs2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IFileOpenPickerActivatedEventArgs2 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{5e731f66-8d1f-45fb-af1d-73205c8fc7a1}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IFileOpenPickerActivatedEventArgs2 {
    type Vtable = IFileOpenPickerActivatedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e731f66_8d1f_45fb_af1d_73205c8fc7a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IFileOpenPickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl IFileOpenPickerContinuationEventArgs {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IFileOpenPickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: IFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IFileOpenPickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IFileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IFileOpenPickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: IFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IFileOpenPickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IFileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<IFileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&IFileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<IFileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&IFileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for &IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IFileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IFileOpenPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IFileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IFileOpenPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOpenPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for IFileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IFileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0fa3f3a_d4e8_4ad3_9c34_2308f32fcec9);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerContinuationEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated")))]
    Files: usize,
}
#[repr(transparent)]
pub struct IFileSavePickerActivatedEventArgs(::windows_core::IUnknown);
impl IFileSavePickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> ::windows_core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FileSavePickerUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IFileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IFileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IFileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IFileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IFileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IFileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileSavePickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSavePickerActivatedEventArgs {}
impl ::core::fmt::Debug for IFileSavePickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSavePickerActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IFileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{81c19cf1-74e6-4387-82eb-bb8fd64b4346}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IFileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81c19cf1_74e6_4387_82eb_bb8fd64b4346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileSavePickerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileSavePickerUI: usize,
}
#[repr(transparent)]
pub struct IFileSavePickerActivatedEventArgs2(::windows_core::IUnknown);
impl IFileSavePickerActivatedEventArgs2 {
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EnterpriseId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs2> for ::windows_core::IUnknown {
    fn from(value: IFileSavePickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs2> for ::windows_core::IUnknown {
    fn from(value: &IFileSavePickerActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs2> for ::windows_core::IInspectable {
    fn from(value: IFileSavePickerActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs2> for ::windows_core::IInspectable {
    fn from(value: &IFileSavePickerActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFileSavePickerActivatedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFileSavePickerActivatedEventArgs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSavePickerActivatedEventArgs2 {}
impl ::core::fmt::Debug for IFileSavePickerActivatedEventArgs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSavePickerActivatedEventArgs2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IFileSavePickerActivatedEventArgs2 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{6b73fe13-2cf2-4d48-8cbc-af67d23f1ce7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IFileSavePickerActivatedEventArgs2 {
    type Vtable = IFileSavePickerActivatedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b73fe13_2cf2_4d48_8cbc_af67d23f1ce7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IFileSavePickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl IFileSavePickerContinuationEventArgs {
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn File(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IFileSavePickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: IFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IFileSavePickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IFileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IFileSavePickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: IFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IFileSavePickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IFileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<IFileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&IFileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<IFileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&IFileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for &IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IFileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IFileSavePickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IFileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IFileSavePickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSavePickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for IFileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{2c846fe1-3bad-4f33-8c8b-e46fae824b4b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IFileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c846fe1_3bad_4f33_8c8b_e46fae824b4b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerContinuationEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    File: usize,
}
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IFolderPickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl IFolderPickerContinuationEventArgs {
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn Folder(&self) -> ::windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IFolderPickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: IFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IFolderPickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IFolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<IFolderPickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: IFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&IFolderPickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IFolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<IFolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&IFolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<IFolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::TryFrom<&IFolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for &IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IFolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IFolderPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IFolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IFolderPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::RuntimeType for IFolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{51882366-9f4b-498f-beb0-42684f6e1c29}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IFolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51882366_9f4b_498f_beb0_42684f6e1c29);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPickerContinuationEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    Folder: usize,
}
#[repr(transparent)]
pub struct ILaunchActivatedEventArgs(::windows_core::IUnknown);
impl ILaunchActivatedEventArgs {
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ILaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ILaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for ILaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchActivatedEventArgs {}
impl ::core::fmt::Debug for ILaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILaunchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ILaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{fbc93e26-a14a-4b4f-82b0-33bed920af52}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ILaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbc93e26_a14a_4b4f_82b0_33bed920af52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ILaunchActivatedEventArgs2(::windows_core::IUnknown);
impl ILaunchActivatedEventArgs2 {
    pub fn TileActivatedInfo(&self) -> ::windows_core::Result<TileActivatedInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TileActivatedInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TileActivatedInfo>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs2> for ::windows_core::IUnknown {
    fn from(value: ILaunchActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs2> for ::windows_core::IUnknown {
    fn from(value: &ILaunchActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs2> for ::windows_core::IInspectable {
    fn from(value: ILaunchActivatedEventArgs2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs2> for ::windows_core::IInspectable {
    fn from(value: &ILaunchActivatedEventArgs2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs2> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ILaunchActivatedEventArgs2) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs2> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs2) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs2> for ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ILaunchActivatedEventArgs2) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs2> for ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs2) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs> for &ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for ILaunchActivatedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILaunchActivatedEventArgs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchActivatedEventArgs2 {}
impl ::core::fmt::Debug for ILaunchActivatedEventArgs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILaunchActivatedEventArgs2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ILaunchActivatedEventArgs2 {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{0fd37ebc-9dc9-46b5-9ace-bd95d4565345}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ILaunchActivatedEventArgs2 {
    type Vtable = ILaunchActivatedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0fd37ebc_9dc9_46b5_9ace_bd95d4565345);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TileActivatedInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ILockScreenActivatedEventArgs(::windows_core::IUnknown);
impl ILockScreenActivatedEventArgs {
    pub fn Info(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Info)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ILockScreenActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockScreenActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ILockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILockScreenActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockScreenActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ILockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ILockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ILockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ILockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for ILockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILockScreenActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILockScreenActivatedEventArgs {}
impl ::core::fmt::Debug for ILockScreenActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILockScreenActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ILockScreenActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{3ca77966-6108-4a41-8220-ee7d133c8532}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ILockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ca77966_6108_4a41_8220_ee7d133c8532);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ILockScreenCallActivatedEventArgs(::windows_core::IUnknown);
impl ILockScreenCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> ::windows_core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CallUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Calls::LockScreenCallUI>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<ILockScreenCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockScreenCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ILockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILockScreenCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILockScreenCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ILockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ILockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ILockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs> for &ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for ILockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILockScreenCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILockScreenCallActivatedEventArgs {}
impl ::core::fmt::Debug for ILockScreenCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILockScreenCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ILockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{06f37fbe-b5f2-448b-b13e-e328ac1c516a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ILockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06f37fbe_b5f2_448b_b13e_e328ac1c516a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Calls")]
    pub CallUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls"))]
    CallUI: usize,
}
#[repr(transparent)]
pub struct IPhoneCallActivatedEventArgs(::windows_core::IUnknown);
impl IPhoneCallActivatedEventArgs {
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPhoneCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhoneCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IPhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPhoneCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPhoneCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IPhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IPhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IPhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhoneCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhoneCallActivatedEventArgs {}
impl ::core::fmt::Debug for IPhoneCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhoneCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{54615221-a3c1-4ced-b62f-8c60523619ad}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54615221_a3c1_4ced_b62f_8c60523619ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPickerReturnedActivatedEventArgs(::windows_core::IUnknown);
impl IPickerReturnedActivatedEventArgs {
    pub fn PickerOperationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PickerOperationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPickerReturnedActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IPickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPickerReturnedActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IPickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPickerReturnedActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IPickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPickerReturnedActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IPickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IPickerReturnedActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPickerReturnedActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IPickerReturnedActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPickerReturnedActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPickerReturnedActivatedEventArgs {}
impl ::core::fmt::Debug for IPickerReturnedActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPickerReturnedActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{360defb9-a9d3-4984-a4ed-9ec734604921}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x360defb9_a9d3_4984_a4ed_9ec734604921);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerReturnedActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PickerOperationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPrelaunchActivatedEventArgs(::windows_core::IUnknown);
impl IPrelaunchActivatedEventArgs {
    pub fn PrelaunchActivated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PrelaunchActivated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPrelaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IPrelaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrelaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IPrelaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPrelaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IPrelaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrelaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IPrelaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPrelaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IPrelaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrelaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPrelaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IPrelaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrelaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrelaunchActivatedEventArgs {}
impl ::core::fmt::Debug for IPrelaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrelaunchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPrelaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{0c44717b-19f7-48d6-b046-cf22826eaa74}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPrelaunchActivatedEventArgs {
    type Vtable = IPrelaunchActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c44717b_19f7_48d6_b046_cf22826eaa74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrelaunchActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PrelaunchActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IPrint3DWorkflowActivatedEventArgs(::windows_core::IUnknown);
impl IPrint3DWorkflowActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> ::windows_core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Workflow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPrint3DWorkflowActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrint3DWorkflowActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IPrint3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPrint3DWorkflowActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrint3DWorkflowActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IPrint3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IPrint3DWorkflowActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPrint3DWorkflowActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IPrint3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrint3DWorkflowActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrint3DWorkflowActivatedEventArgs {}
impl ::core::fmt::Debug for IPrint3DWorkflowActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrint3DWorkflowActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{3f57e78b-f2ac-4619-8302-ef855e1c9b90}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPrint3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f57e78b_f2ac_4619_8302_ef855e1c9b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Workflow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Workflow: usize,
}
#[repr(transparent)]
pub struct IPrintTaskSettingsActivatedEventArgs(::windows_core::IUnknown);
impl IPrintTaskSettingsActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> ::windows_core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IPrintTaskSettingsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintTaskSettingsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IPrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPrintTaskSettingsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintTaskSettingsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IPrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IPrintTaskSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IPrintTaskSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IPrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintTaskSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTaskSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for IPrintTaskSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTaskSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ee30a0c9-ce56-4865-ba8e-8954ac271107}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IPrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee30a0c9_ce56_4865_ba8e_8954ac271107);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSettingsActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Configuration: usize,
}
#[repr(transparent)]
pub struct IProtocolActivatedEventArgs(::windows_core::IUnknown);
impl IProtocolActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtocolActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolActivatedEventArgs {}
impl ::core::fmt::Debug for IProtocolActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtocolActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IProtocolActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{6095f4dd-b7c0-46ab-81fe-d90f36d00d24}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6095f4dd_b7c0_46ab_81fe_d90f36d00d24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[repr(transparent)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData(::windows_core::IUnknown);
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows_core::IUnknown {
    fn from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows_core::IUnknown {
    fn from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows_core::IInspectable {
    fn from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows_core::IInspectable {
    fn from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {}
impl ::core::fmt::Debug for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d84a0c12-5c8f-438c-83cb-c28fcc0b2fdb}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Vtable = IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd84a0c12_5c8f_438c_83cb_c28fcc0b2fdb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Data: usize,
}
#[repr(transparent)]
pub struct IProtocolForResultsActivatedEventArgs(::windows_core::IUnknown);
impl IProtocolForResultsActivatedEventArgs {
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> ::windows_core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProtocolForResultsOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::ProtocolForResultsOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IProtocolForResultsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolForResultsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IProtocolForResultsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProtocolForResultsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtocolForResultsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolForResultsActivatedEventArgs {}
impl ::core::fmt::Debug for IProtocolForResultsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtocolForResultsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe75132c2_7ae7_4517_80ac_dbe8d7cc5b9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub ProtocolForResultsOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ProtocolForResultsOperation: usize,
}
#[repr(transparent)]
pub struct IRestrictedLaunchActivatedEventArgs(::windows_core::IUnknown);
impl IRestrictedLaunchActivatedEventArgs {
    pub fn SharedContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).SharedContext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IRestrictedLaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRestrictedLaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IRestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRestrictedLaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRestrictedLaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IRestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IRestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IRestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IRestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IRestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRestrictedLaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRestrictedLaunchActivatedEventArgs {}
impl ::core::fmt::Debug for IRestrictedLaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRestrictedLaunchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IRestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e0b7ac81-bfc3-4344-a5da-19fd5a27baae}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IRestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0b7ac81_bfc3_4344_a5da_19fd5a27baae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedLaunchActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SharedContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISearchActivatedEventArgs(::windows_core::IUnknown);
impl ISearchActivatedEventArgs {
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<ISearchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ISearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISearchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ISearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ISearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ISearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ISearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ISearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for ISearchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISearchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchActivatedEventArgs {}
impl ::core::fmt::Debug for ISearchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISearchActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{8cb36951-58c8-43e3-94bc-41d33f8b630e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cb36951_58c8_43e3_94bc_41d33f8b630e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails(::windows_core::IUnknown);
impl ISearchActivatedEventArgsWithLinguisticDetails {
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Search::SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
impl ::core::convert::From<ISearchActivatedEventArgsWithLinguisticDetails> for ::windows_core::IUnknown {
    fn from(value: ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgsWithLinguisticDetails> for ::windows_core::IUnknown {
    fn from(value: &ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISearchActivatedEventArgsWithLinguisticDetails> for ::windows_core::IInspectable {
    fn from(value: ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgsWithLinguisticDetails> for ::windows_core::IInspectable {
    fn from(value: &ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISearchActivatedEventArgsWithLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISearchActivatedEventArgsWithLinguisticDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchActivatedEventArgsWithLinguisticDetails {}
impl ::core::fmt::Debug for ISearchActivatedEventArgsWithLinguisticDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchActivatedEventArgsWithLinguisticDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ISearchActivatedEventArgsWithLinguisticDetails {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{c09f33da-08ab-4931-9b7c-451025f21f81}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ISearchActivatedEventArgsWithLinguisticDetails {
    type Vtable = ISearchActivatedEventArgsWithLinguisticDetails_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc09f33da_08ab_4931_9b7c_451025f21f81);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Search")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))]
    LinguisticDetails: usize,
}
#[repr(transparent)]
pub struct IShareTargetActivatedEventArgs(::windows_core::IUnknown);
impl IShareTargetActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> ::windows_core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShareOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DataTransfer::ShareTarget::ShareOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IShareTargetActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IShareTargetActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IShareTargetActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IShareTargetActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IShareTargetActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShareTargetActivatedEventArgs {}
impl ::core::fmt::Debug for IShareTargetActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShareTargetActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4bdaf9c8_cdb2_4acb_bfc3_6648563378ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareTargetActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub ShareOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer_ShareTarget"))]
    ShareOperation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplashScreen(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplashScreen {
    type Vtable = ISplashScreen_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca4d975c_d4d6_43f0_97c0_0833c6391c24);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplashScreen_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ImageLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageLocation: usize,
    #[cfg(feature = "Foundation")]
    pub Dismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Dismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDismissed: usize,
}
#[repr(transparent)]
pub struct IStartupTaskActivatedEventArgs(::windows_core::IUnknown);
impl IStartupTaskActivatedEventArgs {
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IStartupTaskActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStartupTaskActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IStartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStartupTaskActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStartupTaskActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IStartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IStartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IStartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IStartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IStartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IStartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStartupTaskActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStartupTaskActivatedEventArgs {}
impl ::core::fmt::Debug for IStartupTaskActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStartupTaskActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IStartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{03b11a58-5276-4d91-8621-54611864d5fa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IStartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03b11a58_5276_4d91_8621_54611864d5fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTaskActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileActivatedInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileActivatedInfo {
    type Vtable = ITileActivatedInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80e4a3b1_3980_4f17_b738_89194e0b8f65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileActivatedInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    pub RecentlyShownNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Notifications")))]
    RecentlyShownNotifications: usize,
}
#[repr(transparent)]
pub struct IToastNotificationActivatedEventArgs(::windows_core::IUnknown);
impl IToastNotificationActivatedEventArgs {
    pub fn Argument(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Argument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UserInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IToastNotificationActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IToastNotificationActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IToastNotificationActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IToastNotificationActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IToastNotificationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IToastNotificationActivatedEventArgs {}
impl ::core::fmt::Debug for IToastNotificationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IToastNotificationActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{92a86f82-5290-431d-be85-c4aaeeb8685f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92a86f82_5290_431d_be85_c4aaeeb8685f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Argument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
#[repr(transparent)]
pub struct IUserDataAccountProviderActivatedEventArgs(::windows_core::IUnknown);
impl IUserDataAccountProviderActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> ::windows_core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IUserDataAccountProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserDataAccountProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IUserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUserDataAccountProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserDataAccountProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IUserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IUserDataAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IUserDataAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IUserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserDataAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserDataAccountProviderActivatedEventArgs {}
impl ::core::fmt::Debug for IUserDataAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserDataAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{1bc9f723-8ef1-4a51-a63a-fe711eeab607}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IUserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bc9f723_8ef1_4a51_a63a_fe711eeab607);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_UserDataAccounts_Provider"))]
    Operation: usize,
}
#[repr(transparent)]
pub struct IViewSwitcherProvider(::windows_core::IUnknown);
impl IViewSwitcherProvider {
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IViewSwitcherProvider> for ::windows_core::IUnknown {
    fn from(value: IViewSwitcherProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IViewSwitcherProvider> for ::windows_core::IUnknown {
    fn from(value: &IViewSwitcherProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IViewSwitcherProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IViewSwitcherProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IViewSwitcherProvider> for ::windows_core::IInspectable {
    fn from(value: IViewSwitcherProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IViewSwitcherProvider> for ::windows_core::IInspectable {
    fn from(value: &IViewSwitcherProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IViewSwitcherProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IViewSwitcherProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IViewSwitcherProvider> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IViewSwitcherProvider) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IViewSwitcherProvider> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IViewSwitcherProvider) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IViewSwitcherProvider {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IViewSwitcherProvider {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IViewSwitcherProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IViewSwitcherProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewSwitcherProvider {}
impl ::core::fmt::Debug for IViewSwitcherProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewSwitcherProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IViewSwitcherProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{33f288a6-5c2c-4d27-bac7-7536088f1219}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IViewSwitcherProvider {
    type Vtable = IViewSwitcherProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33f288a6_5c2c_4d27_bac7_7536088f1219);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewSwitcherProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_ViewManagement")]
    pub ViewSwitcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    ViewSwitcher: usize,
}
#[repr(transparent)]
pub struct IVoiceCommandActivatedEventArgs(::windows_core::IUnknown);
impl IVoiceCommandActivatedEventArgs {
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> ::windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IVoiceCommandActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVoiceCommandActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IVoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVoiceCommandActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVoiceCommandActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IVoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IVoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IVoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IVoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IVoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IVoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVoiceCommandActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVoiceCommandActivatedEventArgs {}
impl ::core::fmt::Debug for IVoiceCommandActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVoiceCommandActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IVoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ab92dcfd-8d43-4de6-9775-20704b581b00}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IVoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab92dcfd_8d43_4de6_9775_20704b581b00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Media_SpeechRecognition")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))]
    Result: usize,
}
#[repr(transparent)]
pub struct IWalletActionActivatedEventArgs(::windows_core::IUnknown);
impl IWalletActionActivatedEventArgs {
    pub fn ItemId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ItemId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub fn ActionKind(&self) -> ::windows_core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Wallet::WalletActionKind>::zeroed();
            (::windows_core::Interface::vtable(this).ActionKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Wallet::WalletActionKind>(result__)
        }
    }
    pub fn ActionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IWalletActionActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWalletActionActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IWalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWalletActionActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWalletActionActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IWalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IWalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IWalletActionActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IWalletActionActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IWalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWalletActionActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWalletActionActivatedEventArgs {}
impl ::core::fmt::Debug for IWalletActionActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWalletActionActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IWalletActionActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IWalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcfc027b_1a1a_4d22_923f_ae6f45fa52d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletActionActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub ActionKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Wallet::WalletActionKind) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Wallet"))]
    ActionKind: usize,
    pub ActionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWebAccountProviderActivatedEventArgs(::windows_core::IUnknown);
impl IWebAccountProviderActivatedEventArgs {
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> ::windows_core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::convert::From<IWebAccountProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IWebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAccountProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IWebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IWebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IWebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IWebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IWebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccountProviderActivatedEventArgs {}
impl ::core::fmt::Debug for IWebAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IWebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{72b71774-98ea-4ccf-9752-46d9051004f1}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IWebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72b71774_98ea_4ccf_9752_46d9051004f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Provider"))]
    Operation: usize,
}
#[repr(transparent)]
pub struct IWebAuthenticationBrokerContinuationEventArgs(::windows_core::IUnknown);
impl IWebAuthenticationBrokerContinuationEventArgs {
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> ::windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WebAuthenticationResult)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::convert::From<IWebAuthenticationBrokerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAuthenticationBrokerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAuthenticationBrokerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAuthenticationBrokerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IWebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<IWebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: IWebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for &IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IWebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebAuthenticationBrokerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAuthenticationBrokerContinuationEventArgs {}
impl ::core::fmt::Debug for IWebAuthenticationBrokerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAuthenticationBrokerContinuationEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{75dda3d4-7714-453d-b7ff-b95e3a1709da}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IWebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75dda3d4_7714_453d_b7ff_b95e3a1709da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerContinuationEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Security_Authentication_Web")]
    pub WebAuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    WebAuthenticationResult: usize,
}
#[repr(transparent)]
pub struct LaunchActivatedEventArgs(::windows_core::IUnknown);
impl LaunchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TileActivatedInfo(&self) -> ::windows_core::Result<TileActivatedInfo> {
        let this = &::windows_core::Interface::cast::<ILaunchActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TileActivatedInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TileActivatedInfo>(result__)
        }
    }
    pub fn PrelaunchActivated(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PrelaunchActivated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for LaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LaunchActivatedEventArgs {}
impl ::core::fmt::Debug for LaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LaunchActivatedEventArgs;{fbc93e26-a14a-4b4f-82b0-33bed920af52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ILaunchActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LaunchActivatedEventArgs";
}
impl ::core::convert::From<LaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: LaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: LaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    type Error = ::windows_core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs2> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs2> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs2> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPrelaunchActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IPrelaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPrelaunchActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IPrelaunchActivatedEventArgs> {
        ::core::convert::TryInto::<IPrelaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LaunchActivatedEventArgs {}
#[repr(transparent)]
pub struct LockScreenActivatedEventArgs(::windows_core::IUnknown);
impl LockScreenActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn Info(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Info)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for LockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenActivatedEventArgs {}
impl ::core::fmt::Debug for LockScreenActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LockScreenActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs;{3ca77966-6108-4a41-8220-ee7d133c8532})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ILockScreenActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs";
}
impl ::core::convert::From<LockScreenActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: LockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: LockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for ILockScreenActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for ILockScreenActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILockScreenActivatedEventArgs> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILockScreenActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILockScreenActivatedEventArgs> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILockScreenActivatedEventArgs> {
        ::core::convert::TryInto::<ILockScreenActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LockScreenActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenActivatedEventArgs {}
#[repr(transparent)]
pub struct LockScreenCallActivatedEventArgs(::windows_core::IUnknown);
impl LockScreenCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> ::windows_core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CallUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Calls::LockScreenCallUI>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for LockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenCallActivatedEventArgs {}
impl ::core::fmt::Debug for LockScreenCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs;{06f37fbe-b5f2-448b-b13e-e328ac1c516a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ILockScreenCallActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs";
}
impl ::core::convert::From<LockScreenCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: LockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: LockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILaunchActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILockScreenCallActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILockScreenCallActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ILockScreenCallActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ILockScreenCallActivatedEventArgs> {
        ::core::convert::TryInto::<ILockScreenCallActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LockScreenCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenCallActivatedEventArgs {}
#[repr(transparent)]
pub struct LockScreenComponentActivatedEventArgs(::windows_core::IUnknown);
impl LockScreenComponentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
}
impl ::core::clone::Clone for LockScreenComponentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LockScreenComponentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenComponentActivatedEventArgs {}
impl ::core::fmt::Debug for LockScreenComponentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenComponentActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LockScreenComponentActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LockScreenComponentActivatedEventArgs {
    type Vtable = IActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs";
}
impl ::core::convert::From<LockScreenComponentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: LockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LockScreenComponentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: LockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LockScreenComponentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LockScreenComponentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenComponentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LockScreenComponentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LockScreenComponentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenComponentActivatedEventArgs {}
#[repr(transparent)]
pub struct PhoneCallActivatedEventArgs(::windows_core::IUnknown);
impl PhoneCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
}
impl ::core::clone::Clone for PhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhoneCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallActivatedEventArgs {}
impl ::core::fmt::Debug for PhoneCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs;{54615221-a3c1-4ced-b62f-8c60523619ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPhoneCallActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs";
}
impl ::core::convert::From<PhoneCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhoneCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IPhoneCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IPhoneCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPhoneCallActivatedEventArgs> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IPhoneCallActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPhoneCallActivatedEventArgs> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IPhoneCallActivatedEventArgs> {
        ::core::convert::TryInto::<IPhoneCallActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PhoneCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PhoneCallActivatedEventArgs {}
#[repr(transparent)]
pub struct PickerReturnedActivatedEventArgs(::windows_core::IUnknown);
impl PickerReturnedActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn PickerOperationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PickerOperationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PickerReturnedActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PickerReturnedActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PickerReturnedActivatedEventArgs {}
impl ::core::fmt::Debug for PickerReturnedActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerReturnedActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs;{360defb9-a9d3-4984-a4ed-9ec734604921})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPickerReturnedActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs";
}
impl ::core::convert::From<PickerReturnedActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PickerReturnedActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: PickerReturnedActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &PickerReturnedActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PickerReturnedActivatedEventArgs> for IPickerReturnedActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: PickerReturnedActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PickerReturnedActivatedEventArgs> for IPickerReturnedActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &PickerReturnedActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPickerReturnedActivatedEventArgs> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IPickerReturnedActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPickerReturnedActivatedEventArgs> for &PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IPickerReturnedActivatedEventArgs> {
        ::core::convert::TryInto::<IPickerReturnedActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PickerReturnedActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PickerReturnedActivatedEventArgs {}
#[repr(transparent)]
pub struct Print3DWorkflowActivatedEventArgs(::windows_core::IUnknown);
impl Print3DWorkflowActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> ::windows_core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Workflow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
}
impl ::core::clone::Clone for Print3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Print3DWorkflowActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DWorkflowActivatedEventArgs {}
impl ::core::fmt::Debug for Print3DWorkflowActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflowActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Print3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs;{3f57e78b-f2ac-4619-8302-ef855e1c9b90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Print3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrint3DWorkflowActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Print3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs";
}
impl ::core::convert::From<Print3DWorkflowActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: Print3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Print3DWorkflowActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: Print3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<Print3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: Print3DWorkflowActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Print3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &Print3DWorkflowActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<Print3DWorkflowActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: Print3DWorkflowActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Print3DWorkflowActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &Print3DWorkflowActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPrint3DWorkflowActivatedEventArgs> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IPrint3DWorkflowActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPrint3DWorkflowActivatedEventArgs> for &Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IPrint3DWorkflowActivatedEventArgs> {
        ::core::convert::TryInto::<IPrint3DWorkflowActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Print3DWorkflowActivatedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowActivatedEventArgs {}
#[repr(transparent)]
pub struct PrintTaskSettingsActivatedEventArgs(::windows_core::IUnknown);
impl PrintTaskSettingsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> ::windows_core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintTaskSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for PrintTaskSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs;{ee30a0c9-ce56-4865-ba8e-8954ac271107})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPrintTaskSettingsActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs";
}
impl ::core::convert::From<PrintTaskSettingsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: PrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTaskSettingsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: PrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: PrintTaskSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &PrintTaskSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PrintTaskSettingsActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: PrintTaskSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskSettingsActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &PrintTaskSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPrintTaskSettingsActivatedEventArgs> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IPrintTaskSettingsActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IPrintTaskSettingsActivatedEventArgs> for &PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IPrintTaskSettingsActivatedEventArgs> {
        ::core::convert::TryInto::<IPrintTaskSettingsActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintTaskSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskSettingsActivatedEventArgs {}
#[repr(transparent)]
pub struct ProtocolActivatedEventArgs(::windows_core::IUnknown);
impl ProtocolActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtocolActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtocolActivatedEventArgs {}
impl ::core::fmt::Debug for ProtocolActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtocolActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtocolActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs;{6095f4dd-b7c0-46ab-81fe-d90f36d00d24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IProtocolActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs";
}
impl ::core::convert::From<ProtocolActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtocolActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IProtocolActivatedEventArgs> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IProtocolActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IProtocolActivatedEventArgs> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IProtocolActivatedEventArgs> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ProtocolActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolActivatedEventArgs {}
#[repr(transparent)]
pub struct ProtocolForResultsActivatedEventArgs(::windows_core::IUnknown);
impl ProtocolForResultsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::Interface::cast::<IProtocolActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> ::windows_core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProtocolForResultsOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::ProtocolForResultsOperation>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for ProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtocolForResultsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtocolForResultsActivatedEventArgs {}
impl ::core::fmt::Debug for ProtocolForResultsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtocolForResultsActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs;{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IProtocolForResultsActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs";
}
impl ::core::convert::From<ProtocolForResultsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProtocolForResultsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IProtocolActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IProtocolActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IProtocolActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IProtocolActivatedEventArgs> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IProtocolForResultsActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IProtocolForResultsActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IProtocolForResultsActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IProtocolForResultsActivatedEventArgs> {
        ::core::convert::TryInto::<IProtocolForResultsActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ProtocolForResultsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolForResultsActivatedEventArgs {}
#[repr(transparent)]
pub struct RestrictedLaunchActivatedEventArgs(::windows_core::IUnknown);
impl RestrictedLaunchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn SharedContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).SharedContext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for RestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RestrictedLaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RestrictedLaunchActivatedEventArgs {}
impl ::core::fmt::Debug for RestrictedLaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RestrictedLaunchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs;{e0b7ac81-bfc3-4344-a5da-19fd5a27baae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRestrictedLaunchActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs";
}
impl ::core::convert::From<RestrictedLaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RestrictedLaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRestrictedLaunchActivatedEventArgs> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IRestrictedLaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IRestrictedLaunchActivatedEventArgs> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IRestrictedLaunchActivatedEventArgs> {
        ::core::convert::TryInto::<IRestrictedLaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RestrictedLaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for RestrictedLaunchActivatedEventArgs {}
#[repr(transparent)]
pub struct SearchActivatedEventArgs(::windows_core::IUnknown);
impl SearchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = &::windows_core::Interface::cast::<ISearchActivatedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Search::SearchPaneQueryLinguisticDetails>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
impl ::core::clone::Clone for SearchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SearchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchActivatedEventArgs {}
impl ::core::fmt::Debug for SearchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SearchActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SearchActivatedEventArgs;{8cb36951-58c8-43e3-94bc-41d33f8b630e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <ISearchActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SearchActivatedEventArgs";
}
impl ::core::convert::From<SearchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: SearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SearchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: SearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for ISearchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for ISearchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISearchActivatedEventArgs> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ISearchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISearchActivatedEventArgs> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ISearchActivatedEventArgs> {
        ::core::convert::TryInto::<ISearchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISearchActivatedEventArgsWithLinguisticDetails> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ISearchActivatedEventArgsWithLinguisticDetails> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISearchActivatedEventArgsWithLinguisticDetails> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ISearchActivatedEventArgsWithLinguisticDetails> {
        ::core::convert::TryInto::<ISearchActivatedEventArgsWithLinguisticDetails>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IViewSwitcherProvider> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SearchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for SearchActivatedEventArgs {}
#[repr(transparent)]
pub struct ShareTargetActivatedEventArgs(::windows_core::IUnknown);
impl ShareTargetActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> ::windows_core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShareOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DataTransfer::ShareTarget::ShareOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for ShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ShareTargetActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareTargetActivatedEventArgs {}
impl ::core::fmt::Debug for ShareTargetActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareTargetActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs;{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IShareTargetActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs";
}
impl ::core::convert::From<ShareTargetActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ShareTargetActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IShareTargetActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IShareTargetActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IShareTargetActivatedEventArgs> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IShareTargetActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IShareTargetActivatedEventArgs> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IShareTargetActivatedEventArgs> {
        ::core::convert::TryInto::<IShareTargetActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ShareTargetActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ShareTargetActivatedEventArgs {}
#[repr(transparent)]
pub struct SplashScreen(::windows_core::IUnknown);
impl SplashScreen {
    #[cfg(feature = "Foundation")]
    pub fn ImageLocation(&self) -> ::windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).ImageLocation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Dismissed<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SplashScreen, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Dismissed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDismissed<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDismissed)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SplashScreen {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplashScreen {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplashScreen {}
impl ::core::fmt::Debug for SplashScreen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplashScreen").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SplashScreen {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SplashScreen;{ca4d975c-d4d6-43f0-97c0-0833c6391c24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SplashScreen {
    type Vtable = ISplashScreen_Vtbl;
    const IID: ::windows_core::GUID = <ISplashScreen as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SplashScreen {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SplashScreen";
}
impl ::core::convert::From<SplashScreen> for ::windows_core::IUnknown {
    fn from(value: SplashScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplashScreen> for ::windows_core::IUnknown {
    fn from(value: &SplashScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SplashScreen {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SplashScreen {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplashScreen> for ::windows_core::IInspectable {
    fn from(value: SplashScreen) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplashScreen> for ::windows_core::IInspectable {
    fn from(value: &SplashScreen) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SplashScreen {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SplashScreen {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct StartupTaskActivatedEventArgs(::windows_core::IUnknown);
impl StartupTaskActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StartupTaskActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StartupTaskActivatedEventArgs {}
impl ::core::fmt::Debug for StartupTaskActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StartupTaskActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs;{03b11a58-5276-4d91-8621-54611864d5fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IStartupTaskActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs";
}
impl ::core::convert::From<StartupTaskActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: StartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StartupTaskActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: StartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IStartupTaskActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IStartupTaskActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStartupTaskActivatedEventArgs> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IStartupTaskActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IStartupTaskActivatedEventArgs> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IStartupTaskActivatedEventArgs> {
        ::core::convert::TryInto::<IStartupTaskActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for StartupTaskActivatedEventArgs {}
unsafe impl ::core::marker::Sync for StartupTaskActivatedEventArgs {}
#[repr(transparent)]
pub struct TileActivatedInfo(::windows_core::IUnknown);
impl TileActivatedInfo {
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    pub fn RecentlyShownNotifications(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RecentlyShownNotifications)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>>(result__)
        }
    }
}
impl ::core::clone::Clone for TileActivatedInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileActivatedInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileActivatedInfo {}
impl ::core::fmt::Debug for TileActivatedInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileActivatedInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TileActivatedInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.TileActivatedInfo;{80e4a3b1-3980-4f17-b738-89194e0b8f65})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TileActivatedInfo {
    type Vtable = ITileActivatedInfo_Vtbl;
    const IID: ::windows_core::GUID = <ITileActivatedInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TileActivatedInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.TileActivatedInfo";
}
impl ::core::convert::From<TileActivatedInfo> for ::windows_core::IUnknown {
    fn from(value: TileActivatedInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileActivatedInfo> for ::windows_core::IUnknown {
    fn from(value: &TileActivatedInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TileActivatedInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TileActivatedInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TileActivatedInfo> for ::windows_core::IInspectable {
    fn from(value: TileActivatedInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileActivatedInfo> for ::windows_core::IInspectable {
    fn from(value: &TileActivatedInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TileActivatedInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TileActivatedInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TileActivatedInfo {}
unsafe impl ::core::marker::Sync for TileActivatedInfo {}
#[repr(transparent)]
pub struct ToastNotificationActivatedEventArgs(::windows_core::IUnknown);
impl ToastNotificationActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Argument(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Argument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UserInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationActivatedEventArgs {}
impl ::core::fmt::Debug for ToastNotificationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs;{92a86f82-5290-431d-be85-c4aaeeb8685f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IToastNotificationActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs";
}
impl ::core::convert::From<ToastNotificationActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastNotificationActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IToastNotificationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IToastNotificationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IToastNotificationActivatedEventArgs> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IToastNotificationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IToastNotificationActivatedEventArgs> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IToastNotificationActivatedEventArgs> {
        ::core::convert::TryInto::<IToastNotificationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ToastNotificationActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ToastNotificationActivatedEventArgs {}
#[repr(transparent)]
pub struct UserDataAccountProviderActivatedEventArgs(::windows_core::IUnknown);
impl UserDataAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> ::windows_core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountProviderActivatedEventArgs {}
impl ::core::fmt::Debug for UserDataAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs;{1bc9f723-8ef1-4a51-a63a-fe711eeab607})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataAccountProviderActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs";
}
impl ::core::convert::From<UserDataAccountProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: UserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: UserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: UserDataAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &UserDataAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: UserDataAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &UserDataAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUserDataAccountProviderActivatedEventArgs> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IUserDataAccountProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IUserDataAccountProviderActivatedEventArgs> for &UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IUserDataAccountProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IUserDataAccountProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderActivatedEventArgs {}
#[repr(transparent)]
pub struct VoiceCommandActivatedEventArgs(::windows_core::IUnknown);
impl VoiceCommandActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> ::windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
}
impl ::core::clone::Clone for VoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VoiceCommandActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandActivatedEventArgs {}
impl ::core::fmt::Debug for VoiceCommandActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for VoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs;{ab92dcfd-8d43-4de6-9775-20704b581b00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for VoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IVoiceCommandActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs";
}
impl ::core::convert::From<VoiceCommandActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: VoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VoiceCommandActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: VoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IVoiceCommandActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IVoiceCommandActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IVoiceCommandActivatedEventArgs> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IVoiceCommandActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IVoiceCommandActivatedEventArgs> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IVoiceCommandActivatedEventArgs> {
        ::core::convert::TryInto::<IVoiceCommandActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VoiceCommandActivatedEventArgs {}
unsafe impl ::core::marker::Sync for VoiceCommandActivatedEventArgs {}
#[repr(transparent)]
pub struct WalletActionActivatedEventArgs(::windows_core::IUnknown);
impl WalletActionActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    pub fn ItemId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ItemId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Wallet")]
    pub fn ActionKind(&self) -> ::windows_core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Wallet::WalletActionKind>::zeroed();
            (::windows_core::Interface::vtable(this).ActionKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Wallet::WalletActionKind>(result__)
        }
    }
    pub fn ActionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for WalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WalletActionActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WalletActionActivatedEventArgs {}
impl ::core::fmt::Debug for WalletActionActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletActionActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WalletActionActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs;{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWalletActionActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs";
}
impl ::core::convert::From<WalletActionActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WalletActionActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WalletActionActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WalletActionActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<WalletActionActivatedEventArgs> for IWalletActionActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WalletActionActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WalletActionActivatedEventArgs> for IWalletActionActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WalletActionActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWalletActionActivatedEventArgs> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IWalletActionActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWalletActionActivatedEventArgs> for &WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IWalletActionActivatedEventArgs> {
        ::core::convert::TryInto::<IWalletActionActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WalletActionActivatedEventArgs {}
unsafe impl ::core::marker::Sync for WalletActionActivatedEventArgs {}
#[repr(transparent)]
pub struct WebAccountProviderActivatedEventArgs(::windows_core::IUnknown);
impl WebAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> ::windows_core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderActivatedEventArgs {}
impl ::core::fmt::Debug for WebAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs;{72b71774-98ea-4ccf-9752-46d9051004f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebAccountProviderActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs";
}
impl ::core::convert::From<WebAccountProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsWithUser> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWebAccountProviderActivatedEventArgs> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IWebAccountProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWebAccountProviderActivatedEventArgs> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IWebAccountProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IWebAccountProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for WebAccountProviderActivatedEventArgs {}
#[repr(transparent)]
pub struct WebAuthenticationBrokerContinuationEventArgs(::windows_core::IUnknown);
impl WebAuthenticationBrokerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivationKind>(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> ::windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WebAuthenticationResult)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAuthenticationBrokerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAuthenticationBrokerContinuationEventArgs {}
impl ::core::fmt::Debug for WebAuthenticationBrokerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAuthenticationBrokerContinuationEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs;{75dda3d4-7714-453d-b7ff-b95e3a1709da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebAuthenticationBrokerContinuationEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs";
}
impl ::core::convert::From<WebAuthenticationBrokerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAuthenticationBrokerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgs> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IContinuationActivatedEventArgs> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWebAuthenticationBrokerContinuationEventArgs> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IWebAuthenticationBrokerContinuationEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWebAuthenticationBrokerContinuationEventArgs> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IWebAuthenticationBrokerContinuationEventArgs> {
        ::core::convert::TryInto::<IWebAuthenticationBrokerContinuationEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAuthenticationBrokerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for WebAuthenticationBrokerContinuationEventArgs {}