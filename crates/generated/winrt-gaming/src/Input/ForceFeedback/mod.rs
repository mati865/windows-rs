#[repr(transparent)]
pub struct ConditionForceEffect(::windows_core::IUnknown);
impl ConditionForceEffect {
    pub fn Kind(&self) -> ::windows_core::Result<ConditionForceEffectKind> {
        let this = &::windows_core::Interface::cast::<IConditionForceEffect>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ConditionForceEffectKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConditionForceEffectKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(&self, direction: Param0, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IConditionForceEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetParameters)(::windows_core::Interface::as_raw(this), direction.into_param().abi(), positivecoefficient, negativecoefficient, maxpositivemagnitude, maxnegativemagnitude, deadzone, bias).ok() }
    }
    pub fn CreateInstance(effectkind: ConditionForceEffectKind) -> ::windows_core::Result<ConditionForceEffect> {
        Self::IConditionForceEffectFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), effectkind, result__.as_mut_ptr()).from_abi::<ConditionForceEffect>(result__)
        })
    }
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Gain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ForceFeedbackEffectState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IConditionForceEffectFactory<R, F: FnOnce(&IConditionForceEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ConditionForceEffect, IConditionForceEffectFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ConditionForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConditionForceEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConditionForceEffect {}
impl ::core::fmt::Debug for ConditionForceEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConditionForceEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConditionForceEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ConditionForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConditionForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: ::windows_core::GUID = <IForceFeedbackEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConditionForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConditionForceEffect";
}
impl ::core::convert::From<ConditionForceEffect> for ::windows_core::IUnknown {
    fn from(value: ConditionForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConditionForceEffect> for ::windows_core::IUnknown {
    fn from(value: &ConditionForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConditionForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConditionForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConditionForceEffect> for ::windows_core::IInspectable {
    fn from(value: ConditionForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConditionForceEffect> for ::windows_core::IInspectable {
    fn from(value: &ConditionForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConditionForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConditionForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ConditionForceEffect> for IForceFeedbackEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: ConditionForceEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ConditionForceEffect> for IForceFeedbackEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: &ConditionForceEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IForceFeedbackEffect> for ConditionForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IForceFeedbackEffect> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IForceFeedbackEffect> for &ConditionForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IForceFeedbackEffect> {
        ::core::convert::TryInto::<IForceFeedbackEffect>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ConditionForceEffect {}
unsafe impl ::core::marker::Sync for ConditionForceEffect {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConditionForceEffectKind(pub i32);
impl ConditionForceEffectKind {
    pub const Spring: Self = Self(0i32);
    pub const Damper: Self = Self(1i32);
    pub const Inertia: Self = Self(2i32);
    pub const Friction: Self = Self(3i32);
}
impl ::core::marker::Copy for ConditionForceEffectKind {}
impl ::core::clone::Clone for ConditionForceEffectKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConditionForceEffectKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ConditionForceEffectKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConditionForceEffectKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConditionForceEffectKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConditionForceEffectKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ConditionForceEffectKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ConstantForceEffect(::windows_core::IUnknown);
impl ConstantForceEffect {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ConstantForceEffect, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, vector: Param0, duration: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IConstantForceEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetParameters)(::windows_core::Interface::as_raw(this), vector.into_param().abi(), duration.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param4: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param5: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param6: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param7: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, vector: Param0, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: Param4, attackduration: Param5, sustainduration: Param6, releaseduration: Param7, repeatcount: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IConstantForceEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetParametersWithEnvelope)(::windows_core::Interface::as_raw(this), vector.into_param().abi(), attackgain, sustaingain, releasegain, startdelay.into_param().abi(), attackduration.into_param().abi(), sustainduration.into_param().abi(), releaseduration.into_param().abi(), repeatcount).ok() }
    }
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Gain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ForceFeedbackEffectState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ForceFeedbackEffectState>(result__)
        }
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
impl ::core::clone::Clone for ConstantForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConstantForceEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConstantForceEffect {}
impl ::core::fmt::Debug for ConstantForceEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConstantForceEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConstantForceEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ConstantForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConstantForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: ::windows_core::GUID = <IForceFeedbackEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConstantForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ConstantForceEffect";
}
impl ::core::convert::From<ConstantForceEffect> for ::windows_core::IUnknown {
    fn from(value: ConstantForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConstantForceEffect> for ::windows_core::IUnknown {
    fn from(value: &ConstantForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConstantForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConstantForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConstantForceEffect> for ::windows_core::IInspectable {
    fn from(value: ConstantForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConstantForceEffect> for ::windows_core::IInspectable {
    fn from(value: &ConstantForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConstantForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConstantForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ConstantForceEffect> for IForceFeedbackEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: ConstantForceEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ConstantForceEffect> for IForceFeedbackEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: &ConstantForceEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IForceFeedbackEffect> for ConstantForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IForceFeedbackEffect> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IForceFeedbackEffect> for &ConstantForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IForceFeedbackEffect> {
        ::core::convert::TryInto::<IForceFeedbackEffect>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ConstantForceEffect {}
unsafe impl ::core::marker::Sync for ConstantForceEffect {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ForceFeedbackEffectAxes(pub u32);
impl ForceFeedbackEffectAxes {
    pub const None: Self = Self(0u32);
    pub const X: Self = Self(1u32);
    pub const Y: Self = Self(2u32);
    pub const Z: Self = Self(4u32);
}
impl ::core::marker::Copy for ForceFeedbackEffectAxes {}
impl ::core::clone::Clone for ForceFeedbackEffectAxes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ForceFeedbackEffectAxes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ForceFeedbackEffectAxes {
    type Abi = Self;
}
impl ::core::fmt::Debug for ForceFeedbackEffectAxes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackEffectAxes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ForceFeedbackEffectAxes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ForceFeedbackEffectAxes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ForceFeedbackEffectAxes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ForceFeedbackEffectAxes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ForceFeedbackEffectAxes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for ForceFeedbackEffectAxes {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectAxes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ForceFeedbackEffectState(pub i32);
impl ForceFeedbackEffectState {
    pub const Stopped: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Faulted: Self = Self(3i32);
}
impl ::core::marker::Copy for ForceFeedbackEffectState {}
impl ::core::clone::Clone for ForceFeedbackEffectState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ForceFeedbackEffectState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ForceFeedbackEffectState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ForceFeedbackEffectState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackEffectState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ForceFeedbackEffectState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackEffectState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ForceFeedbackLoadEffectResult(pub i32);
impl ForceFeedbackLoadEffectResult {
    pub const Succeeded: Self = Self(0i32);
    pub const EffectStorageFull: Self = Self(1i32);
    pub const EffectNotSupported: Self = Self(2i32);
}
impl ::core::marker::Copy for ForceFeedbackLoadEffectResult {}
impl ::core::clone::Clone for ForceFeedbackLoadEffectResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ForceFeedbackLoadEffectResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ForceFeedbackLoadEffectResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ForceFeedbackLoadEffectResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackLoadEffectResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ForceFeedbackLoadEffectResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.ForceFeedbackLoadEffectResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ForceFeedbackMotor(::windows_core::IUnknown);
impl ForceFeedbackMotor {
    pub fn AreEffectsPaused(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreEffectsPaused)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MasterGain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MasterGain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetMasterGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMasterGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SupportedAxes(&self) -> ::windows_core::Result<ForceFeedbackEffectAxes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ForceFeedbackEffectAxes>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedAxes)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ForceFeedbackEffectAxes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn LoadEffectAsync<'a, Param0: ::windows_core::IntoParam<'a, IForceFeedbackEffect>>(&self, effect: Param0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LoadEffectAsync)(::windows_core::Interface::as_raw(this), effect.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<ForceFeedbackLoadEffectResult>>(result__)
        }
    }
    pub fn PauseAllEffects(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PauseAllEffects)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ResumeAllEffects(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ResumeAllEffects)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StopAllEffects(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopAllEffects)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryDisableAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryDisableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryEnableAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryEnableAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryResetAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryResetAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryUnloadEffectAsync<'a, Param0: ::windows_core::IntoParam<'a, IForceFeedbackEffect>>(&self, effect: Param0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryUnloadEffectAsync)(::windows_core::Interface::as_raw(this), effect.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for ForceFeedbackMotor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ForceFeedbackMotor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ForceFeedbackMotor {}
impl ::core::fmt::Debug for ForceFeedbackMotor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForceFeedbackMotor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ForceFeedbackMotor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor;{8d3d417c-a5ea-4516-8026-2b00f74ef6e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ForceFeedbackMotor {
    type Vtable = IForceFeedbackMotor_Vtbl;
    const IID: ::windows_core::GUID = <IForceFeedbackMotor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ForceFeedbackMotor {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.ForceFeedbackMotor";
}
impl ::core::convert::From<ForceFeedbackMotor> for ::windows_core::IUnknown {
    fn from(value: ForceFeedbackMotor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ForceFeedbackMotor> for ::windows_core::IUnknown {
    fn from(value: &ForceFeedbackMotor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ForceFeedbackMotor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ForceFeedbackMotor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ForceFeedbackMotor> for ::windows_core::IInspectable {
    fn from(value: ForceFeedbackMotor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ForceFeedbackMotor> for ::windows_core::IInspectable {
    fn from(value: &ForceFeedbackMotor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ForceFeedbackMotor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ForceFeedbackMotor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ForceFeedbackMotor {}
unsafe impl ::core::marker::Sync for ForceFeedbackMotor {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConditionForceEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConditionForceEffect {
    type Vtable = IConditionForceEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32d1ea68_3695_4e69_85c0_cd1944189140);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionForceEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ConditionForceEffectKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: super::super::super::Foundation::Numerics::Vector3, positivecoefficient: f32, negativecoefficient: f32, maxpositivemagnitude: f32, maxnegativemagnitude: f32, deadzone: f32, bias: f32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConditionForceEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConditionForceEffectFactory {
    type Vtable = IConditionForceEffectFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91a99264_1810_4eb6_a773_bfd3b8cddbab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConditionForceEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectkind: ConditionForceEffectKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConstantForceEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConstantForceEffect {
    type Vtable = IConstantForceEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9bfa0140_f3c7_415c_b068_0f068734bce0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstantForceEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
#[repr(transparent)]
pub struct IForceFeedbackEffect(::windows_core::IUnknown);
impl IForceFeedbackEffect {
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Gain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ForceFeedbackEffectState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ForceFeedbackEffectState>(result__)
        }
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
impl ::core::convert::From<IForceFeedbackEffect> for ::windows_core::IUnknown {
    fn from(value: IForceFeedbackEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IForceFeedbackEffect> for ::windows_core::IUnknown {
    fn from(value: &IForceFeedbackEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IForceFeedbackEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IForceFeedbackEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IForceFeedbackEffect> for ::windows_core::IInspectable {
    fn from(value: IForceFeedbackEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IForceFeedbackEffect> for ::windows_core::IInspectable {
    fn from(value: &IForceFeedbackEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IForceFeedbackEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IForceFeedbackEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IForceFeedbackEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IForceFeedbackEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IForceFeedbackEffect {}
impl ::core::fmt::Debug for IForceFeedbackEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IForceFeedbackEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IForceFeedbackEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{a17fba0c-2ae4-48c2-8063-eabd0777cb89}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IForceFeedbackEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa17fba0c_2ae4_48c2_8063_eabd0777cb89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForceFeedbackEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Gain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ForceFeedbackEffectState) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IForceFeedbackMotor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IForceFeedbackMotor {
    type Vtable = IForceFeedbackMotor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d3d417c_a5ea_4516_8026_2b00f74ef6e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForceFeedbackMotor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AreEffectsPaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MasterGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetMasterGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SupportedAxes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ForceFeedbackEffectAxes) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LoadEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadEffectAsync: usize,
    pub PauseAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ResumeAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StopAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryDisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryEnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryEnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryResetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryResetAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUnloadEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUnloadEffectAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeriodicForceEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPeriodicForceEffect {
    type Vtable = IPeriodicForceEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c5138d7_fc75_4d52_9a0a_efe4cab5fe64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeriodicForceEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PeriodicForceEffectKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, duration: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vector: super::super::super::Foundation::Numerics::Vector3, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeriodicForceEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPeriodicForceEffectFactory {
    type Vtable = IPeriodicForceEffectFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f62eb1a_9851_477b_b318_35ecaa15070f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeriodicForceEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectkind: PeriodicForceEffectKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRampForceEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRampForceEffect {
    type Vtable = IRampForceEffect_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1f81259_1ca6_4080_b56d_b43f3354d052);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRampForceEffect_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, duration: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParameters: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetParametersWithEnvelope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startvector: super::super::super::Foundation::Numerics::Vector3, endvector: super::super::super::Foundation::Numerics::Vector3, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: super::super::super::Foundation::TimeSpan, attackduration: super::super::super::Foundation::TimeSpan, sustainduration: super::super::super::Foundation::TimeSpan, releaseduration: super::super::super::Foundation::TimeSpan, repeatcount: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetParametersWithEnvelope: usize,
}
#[repr(transparent)]
pub struct PeriodicForceEffect(::windows_core::IUnknown);
impl PeriodicForceEffect {
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Gain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ForceFeedbackEffectState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<PeriodicForceEffectKind> {
        let this = &::windows_core::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PeriodicForceEffectKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PeriodicForceEffectKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param4: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, vector: Param0, frequency: f32, phase: f32, bias: f32, duration: Param4) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetParameters)(::windows_core::Interface::as_raw(this), vector.into_param().abi(), frequency, phase, bias, duration.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param7: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param8: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param9: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param10: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, vector: Param0, frequency: f32, phase: f32, bias: f32, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: Param7, attackduration: Param8, sustainduration: Param9, releaseduration: Param10, repeatcount: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPeriodicForceEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetParametersWithEnvelope)(::windows_core::Interface::as_raw(this), vector.into_param().abi(), frequency, phase, bias, attackgain, sustaingain, releasegain, startdelay.into_param().abi(), attackduration.into_param().abi(), sustainduration.into_param().abi(), releaseduration.into_param().abi(), repeatcount).ok() }
    }
    pub fn CreateInstance(effectkind: PeriodicForceEffectKind) -> ::windows_core::Result<PeriodicForceEffect> {
        Self::IPeriodicForceEffectFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), effectkind, result__.as_mut_ptr()).from_abi::<PeriodicForceEffect>(result__)
        })
    }
    pub fn IPeriodicForceEffectFactory<R, F: FnOnce(&IPeriodicForceEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PeriodicForceEffect, IPeriodicForceEffectFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PeriodicForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PeriodicForceEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PeriodicForceEffect {}
impl ::core::fmt::Debug for PeriodicForceEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeriodicForceEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PeriodicForceEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PeriodicForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: ::windows_core::GUID = <IForceFeedbackEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PeriodicForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.PeriodicForceEffect";
}
impl ::core::convert::From<PeriodicForceEffect> for ::windows_core::IUnknown {
    fn from(value: PeriodicForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PeriodicForceEffect> for ::windows_core::IUnknown {
    fn from(value: &PeriodicForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PeriodicForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PeriodicForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PeriodicForceEffect> for ::windows_core::IInspectable {
    fn from(value: PeriodicForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PeriodicForceEffect> for ::windows_core::IInspectable {
    fn from(value: &PeriodicForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PeriodicForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PeriodicForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PeriodicForceEffect> for IForceFeedbackEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: PeriodicForceEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PeriodicForceEffect> for IForceFeedbackEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: &PeriodicForceEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IForceFeedbackEffect> for PeriodicForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IForceFeedbackEffect> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IForceFeedbackEffect> for &PeriodicForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IForceFeedbackEffect> {
        ::core::convert::TryInto::<IForceFeedbackEffect>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PeriodicForceEffect {}
unsafe impl ::core::marker::Sync for PeriodicForceEffect {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PeriodicForceEffectKind(pub i32);
impl PeriodicForceEffectKind {
    pub const SquareWave: Self = Self(0i32);
    pub const SineWave: Self = Self(1i32);
    pub const TriangleWave: Self = Self(2i32);
    pub const SawtoothWaveUp: Self = Self(3i32);
    pub const SawtoothWaveDown: Self = Self(4i32);
}
impl ::core::marker::Copy for PeriodicForceEffectKind {}
impl ::core::clone::Clone for PeriodicForceEffectKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PeriodicForceEffectKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PeriodicForceEffectKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PeriodicForceEffectKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeriodicForceEffectKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PeriodicForceEffectKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.ForceFeedback.PeriodicForceEffectKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RampForceEffect(::windows_core::IUnknown);
impl RampForceEffect {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RampForceEffect, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Gain(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Gain)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetGain(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGain)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<ForceFeedbackEffectState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ForceFeedbackEffectState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ForceFeedbackEffectState>(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParameters<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, startvector: Param0, endvector: Param1, duration: Param2) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IRampForceEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetParameters)(::windows_core::Interface::as_raw(this), startvector.into_param().abi(), endvector.into_param().abi(), duration.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetParametersWithEnvelope<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param1: ::windows_core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param5: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param6: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param7: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>, Param8: ::windows_core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, startvector: Param0, endvector: Param1, attackgain: f32, sustaingain: f32, releasegain: f32, startdelay: Param5, attackduration: Param6, sustainduration: Param7, releaseduration: Param8, repeatcount: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IRampForceEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetParametersWithEnvelope)(::windows_core::Interface::as_raw(this), startvector.into_param().abi(), endvector.into_param().abi(), attackgain, sustaingain, releasegain, startdelay.into_param().abi(), attackduration.into_param().abi(), sustainduration.into_param().abi(), releaseduration.into_param().abi(), repeatcount).ok() }
    }
}
impl ::core::clone::Clone for RampForceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RampForceEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RampForceEffect {}
impl ::core::fmt::Debug for RampForceEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RampForceEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RampForceEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.ForceFeedback.RampForceEffect;{a17fba0c-2ae4-48c2-8063-eabd0777cb89})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RampForceEffect {
    type Vtable = IForceFeedbackEffect_Vtbl;
    const IID: ::windows_core::GUID = <IForceFeedbackEffect as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RampForceEffect {
    const NAME: &'static str = "Windows.Gaming.Input.ForceFeedback.RampForceEffect";
}
impl ::core::convert::From<RampForceEffect> for ::windows_core::IUnknown {
    fn from(value: RampForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RampForceEffect> for ::windows_core::IUnknown {
    fn from(value: &RampForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RampForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RampForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RampForceEffect> for ::windows_core::IInspectable {
    fn from(value: RampForceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RampForceEffect> for ::windows_core::IInspectable {
    fn from(value: &RampForceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RampForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RampForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RampForceEffect> for IForceFeedbackEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: RampForceEffect) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RampForceEffect> for IForceFeedbackEffect {
    type Error = ::windows_core::Error;
    fn try_from(value: &RampForceEffect) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IForceFeedbackEffect> for RampForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IForceFeedbackEffect> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IForceFeedbackEffect> for &RampForceEffect {
    fn into_param(self) -> ::windows_core::Param<'a, IForceFeedbackEffect> {
        ::core::convert::TryInto::<IForceFeedbackEffect>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RampForceEffect {}
unsafe impl ::core::marker::Sync for RampForceEffect {}