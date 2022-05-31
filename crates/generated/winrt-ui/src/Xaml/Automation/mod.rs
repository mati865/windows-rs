#[cfg(feature = "Peers")]
pub mod Peers;
#[cfg(feature = "Provider")]
pub mod Provider;
#[cfg(feature = "Text")]
pub mod Text;
#[repr(transparent)]
pub struct AnnotationPatternIdentifiers(::windows_core::IUnknown);
impl AnnotationPatternIdentifiers {
    pub fn AnnotationTypeIdProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationTypeIdProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn AnnotationTypeNameProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationTypeNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn AuthorProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AuthorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn DateTimeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DateTimeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn TargetProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAnnotationPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IAnnotationPatternIdentifiersStatics<R, F: FnOnce(&IAnnotationPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AnnotationPatternIdentifiers, IAnnotationPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AnnotationPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AnnotationPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AnnotationPatternIdentifiers {}
impl ::core::fmt::Debug for AnnotationPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnnotationPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AnnotationPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers;{d475a0c1-48b2-4e40-a6cf-3dc4b638c0de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AnnotationPatternIdentifiers {
    type Vtable = IAnnotationPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IAnnotationPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AnnotationPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers";
}
impl ::core::convert::From<AnnotationPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: AnnotationPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnnotationPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &AnnotationPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AnnotationPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AnnotationPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AnnotationPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: AnnotationPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AnnotationPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &AnnotationPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AnnotationPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AnnotationPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AnnotationPatternIdentifiers {}
unsafe impl ::core::marker::Sync for AnnotationPatternIdentifiers {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AnnotationType(pub i32);
impl AnnotationType {
    pub const Unknown: Self = Self(60000i32);
    pub const SpellingError: Self = Self(60001i32);
    pub const GrammarError: Self = Self(60002i32);
    pub const Comment: Self = Self(60003i32);
    pub const FormulaError: Self = Self(60004i32);
    pub const TrackChanges: Self = Self(60005i32);
    pub const Header: Self = Self(60006i32);
    pub const Footer: Self = Self(60007i32);
    pub const Highlighted: Self = Self(60008i32);
    pub const Endnote: Self = Self(60009i32);
    pub const Footnote: Self = Self(60010i32);
    pub const InsertionChange: Self = Self(60011i32);
    pub const DeletionChange: Self = Self(60012i32);
    pub const MoveChange: Self = Self(60013i32);
    pub const FormatChange: Self = Self(60014i32);
    pub const UnsyncedChange: Self = Self(60015i32);
    pub const EditingLockedChange: Self = Self(60016i32);
    pub const ExternalChange: Self = Self(60017i32);
    pub const ConflictingChange: Self = Self(60018i32);
    pub const Author: Self = Self(60019i32);
    pub const AdvancedProofingIssue: Self = Self(60020i32);
    pub const DataValidationError: Self = Self(60021i32);
    pub const CircularReferenceError: Self = Self(60022i32);
}
impl ::core::marker::Copy for AnnotationType {}
impl ::core::clone::Clone for AnnotationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AnnotationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AnnotationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AnnotationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnnotationType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AnnotationType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AnnotationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomationActiveEnd(pub i32);
impl AutomationActiveEnd {
    pub const None: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const End: Self = Self(2i32);
}
impl ::core::marker::Copy for AutomationActiveEnd {}
impl ::core::clone::Clone for AutomationActiveEnd {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationActiveEnd {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomationActiveEnd {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomationActiveEnd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationActiveEnd").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationActiveEnd {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationActiveEnd;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomationAnimationStyle(pub i32);
impl AutomationAnimationStyle {
    pub const None: Self = Self(0i32);
    pub const LasVegasLights: Self = Self(1i32);
    pub const BlinkingBackground: Self = Self(2i32);
    pub const SparkleText: Self = Self(3i32);
    pub const MarchingBlackAnts: Self = Self(4i32);
    pub const MarchingRedAnts: Self = Self(5i32);
    pub const Shimmer: Self = Self(6i32);
    pub const Other: Self = Self(7i32);
}
impl ::core::marker::Copy for AutomationAnimationStyle {}
impl ::core::clone::Clone for AutomationAnimationStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationAnimationStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomationAnimationStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomationAnimationStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationAnimationStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationAnimationStyle {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationAnimationStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AutomationAnnotation(::windows_core::IUnknown);
impl AutomationAnnotation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationAnnotation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Type(&self) -> ::windows_core::Result<AnnotationType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AnnotationType>::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AnnotationType>(result__)
        }
    }
    pub fn SetType(&self, value: AnnotationType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Element(&self) -> ::windows_core::Result<super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Element)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UIElement>(result__)
        }
    }
    pub fn SetElement<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetElement)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateInstance(r#type: AnnotationType) -> ::windows_core::Result<AutomationAnnotation> {
        Self::IAutomationAnnotationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<AutomationAnnotation>(result__)
        })
    }
    pub fn CreateWithElementParameter<'a, Param1: ::windows_core::IntoParam<'a, super::UIElement>>(r#type: AnnotationType, element: Param1) -> ::windows_core::Result<AutomationAnnotation> {
        Self::IAutomationAnnotationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithElementParameter)(::windows_core::Interface::as_raw(this), r#type, element.into_param().abi(), result__.as_mut_ptr()).from_abi::<AutomationAnnotation>(result__)
        })
    }
    pub fn TypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationAnnotationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn ElementProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationAnnotationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IAutomationAnnotationFactory<R, F: FnOnce(&IAutomationAnnotationFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationAnnotation, IAutomationAnnotationFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationAnnotationStatics<R, F: FnOnce(&IAutomationAnnotationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationAnnotation, IAutomationAnnotationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AutomationAnnotation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationAnnotation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationAnnotation {}
impl ::core::fmt::Debug for AutomationAnnotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationAnnotation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationAnnotation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AutomationAnnotation;{fb3c30ca-03d8-4618-91bf-e4d84f4af318})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AutomationAnnotation {
    type Vtable = IAutomationAnnotation_Vtbl;
    const IID: ::windows_core::GUID = <IAutomationAnnotation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AutomationAnnotation {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AutomationAnnotation";
}
impl ::core::convert::From<AutomationAnnotation> for ::windows_core::IUnknown {
    fn from(value: AutomationAnnotation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationAnnotation> for ::windows_core::IUnknown {
    fn from(value: &AutomationAnnotation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AutomationAnnotation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AutomationAnnotation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationAnnotation> for ::windows_core::IInspectable {
    fn from(value: AutomationAnnotation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationAnnotation> for ::windows_core::IInspectable {
    fn from(value: &AutomationAnnotation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AutomationAnnotation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AutomationAnnotation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationAnnotation> for super::DependencyObject {
    fn from(value: AutomationAnnotation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&AutomationAnnotation> for super::DependencyObject {
    fn from(value: &AutomationAnnotation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for AutomationAnnotation {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &AutomationAnnotation {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for AutomationAnnotation {}
unsafe impl ::core::marker::Sync for AutomationAnnotation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomationBulletStyle(pub i32);
impl AutomationBulletStyle {
    pub const None: Self = Self(0i32);
    pub const HollowRoundBullet: Self = Self(1i32);
    pub const FilledRoundBullet: Self = Self(2i32);
    pub const HollowSquareBullet: Self = Self(3i32);
    pub const FilledSquareBullet: Self = Self(4i32);
    pub const DashBullet: Self = Self(5i32);
    pub const Other: Self = Self(6i32);
}
impl ::core::marker::Copy for AutomationBulletStyle {}
impl ::core::clone::Clone for AutomationBulletStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationBulletStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomationBulletStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomationBulletStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationBulletStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationBulletStyle {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationBulletStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomationCaretBidiMode(pub i32);
impl AutomationCaretBidiMode {
    pub const LTR: Self = Self(0i32);
    pub const RTL: Self = Self(1i32);
}
impl ::core::marker::Copy for AutomationCaretBidiMode {}
impl ::core::clone::Clone for AutomationCaretBidiMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationCaretBidiMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomationCaretBidiMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomationCaretBidiMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationCaretBidiMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationCaretBidiMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationCaretBidiMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomationCaretPosition(pub i32);
impl AutomationCaretPosition {
    pub const Unknown: Self = Self(0i32);
    pub const EndOfLine: Self = Self(1i32);
    pub const BeginningOfLine: Self = Self(2i32);
}
impl ::core::marker::Copy for AutomationCaretPosition {}
impl ::core::clone::Clone for AutomationCaretPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationCaretPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomationCaretPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomationCaretPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationCaretPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationCaretPosition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationCaretPosition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AutomationElementIdentifiers(::windows_core::IUnknown);
impl AutomationElementIdentifiers {
    pub fn AcceleratorKeyProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AcceleratorKeyProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn AccessKeyProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn AutomationIdProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AutomationIdProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn BoundingRectangleProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BoundingRectangleProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ClassNameProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClassNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ClickablePointProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClickablePointProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ControlTypeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ControlTypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn HasKeyboardFocusProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HasKeyboardFocusProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn HelpTextProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HelpTextProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsContentElementProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsContentElementProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsControlElementProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsControlElementProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsEnabledProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsKeyboardFocusableProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsKeyboardFocusableProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsOffscreenProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsOffscreenProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsPasswordProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsPasswordProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsRequiredForFormProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsRequiredForFormProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ItemStatusProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ItemStatusProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ItemTypeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ItemTypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LabeledByProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LabeledByProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LocalizedControlTypeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LocalizedControlTypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn NameProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn OrientationProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OrientationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LiveSettingProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LiveSettingProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ControlledPeersProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ControlledPeersProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn PositionInSetProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PositionInSetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn SizeOfSetProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SizeOfSetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LevelProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LevelProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn AnnotationsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LandmarkTypeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LandmarkTypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LocalizedLandmarkTypeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LocalizedLandmarkTypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsPeripheralProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsPeripheralProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsDataValidForFormProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsDataValidForFormProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn FullDescriptionProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FullDescriptionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn DescribedByProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DescribedByProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn FlowsToProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlowsToProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn FlowsFromProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlowsFromProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn CultureProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics6(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CultureProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn HeadingLevelProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics7(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HeadingLevelProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsDialogProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IAutomationElementIdentifiersStatics8(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsDialogProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IAutomationElementIdentifiersStatics<R, F: FnOnce(&IAutomationElementIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics2<R, F: FnOnce(&IAutomationElementIdentifiersStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics3<R, F: FnOnce(&IAutomationElementIdentifiersStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics4<R, F: FnOnce(&IAutomationElementIdentifiersStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics5<R, F: FnOnce(&IAutomationElementIdentifiersStatics5) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics5> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics6<R, F: FnOnce(&IAutomationElementIdentifiersStatics6) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics6> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics7<R, F: FnOnce(&IAutomationElementIdentifiersStatics7) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics7> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationElementIdentifiersStatics8<R, F: FnOnce(&IAutomationElementIdentifiersStatics8) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationElementIdentifiers, IAutomationElementIdentifiersStatics8> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AutomationElementIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationElementIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationElementIdentifiers {}
impl ::core::fmt::Debug for AutomationElementIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationElementIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationElementIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AutomationElementIdentifiers;{e68a63cf-4345-4e2d-8a6a-49cce1fa2dcc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AutomationElementIdentifiers {
    type Vtable = IAutomationElementIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IAutomationElementIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AutomationElementIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AutomationElementIdentifiers";
}
impl ::core::convert::From<AutomationElementIdentifiers> for ::windows_core::IUnknown {
    fn from(value: AutomationElementIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationElementIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &AutomationElementIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AutomationElementIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AutomationElementIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationElementIdentifiers> for ::windows_core::IInspectable {
    fn from(value: AutomationElementIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationElementIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &AutomationElementIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AutomationElementIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AutomationElementIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutomationElementIdentifiers {}
unsafe impl ::core::marker::Sync for AutomationElementIdentifiers {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomationFlowDirections(pub i32);
impl AutomationFlowDirections {
    pub const Default: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
    pub const BottomToTop: Self = Self(2i32);
    pub const Vertical: Self = Self(3i32);
}
impl ::core::marker::Copy for AutomationFlowDirections {}
impl ::core::clone::Clone for AutomationFlowDirections {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationFlowDirections {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomationFlowDirections {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomationFlowDirections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationFlowDirections").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationFlowDirections {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationFlowDirections;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomationOutlineStyles(pub i32);
impl AutomationOutlineStyles {
    pub const None: Self = Self(0i32);
    pub const Outline: Self = Self(1i32);
    pub const Shadow: Self = Self(2i32);
    pub const Engraved: Self = Self(3i32);
    pub const Embossed: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationOutlineStyles {}
impl ::core::clone::Clone for AutomationOutlineStyles {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationOutlineStyles {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomationOutlineStyles {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomationOutlineStyles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationOutlineStyles").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationOutlineStyles {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationOutlineStyles;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AutomationProperties(::windows_core::IUnknown);
impl AutomationProperties {
    pub fn AcceleratorKeyProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AcceleratorKeyProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetAcceleratorKey<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetAcceleratorKey)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetAcceleratorKey<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetAcceleratorKey)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn AccessKeyProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AccessKeyProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetAccessKey<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetAccessKey)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetAccessKey<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetAccessKey)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn AutomationIdProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AutomationIdProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetAutomationId<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetAutomationId)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetAutomationId<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetAutomationId)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn HelpTextProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HelpTextProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetHelpText<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetHelpText)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetHelpText<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetHelpText)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn IsRequiredForFormProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsRequiredForFormProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsRequiredForForm<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<bool> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsRequiredForForm)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsRequiredForForm<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsRequiredForForm)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn ItemStatusProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ItemStatusProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetItemStatus<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemStatus)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetItemStatus<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetItemStatus)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn ItemTypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ItemTypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetItemType<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetItemType)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetItemType<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetItemType)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn LabeledByProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LabeledByProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetLabeledBy<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<super::UIElement> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetLabeledBy)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::UIElement>(result__)
        })
    }
    pub fn SetLabeledBy<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, super::UIElement>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetLabeledBy)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn NameProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetName<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetName)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetName<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn LiveSettingProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LiveSettingProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn GetLiveSetting<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<Peers::AutomationLiveSetting> {
        Self::IAutomationPropertiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Peers::AutomationLiveSetting>::zeroed();
            (::windows_core::Interface::vtable(this).GetLiveSetting)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<Peers::AutomationLiveSetting>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn SetLiveSetting<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: Peers::AutomationLiveSetting) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetLiveSetting)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn AccessibilityViewProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AccessibilityViewProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn GetAccessibilityView<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<Peers::AccessibilityView> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Peers::AccessibilityView>::zeroed();
            (::windows_core::Interface::vtable(this).GetAccessibilityView)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<Peers::AccessibilityView>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn SetAccessibilityView<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: Peers::AccessibilityView) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetAccessibilityView)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn ControlledPeersProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ControlledPeersProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControlledPeers<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<super::UIElement>> {
        Self::IAutomationPropertiesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetControlledPeers)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<super::UIElement>>(result__)
        })
    }
    pub fn PositionInSetProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PositionInSetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetPositionInSet<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<i32> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GetPositionInSet)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    pub fn SetPositionInSet<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).SetPositionInSet)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn SizeOfSetProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SizeOfSetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetSizeOfSet<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<i32> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GetSizeOfSet)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    pub fn SetSizeOfSet<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).SetSizeOfSet)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn LevelProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LevelProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetLevel<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<i32> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GetLevel)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    pub fn SetLevel<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).SetLevel)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn AnnotationsProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AnnotationsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAnnotations<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<AutomationAnnotation>> {
        Self::IAutomationPropertiesStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAnnotations)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<AutomationAnnotation>>(result__)
        })
    }
    pub fn LandmarkTypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LandmarkTypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn GetLandmarkType<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<Peers::AutomationLandmarkType> {
        Self::IAutomationPropertiesStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Peers::AutomationLandmarkType>::zeroed();
            (::windows_core::Interface::vtable(this).GetLandmarkType)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<Peers::AutomationLandmarkType>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn SetLandmarkType<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: Peers::AutomationLandmarkType) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).SetLandmarkType)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn LocalizedLandmarkTypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LocalizedLandmarkTypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetLocalizedLandmarkType<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAutomationPropertiesStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetLocalizedLandmarkType)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetLocalizedLandmarkType<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).SetLocalizedLandmarkType)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn IsPeripheralProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsPeripheralProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsPeripheral<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<bool> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsPeripheral)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsPeripheral<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics5(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsPeripheral)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn IsDataValidForFormProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsDataValidForFormProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsDataValidForForm<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<bool> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsDataValidForForm)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsDataValidForForm<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics5(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsDataValidForForm)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn FullDescriptionProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FullDescriptionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetFullDescription<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetFullDescription)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetFullDescription<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics5(|this| unsafe { (::windows_core::Interface::vtable(this).SetFullDescription)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn LocalizedControlTypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LocalizedControlTypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetLocalizedControlType<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetLocalizedControlType)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetLocalizedControlType<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics5(|this| unsafe { (::windows_core::Interface::vtable(this).SetLocalizedControlType)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn DescribedByProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DescribedByProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDescribedBy<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDescribedBy)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>(result__)
        })
    }
    pub fn FlowsToProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlowsToProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFlowsTo<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFlowsTo)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>(result__)
        })
    }
    pub fn FlowsFromProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlowsFromProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFlowsFrom<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVector<super::DependencyObject>> {
        Self::IAutomationPropertiesStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetFlowsFrom)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<super::DependencyObject>>(result__)
        })
    }
    pub fn CultureProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics6(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CultureProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetCulture<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<i32> {
        Self::IAutomationPropertiesStatics6(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).GetCulture)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    pub fn SetCulture<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: i32) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics6(|this| unsafe { (::windows_core::Interface::vtable(this).SetCulture)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn HeadingLevelProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics7(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HeadingLevelProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn GetHeadingLevel<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<Peers::AutomationHeadingLevel> {
        Self::IAutomationPropertiesStatics7(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Peers::AutomationHeadingLevel>::zeroed();
            (::windows_core::Interface::vtable(this).GetHeadingLevel)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<Peers::AutomationHeadingLevel>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn SetHeadingLevel<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: Peers::AutomationHeadingLevel) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics7(|this| unsafe { (::windows_core::Interface::vtable(this).SetHeadingLevel)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn IsDialogProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics8(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsDialogProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsDialog<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0) -> ::windows_core::Result<bool> {
        Self::IAutomationPropertiesStatics8(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsDialog)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsDialog<'a, Param0: ::windows_core::IntoParam<'a, super::DependencyObject>>(element: Param0, value: bool) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics8(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsDialog)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn AutomationControlTypeProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IAutomationPropertiesStatics9(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AutomationControlTypeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn GetAutomationControlType<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(element: Param0) -> ::windows_core::Result<Peers::AutomationControlType> {
        Self::IAutomationPropertiesStatics9(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Peers::AutomationControlType>::zeroed();
            (::windows_core::Interface::vtable(this).GetAutomationControlType)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<Peers::AutomationControlType>(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn SetAutomationControlType<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(element: Param0, value: Peers::AutomationControlType) -> ::windows_core::Result<()> {
        Self::IAutomationPropertiesStatics9(|this| unsafe { (::windows_core::Interface::vtable(this).SetAutomationControlType)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn IAutomationPropertiesStatics<R, F: FnOnce(&IAutomationPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics2<R, F: FnOnce(&IAutomationPropertiesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics3<R, F: FnOnce(&IAutomationPropertiesStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics4<R, F: FnOnce(&IAutomationPropertiesStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics5<R, F: FnOnce(&IAutomationPropertiesStatics5) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics5> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics6<R, F: FnOnce(&IAutomationPropertiesStatics6) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics6> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics7<R, F: FnOnce(&IAutomationPropertiesStatics7) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics7> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics8<R, F: FnOnce(&IAutomationPropertiesStatics8) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics8> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAutomationPropertiesStatics9<R, F: FnOnce(&IAutomationPropertiesStatics9) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AutomationProperties, IAutomationPropertiesStatics9> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AutomationProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationProperties {}
impl ::core::fmt::Debug for AutomationProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationProperties {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AutomationProperties;{68d7232c-e622-48e9-af0b-1ffa33cc5cba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AutomationProperties {
    type Vtable = IAutomationProperties_Vtbl;
    const IID: ::windows_core::GUID = <IAutomationProperties as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AutomationProperties {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AutomationProperties";
}
impl ::core::convert::From<AutomationProperties> for ::windows_core::IUnknown {
    fn from(value: AutomationProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationProperties> for ::windows_core::IUnknown {
    fn from(value: &AutomationProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AutomationProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AutomationProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationProperties> for ::windows_core::IInspectable {
    fn from(value: AutomationProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationProperties> for ::windows_core::IInspectable {
    fn from(value: &AutomationProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AutomationProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AutomationProperties {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutomationProperties {}
unsafe impl ::core::marker::Sync for AutomationProperties {}
#[repr(transparent)]
pub struct AutomationProperty(::windows_core::IUnknown);
impl AutomationProperty {}
impl ::core::clone::Clone for AutomationProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationProperty {}
impl ::core::fmt::Debug for AutomationProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationProperty {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.AutomationProperty;{b627195b-3227-4e16-9534-ddece30ddb46})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AutomationProperty {
    type Vtable = IAutomationProperty_Vtbl;
    const IID: ::windows_core::GUID = <IAutomationProperty as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AutomationProperty {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.AutomationProperty";
}
impl ::core::convert::From<AutomationProperty> for ::windows_core::IUnknown {
    fn from(value: AutomationProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationProperty> for ::windows_core::IUnknown {
    fn from(value: &AutomationProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AutomationProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AutomationProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AutomationProperty> for ::windows_core::IInspectable {
    fn from(value: AutomationProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationProperty> for ::windows_core::IInspectable {
    fn from(value: &AutomationProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AutomationProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AutomationProperty {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AutomationProperty {}
unsafe impl ::core::marker::Sync for AutomationProperty {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomationStyleId(pub i32);
impl AutomationStyleId {
    pub const Heading1: Self = Self(70001i32);
    pub const Heading2: Self = Self(70002i32);
    pub const Heading3: Self = Self(70003i32);
    pub const Heading4: Self = Self(70004i32);
    pub const Heading5: Self = Self(70005i32);
    pub const Heading6: Self = Self(70006i32);
    pub const Heading7: Self = Self(70007i32);
    pub const Heading8: Self = Self(70008i32);
    pub const Heading9: Self = Self(70009i32);
    pub const Title: Self = Self(70010i32);
    pub const Subtitle: Self = Self(70011i32);
    pub const Normal: Self = Self(70012i32);
    pub const Emphasis: Self = Self(70013i32);
    pub const Quote: Self = Self(70014i32);
    pub const BulletedList: Self = Self(70015i32);
}
impl ::core::marker::Copy for AutomationStyleId {}
impl ::core::clone::Clone for AutomationStyleId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationStyleId {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomationStyleId {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomationStyleId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationStyleId").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationStyleId {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationStyleId;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomationTextDecorationLineStyle(pub i32);
impl AutomationTextDecorationLineStyle {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const WordsOnly: Self = Self(2i32);
    pub const Double: Self = Self(3i32);
    pub const Dot: Self = Self(4i32);
    pub const Dash: Self = Self(5i32);
    pub const DashDot: Self = Self(6i32);
    pub const DashDotDot: Self = Self(7i32);
    pub const Wavy: Self = Self(8i32);
    pub const ThickSingle: Self = Self(9i32);
    pub const DoubleWavy: Self = Self(10i32);
    pub const ThickWavy: Self = Self(11i32);
    pub const LongDash: Self = Self(12i32);
    pub const ThickDash: Self = Self(13i32);
    pub const ThickDashDot: Self = Self(14i32);
    pub const ThickDashDotDot: Self = Self(15i32);
    pub const ThickDot: Self = Self(16i32);
    pub const ThickLongDash: Self = Self(17i32);
    pub const Other: Self = Self(18i32);
}
impl ::core::marker::Copy for AutomationTextDecorationLineStyle {}
impl ::core::clone::Clone for AutomationTextDecorationLineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationTextDecorationLineStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomationTextDecorationLineStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomationTextDecorationLineStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationTextDecorationLineStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationTextDecorationLineStyle {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationTextDecorationLineStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomationTextEditChangeType(pub i32);
impl AutomationTextEditChangeType {
    pub const None: Self = Self(0i32);
    pub const AutoCorrect: Self = Self(1i32);
    pub const Composition: Self = Self(2i32);
    pub const CompositionFinalized: Self = Self(3i32);
}
impl ::core::marker::Copy for AutomationTextEditChangeType {}
impl ::core::clone::Clone for AutomationTextEditChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomationTextEditChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomationTextEditChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomationTextEditChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationTextEditChangeType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AutomationTextEditChangeType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.AutomationTextEditChangeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DockPatternIdentifiers(::windows_core::IUnknown);
impl DockPatternIdentifiers {
    pub fn DockPositionProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDockPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DockPositionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IDockPatternIdentifiersStatics<R, F: FnOnce(&IDockPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DockPatternIdentifiers, IDockPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DockPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DockPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DockPatternIdentifiers {}
impl ::core::fmt::Debug for DockPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DockPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DockPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.DockPatternIdentifiers;{ccd7f4e6-e4f9-47ff-bde7-378b11f78e09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DockPatternIdentifiers {
    type Vtable = IDockPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IDockPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DockPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.DockPatternIdentifiers";
}
impl ::core::convert::From<DockPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: DockPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DockPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &DockPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DockPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DockPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DockPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: DockPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DockPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &DockPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DockPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DockPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DockPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DockPatternIdentifiers {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DockPosition(pub i32);
impl DockPosition {
    pub const Top: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Fill: Self = Self(4i32);
    pub const None: Self = Self(5i32);
}
impl ::core::marker::Copy for DockPosition {}
impl ::core::clone::Clone for DockPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DockPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DockPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for DockPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DockPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DockPosition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.DockPosition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DragPatternIdentifiers(::windows_core::IUnknown);
impl DragPatternIdentifiers {
    pub fn DropEffectProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DropEffectProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn DropEffectsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DropEffectsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn GrabbedItemsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GrabbedItemsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsGrabbedProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDragPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsGrabbedProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IDragPatternIdentifiersStatics<R, F: FnOnce(&IDragPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DragPatternIdentifiers, IDragPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DragPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DragPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragPatternIdentifiers {}
impl ::core::fmt::Debug for DragPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DragPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.DragPatternIdentifiers;{6266e985-4d07-4e80-82eb-8f96690a1a0c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DragPatternIdentifiers {
    type Vtable = IDragPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IDragPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DragPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.DragPatternIdentifiers";
}
impl ::core::convert::From<DragPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: DragPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &DragPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DragPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DragPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: DragPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &DragPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DragPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DragPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DragPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DragPatternIdentifiers {}
#[repr(transparent)]
pub struct DropTargetPatternIdentifiers(::windows_core::IUnknown);
impl DropTargetPatternIdentifiers {
    pub fn DropTargetEffectProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDropTargetPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DropTargetEffectProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn DropTargetEffectsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IDropTargetPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DropTargetEffectsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IDropTargetPatternIdentifiersStatics<R, F: FnOnce(&IDropTargetPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DropTargetPatternIdentifiers, IDropTargetPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DropTargetPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DropTargetPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DropTargetPatternIdentifiers {}
impl ::core::fmt::Debug for DropTargetPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DropTargetPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DropTargetPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers;{11865133-a6fe-4634-bd18-0ef612b7b208})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DropTargetPatternIdentifiers {
    type Vtable = IDropTargetPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IDropTargetPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DropTargetPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers";
}
impl ::core::convert::From<DropTargetPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: DropTargetPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DropTargetPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &DropTargetPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DropTargetPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DropTargetPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DropTargetPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: DropTargetPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DropTargetPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &DropTargetPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DropTargetPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DropTargetPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DropTargetPatternIdentifiers {}
unsafe impl ::core::marker::Sync for DropTargetPatternIdentifiers {}
#[repr(transparent)]
pub struct ExpandCollapsePatternIdentifiers(::windows_core::IUnknown);
impl ExpandCollapsePatternIdentifiers {
    pub fn ExpandCollapseStateProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IExpandCollapsePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExpandCollapseStateProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IExpandCollapsePatternIdentifiersStatics<R, F: FnOnce(&IExpandCollapsePatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ExpandCollapsePatternIdentifiers, IExpandCollapsePatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ExpandCollapsePatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExpandCollapsePatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExpandCollapsePatternIdentifiers {}
impl ::core::fmt::Debug for ExpandCollapsePatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpandCollapsePatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExpandCollapsePatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers;{b006bac0-751b-4d55-92cb-613ec1bdf5d0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ExpandCollapsePatternIdentifiers {
    type Vtable = IExpandCollapsePatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IExpandCollapsePatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ExpandCollapsePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers";
}
impl ::core::convert::From<ExpandCollapsePatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: ExpandCollapsePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpandCollapsePatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &ExpandCollapsePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ExpandCollapsePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ExpandCollapsePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExpandCollapsePatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: ExpandCollapsePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExpandCollapsePatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &ExpandCollapsePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ExpandCollapsePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ExpandCollapsePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExpandCollapsePatternIdentifiers {}
unsafe impl ::core::marker::Sync for ExpandCollapsePatternIdentifiers {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExpandCollapseState(pub i32);
impl ExpandCollapseState {
    pub const Collapsed: Self = Self(0i32);
    pub const Expanded: Self = Self(1i32);
    pub const PartiallyExpanded: Self = Self(2i32);
    pub const LeafNode: Self = Self(3i32);
}
impl ::core::marker::Copy for ExpandCollapseState {}
impl ::core::clone::Clone for ExpandCollapseState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExpandCollapseState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ExpandCollapseState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExpandCollapseState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExpandCollapseState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExpandCollapseState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.ExpandCollapseState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GridItemPatternIdentifiers(::windows_core::IUnknown);
impl GridItemPatternIdentifiers {
    pub fn ColumnProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ColumnProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ColumnSpanProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ColumnSpanProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ContainingGridProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContainingGridProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn RowProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RowProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn RowSpanProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RowSpanProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IGridItemPatternIdentifiersStatics<R, F: FnOnce(&IGridItemPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GridItemPatternIdentifiers, IGridItemPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GridItemPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GridItemPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GridItemPatternIdentifiers {}
impl ::core::fmt::Debug for GridItemPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GridItemPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GridItemPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.GridItemPatternIdentifiers;{757744f1-3285-4fb1-803b-2545bd431599})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GridItemPatternIdentifiers {
    type Vtable = IGridItemPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IGridItemPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GridItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.GridItemPatternIdentifiers";
}
impl ::core::convert::From<GridItemPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: GridItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GridItemPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &GridItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GridItemPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: GridItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GridItemPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &GridItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GridItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GridItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for GridItemPatternIdentifiers {}
#[repr(transparent)]
pub struct GridPatternIdentifiers(::windows_core::IUnknown);
impl GridPatternIdentifiers {
    pub fn ColumnCountProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ColumnCountProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn RowCountProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IGridPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RowCountProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IGridPatternIdentifiersStatics<R, F: FnOnce(&IGridPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GridPatternIdentifiers, IGridPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GridPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GridPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GridPatternIdentifiers {}
impl ::core::fmt::Debug for GridPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GridPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GridPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.GridPatternIdentifiers;{c902980f-96c5-450c-9044-7e52c24f9e94})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GridPatternIdentifiers {
    type Vtable = IGridPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IGridPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GridPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.GridPatternIdentifiers";
}
impl ::core::convert::From<GridPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: GridPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GridPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &GridPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GridPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GridPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GridPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: GridPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GridPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &GridPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GridPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GridPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GridPatternIdentifiers {}
unsafe impl ::core::marker::Sync for GridPatternIdentifiers {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnnotationPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAnnotationPatternIdentifiers {
    type Vtable = IAnnotationPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd475a0c1_48b2_4e40_a6cf_3dc4b638c0de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAnnotationPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAnnotationPatternIdentifiersStatics {
    type Vtable = IAnnotationPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0e3a35d_d167_46dc_95ab_330af61aebb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AnnotationTypeIdProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AnnotationTypeNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AuthorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DateTimeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationAnnotation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationAnnotation {
    type Vtable = IAutomationAnnotation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb3c30ca_03d8_4618_91bf_e4d84f4af318);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AnnotationType) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AnnotationType) -> ::windows_core::HRESULT,
    pub Element: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationAnnotationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationAnnotationFactory {
    type Vtable = IAutomationAnnotationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4906fa52_ddc0_4e69_b76b_019d928d822f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AnnotationType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithElementParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AnnotationType, element: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationAnnotationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationAnnotationStatics {
    type Vtable = IAutomationAnnotationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe503eab7_4ee5_48cb_b5b8_bbcd46c9d1da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationAnnotationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ElementProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElementIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationElementIdentifiers {
    type Vtable = IAutomationElementIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe68a63cf_4345_4e2d_8a6a_49cce1fa2dcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationElementIdentifiersStatics {
    type Vtable = IAutomationElementIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4549399f_8340_4d67_b9bf_8c2ac6a0773a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AcceleratorKeyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AutomationIdProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BoundingRectangleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClassNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClickablePointProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ControlTypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HasKeyboardFocusProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HelpTextProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsContentElementProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsControlElementProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsKeyboardFocusableProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsOffscreenProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsPasswordProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsRequiredForFormProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ItemStatusProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ItemTypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LabeledByProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LocalizedControlTypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LiveSettingProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationElementIdentifiersStatics2 {
    type Vtable = IAutomationElementIdentifiersStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5cbb1e2_d55f_46a9_9eda_1a4742515dc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ControlledPeersProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationElementIdentifiersStatics3 {
    type Vtable = IAutomationElementIdentifiersStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f5cbebd_b3eb_4083_adc7_0c2f39bb3543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PositionInSetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SizeOfSetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LevelProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AnnotationsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationElementIdentifiersStatics4 {
    type Vtable = IAutomationElementIdentifiersStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5af51f75_5913_4d78_b330_a6f50b73ed9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LandmarkTypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LocalizedLandmarkTypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationElementIdentifiersStatics5 {
    type Vtable = IAutomationElementIdentifiersStatics5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x986a8206_de59_42f9_a1e7_62b8af9e756d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsPeripheralProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsDataValidForFormProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FullDescriptionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DescribedByProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FlowsToProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FlowsFromProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationElementIdentifiersStatics6 {
    type Vtable = IAutomationElementIdentifiersStatics6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde52b00d_8328_4eae_8035_f8db99c8bac4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CultureProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationElementIdentifiersStatics7 {
    type Vtable = IAutomationElementIdentifiersStatics7_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00f1abb2_742c_446a_a8f6_1672b10d2874);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics7_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HeadingLevelProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElementIdentifiersStatics8(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationElementIdentifiersStatics8 {
    type Vtable = IAutomationElementIdentifiersStatics8_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8517b060_806c_5dc5_bc41_891bb5a47adf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElementIdentifiersStatics8_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsDialogProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationProperties {
    type Vtable = IAutomationProperties_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68d7232c_e622_48e9_af0b_1ffa33cc5cba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProperties_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationPropertiesStatics {
    type Vtable = IAutomationPropertiesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb618fd7b_32d0_4970_9c42_7c039ac7be78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AcceleratorKeyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAcceleratorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAcceleratorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AccessKeyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAccessKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AutomationIdProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAutomationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAutomationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HelpTextProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetHelpText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetHelpText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsRequiredForFormProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsRequiredForForm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsRequiredForForm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: bool) -> ::windows_core::HRESULT,
    pub ItemStatusProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetItemStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetItemStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ItemTypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetItemType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetItemType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LabeledByProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLabeledBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetLabeledBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LiveSettingProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub GetLiveSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut Peers::AutomationLiveSetting) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    GetLiveSetting: usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub SetLiveSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: Peers::AutomationLiveSetting) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    SetLiveSetting: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationPropertiesStatics2 {
    type Vtable = IAutomationPropertiesStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3976547f_7089_4801_8f1d_aab78090d1a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AccessibilityViewProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub GetAccessibilityView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut Peers::AccessibilityView) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    GetAccessibilityView: usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub SetAccessibilityView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: Peers::AccessibilityView) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    SetAccessibilityView: usize,
    pub ControlledPeersProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControlledPeers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControlledPeers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationPropertiesStatics3 {
    type Vtable = IAutomationPropertiesStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b75d735_5cb1_42ad_9b57_5faba8c1867f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PositionInSetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetPositionInSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetPositionInSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: i32) -> ::windows_core::HRESULT,
    pub SizeOfSetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSizeOfSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetSizeOfSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: i32) -> ::windows_core::HRESULT,
    pub LevelProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: i32) -> ::windows_core::HRESULT,
    pub AnnotationsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAnnotations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAnnotations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationPropertiesStatics4 {
    type Vtable = IAutomationPropertiesStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7d62655_311a_4b7c_a131_524e89cd3cf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LandmarkTypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub GetLandmarkType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut Peers::AutomationLandmarkType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    GetLandmarkType: usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub SetLandmarkType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: Peers::AutomationLandmarkType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    SetLandmarkType: usize,
    pub LocalizedLandmarkTypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLocalizedLandmarkType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLocalizedLandmarkType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationPropertiesStatics5 {
    type Vtable = IAutomationPropertiesStatics5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0be35b26_c8f9_41a2_b4db_e6a7a32b0c34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsPeripheralProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsPeripheral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsPeripheral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: bool) -> ::windows_core::HRESULT,
    pub IsDataValidForFormProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsDataValidForForm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDataValidForForm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: bool) -> ::windows_core::HRESULT,
    pub FullDescriptionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetFullDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetFullDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LocalizedControlTypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLocalizedControlType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLocalizedControlType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DescribedByProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDescribedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDescribedBy: usize,
    pub FlowsToProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFlowsTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFlowsTo: usize,
    pub FlowsFromProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFlowsFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFlowsFrom: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationPropertiesStatics6 {
    type Vtable = IAutomationPropertiesStatics6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc61e030f_eb49_4e5d_b012_4c1c96c3901b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CultureProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetCulture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetCulture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationPropertiesStatics7 {
    type Vtable = IAutomationPropertiesStatics7_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf7e98bf3_8f91_4068_a4ad_b7b402d10a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics7_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HeadingLevelProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub GetHeadingLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut Peers::AutomationHeadingLevel) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    GetHeadingLevel: usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub SetHeadingLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: Peers::AutomationHeadingLevel) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    SetHeadingLevel: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics8(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationPropertiesStatics8 {
    type Vtable = IAutomationPropertiesStatics8_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x432eca20_171a_560d_8524_3e651d3ad6ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics8_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsDialogProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationPropertiesStatics9(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationPropertiesStatics9 {
    type Vtable = IAutomationPropertiesStatics9_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f20b1d1_87b2_5562_8077_da593edafd2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationPropertiesStatics9_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AutomationControlTypeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub GetAutomationControlType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut Peers::AutomationControlType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    GetAutomationControlType: usize,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub SetAutomationControlType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: Peers::AutomationControlType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    SetAutomationControlType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationProperty(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationProperty {
    type Vtable = IAutomationProperty_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb627195b_3227_4e16_9534_ddece30ddb46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationProperty_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDockPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDockPatternIdentifiers {
    type Vtable = IDockPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccd7f4e6_e4f9_47ff_bde7_378b11f78e09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDockPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDockPatternIdentifiersStatics {
    type Vtable = IDockPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b87245c_ed80_4fe5_8eb4_708a39c841e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DockPositionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragPatternIdentifiers {
    type Vtable = IDragPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6266e985_4d07_4e80_82eb_8f96690a1a0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragPatternIdentifiersStatics {
    type Vtable = IDragPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a05379d_1755_4082_9d90_46f1411d7986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DropEffectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DropEffectsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GrabbedItemsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsGrabbedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDropTargetPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDropTargetPatternIdentifiers {
    type Vtable = IDropTargetPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11865133_a6fe_4634_bd18_0ef612b7b208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDropTargetPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDropTargetPatternIdentifiersStatics {
    type Vtable = IDropTargetPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b693304_89fb_4b0a_9452_ca2c66aaf9f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DropTargetEffectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DropTargetEffectsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpandCollapsePatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExpandCollapsePatternIdentifiers {
    type Vtable = IExpandCollapsePatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb006bac0_751b_4d55_92cb_613ec1bdf5d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExpandCollapsePatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExpandCollapsePatternIdentifiersStatics {
    type Vtable = IExpandCollapsePatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7816fd4_6ee0_4f38_8e14_56ef21adacfd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapsePatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExpandCollapseStateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGridItemPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridItemPatternIdentifiers {
    type Vtable = IGridItemPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x757744f1_3285_4fb1_803b_2545bd431599);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGridItemPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridItemPatternIdentifiersStatics {
    type Vtable = IGridItemPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x217d2402_5e46_4d61_8794_b8ee8e774714);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ColumnProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ColumnSpanProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContainingGridProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RowProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RowSpanProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGridPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridPatternIdentifiers {
    type Vtable = IGridPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc902980f_96c5_450c_9044_7e52c24f9e94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGridPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridPatternIdentifiersStatics {
    type Vtable = IGridPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7bc452f3_a181_4137_8de9_1f9b1a8320ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ColumnCountProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RowCountProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMultipleViewPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMultipleViewPatternIdentifiers {
    type Vtable = IMultipleViewPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d5cd3b8_1e12_488b_b0ea_5e6cb89816e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMultipleViewPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMultipleViewPatternIdentifiersStatics {
    type Vtable = IMultipleViewPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9cfa66f_6b84_4d71_9e48_d764d3bcda8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CurrentViewProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SupportedViewsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRangeValuePatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeValuePatternIdentifiers {
    type Vtable = IRangeValuePatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8760f45_33c9_467d_bc9e_d1515263ace1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRangeValuePatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeValuePatternIdentifiersStatics {
    type Vtable = IRangeValuePatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce23450f_1c27_457f_b815_7a5e46863dbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValuePatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsReadOnlyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LargeChangeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaximumProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinimumProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SmallChangeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ValueProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScrollPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScrollPatternIdentifiers {
    type Vtable = IScrollPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x366b1003_425c_4951_ae83_d521e73bc696);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScrollPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScrollPatternIdentifiersStatics {
    type Vtable = IScrollPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4bf8e0a1_fb7f_4fa4_83b3_cfaeb103a685);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HorizontallyScrollableProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HorizontalScrollPercentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HorizontalViewSizeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NoScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub VerticallyScrollableProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VerticalScrollPercentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VerticalViewSizeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectionItemPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectionItemPatternIdentifiers {
    type Vtable = ISelectionItemPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2dafa41a_3ef8_4bb5_a02b_3ee1b2274740);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectionItemPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectionItemPatternIdentifiersStatics {
    type Vtable = ISelectionItemPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa918d163_487e_4e3e_9f86_7b44acbe27ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSelectedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectionContainerProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectionPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectionPatternIdentifiers {
    type Vtable = ISelectionPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4aa66fb0_e3f7_475f_b78d_f8a83bb730c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectionPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectionPatternIdentifiersStatics {
    type Vtable = ISelectionPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93035b4c_6b50_40a1_b23f_5c78ddbd479a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanSelectMultipleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsSelectionRequiredProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpreadsheetItemPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpreadsheetItemPatternIdentifiers {
    type Vtable = ISpreadsheetItemPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84347e19_ca4b_46a2_a794_c87928a3b1ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpreadsheetItemPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpreadsheetItemPatternIdentifiersStatics {
    type Vtable = ISpreadsheetItemPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43658779_5380_4f12_b468_b4f368ad4499);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FormulaProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStylesPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStylesPatternIdentifiers {
    type Vtable = IStylesPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0e4e201_e89d_436b_8287_4f7903466879);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStylesPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStylesPatternIdentifiersStatics {
    type Vtable = IStylesPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x528a457a_bc3c_4d48_94af_1f68703ca296);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedPropertiesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FillColorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FillPatternColorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FillPatternStyleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShapeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StyleIdProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StyleNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITableItemPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITableItemPatternIdentifiers {
    type Vtable = ITableItemPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc326e5ad_8077_4c64_98e4_e83bcf1b4389);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITableItemPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITableItemPatternIdentifiersStatics {
    type Vtable = ITableItemPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24c4b923_e9a2_4de9_b2a4_a8b22d0be362);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ColumnHeaderItemsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RowHeaderItemsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITablePatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITablePatternIdentifiers {
    type Vtable = ITablePatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38d104fe_0d0c_412a_bf8d_51ede683baf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITablePatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITablePatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITablePatternIdentifiersStatics {
    type Vtable = ITablePatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75073d25_32c9_4903_aecf_dc3504cbd244);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITablePatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ColumnHeadersProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RowHeadersProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RowOrColumnMajorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITogglePatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITogglePatternIdentifiers {
    type Vtable = ITogglePatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e191f6b_34d4_4ae7_83ac_29f88882d985);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITogglePatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITogglePatternIdentifiersStatics {
    type Vtable = ITogglePatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7f75544_14a5_4f2f_92fc_760524de06ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITogglePatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ToggleStateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransformPattern2Identifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformPattern2Identifiers {
    type Vtable = ITransformPattern2Identifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08aaa03d_dea7_402f_8097_9a2783d60e5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPattern2Identifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransformPattern2IdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformPattern2IdentifiersStatics {
    type Vtable = ITransformPattern2IdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78963644_11f0_467c_a72b_5dac41c1f6fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPattern2IdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanZoomProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ZoomLevelProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaxZoomProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinZoomProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransformPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformPatternIdentifiers {
    type Vtable = ITransformPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4115b8c_c3c8_4a37_b994_2709a7811665);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransformPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransformPatternIdentifiersStatics {
    type Vtable = ITransformPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4570edab_d705_40c4_a1dc_e9acfcef85f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanMoveProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CanResizeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CanRotateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IValuePatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IValuePatternIdentifiers {
    type Vtable = IValuePatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x425bf64c_5333_4e41_b470_2bad14ecd085);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValuePatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IValuePatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IValuePatternIdentifiersStatics {
    type Vtable = IValuePatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc247e8f7_adcc_440f_b123_33788a40525a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValuePatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsReadOnlyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ValueProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowPatternIdentifiers(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowPatternIdentifiers {
    type Vtable = IWindowPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39f78bb4_7032_41e2_b79e_27b74a8628de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiers_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowPatternIdentifiersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWindowPatternIdentifiersStatics {
    type Vtable = IWindowPatternIdentifiersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07d0ad06_6302_4d29_878b_19da03fc228d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowPatternIdentifiersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanMaximizeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CanMinimizeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsModalProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsTopmostProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WindowInteractionStateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub WindowVisualStateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct MultipleViewPatternIdentifiers(::windows_core::IUnknown);
impl MultipleViewPatternIdentifiers {
    pub fn CurrentViewProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IMultipleViewPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentViewProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn SupportedViewsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IMultipleViewPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedViewsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IMultipleViewPatternIdentifiersStatics<R, F: FnOnce(&IMultipleViewPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MultipleViewPatternIdentifiers, IMultipleViewPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MultipleViewPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MultipleViewPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MultipleViewPatternIdentifiers {}
impl ::core::fmt::Debug for MultipleViewPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MultipleViewPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MultipleViewPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers;{5d5cd3b8-1e12-488b-b0ea-5e6cb89816e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MultipleViewPatternIdentifiers {
    type Vtable = IMultipleViewPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IMultipleViewPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MultipleViewPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers";
}
impl ::core::convert::From<MultipleViewPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: MultipleViewPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultipleViewPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &MultipleViewPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MultipleViewPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MultipleViewPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MultipleViewPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: MultipleViewPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MultipleViewPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &MultipleViewPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MultipleViewPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MultipleViewPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MultipleViewPatternIdentifiers {}
unsafe impl ::core::marker::Sync for MultipleViewPatternIdentifiers {}
#[repr(transparent)]
pub struct RangeValuePatternIdentifiers(::windows_core::IUnknown);
impl RangeValuePatternIdentifiers {
    pub fn IsReadOnlyProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnlyProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn LargeChangeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LargeChangeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn MaximumProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaximumProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn MinimumProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn SmallChangeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SmallChangeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ValueProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IRangeValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ValueProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IRangeValuePatternIdentifiersStatics<R, F: FnOnce(&IRangeValuePatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RangeValuePatternIdentifiers, IRangeValuePatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RangeValuePatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RangeValuePatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RangeValuePatternIdentifiers {}
impl ::core::fmt::Debug for RangeValuePatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RangeValuePatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RangeValuePatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers;{f8760f45-33c9-467d-bc9e-d1515263ace1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RangeValuePatternIdentifiers {
    type Vtable = IRangeValuePatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IRangeValuePatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RangeValuePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers";
}
impl ::core::convert::From<RangeValuePatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: RangeValuePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RangeValuePatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &RangeValuePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RangeValuePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RangeValuePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RangeValuePatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: RangeValuePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RangeValuePatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &RangeValuePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RangeValuePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RangeValuePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RangeValuePatternIdentifiers {}
unsafe impl ::core::marker::Sync for RangeValuePatternIdentifiers {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RowOrColumnMajor(pub i32);
impl RowOrColumnMajor {
    pub const RowMajor: Self = Self(0i32);
    pub const ColumnMajor: Self = Self(1i32);
    pub const Indeterminate: Self = Self(2i32);
}
impl ::core::marker::Copy for RowOrColumnMajor {}
impl ::core::clone::Clone for RowOrColumnMajor {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RowOrColumnMajor {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RowOrColumnMajor {
    type Abi = Self;
}
impl ::core::fmt::Debug for RowOrColumnMajor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RowOrColumnMajor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RowOrColumnMajor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.RowOrColumnMajor;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ScrollAmount(pub i32);
impl ScrollAmount {
    pub const LargeDecrement: Self = Self(0i32);
    pub const SmallDecrement: Self = Self(1i32);
    pub const NoAmount: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const SmallIncrement: Self = Self(4i32);
}
impl ::core::marker::Copy for ScrollAmount {}
impl ::core::clone::Clone for ScrollAmount {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ScrollAmount {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ScrollAmount {
    type Abi = Self;
}
impl ::core::fmt::Debug for ScrollAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollAmount").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScrollAmount {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.ScrollAmount;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ScrollPatternIdentifiers(::windows_core::IUnknown);
impl ScrollPatternIdentifiers {
    pub fn HorizontallyScrollableProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontallyScrollableProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn HorizontalScrollPercentProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalScrollPercentProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn HorizontalViewSizeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalViewSizeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn NoScroll() -> ::windows_core::Result<f64> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).NoScroll)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        })
    }
    pub fn VerticallyScrollableProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VerticallyScrollableProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn VerticalScrollPercentProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalScrollPercentProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn VerticalViewSizeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IScrollPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalViewSizeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IScrollPatternIdentifiersStatics<R, F: FnOnce(&IScrollPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ScrollPatternIdentifiers, IScrollPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ScrollPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScrollPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScrollPatternIdentifiers {}
impl ::core::fmt::Debug for ScrollPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScrollPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.ScrollPatternIdentifiers;{366b1003-425c-4951-ae83-d521e73bc696})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ScrollPatternIdentifiers {
    type Vtable = IScrollPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IScrollPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ScrollPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ScrollPatternIdentifiers";
}
impl ::core::convert::From<ScrollPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: ScrollPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScrollPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &ScrollPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScrollPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: ScrollPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScrollPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &ScrollPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ScrollPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ScrollPatternIdentifiers {}
unsafe impl ::core::marker::Sync for ScrollPatternIdentifiers {}
#[repr(transparent)]
pub struct SelectionItemPatternIdentifiers(::windows_core::IUnknown);
impl SelectionItemPatternIdentifiers {
    pub fn IsSelectedProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ISelectionItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelectedProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn SelectionContainerProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ISelectionItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionContainerProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ISelectionItemPatternIdentifiersStatics<R, F: FnOnce(&ISelectionItemPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SelectionItemPatternIdentifiers, ISelectionItemPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SelectionItemPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SelectionItemPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SelectionItemPatternIdentifiers {}
impl ::core::fmt::Debug for SelectionItemPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectionItemPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SelectionItemPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers;{2dafa41a-3ef8-4bb5-a02b-3ee1b2274740})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SelectionItemPatternIdentifiers {
    type Vtable = ISelectionItemPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <ISelectionItemPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SelectionItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers";
}
impl ::core::convert::From<SelectionItemPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: SelectionItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectionItemPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &SelectionItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SelectionItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SelectionItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SelectionItemPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: SelectionItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectionItemPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &SelectionItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SelectionItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SelectionItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SelectionItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SelectionItemPatternIdentifiers {}
#[repr(transparent)]
pub struct SelectionPatternIdentifiers(::windows_core::IUnknown);
impl SelectionPatternIdentifiers {
    pub fn CanSelectMultipleProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CanSelectMultipleProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsSelectionRequiredProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelectionRequiredProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn SelectionProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ISelectionPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ISelectionPatternIdentifiersStatics<R, F: FnOnce(&ISelectionPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SelectionPatternIdentifiers, ISelectionPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SelectionPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SelectionPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SelectionPatternIdentifiers {}
impl ::core::fmt::Debug for SelectionPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectionPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SelectionPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.SelectionPatternIdentifiers;{4aa66fb0-e3f7-475f-b78d-f8a83bb730c4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SelectionPatternIdentifiers {
    type Vtable = ISelectionPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <ISelectionPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SelectionPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.SelectionPatternIdentifiers";
}
impl ::core::convert::From<SelectionPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: SelectionPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectionPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &SelectionPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SelectionPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: SelectionPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectionPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &SelectionPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SelectionPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SelectionPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SelectionPatternIdentifiers {}
#[repr(transparent)]
pub struct SpreadsheetItemPatternIdentifiers(::windows_core::IUnknown);
impl SpreadsheetItemPatternIdentifiers {
    pub fn FormulaProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ISpreadsheetItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FormulaProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ISpreadsheetItemPatternIdentifiersStatics<R, F: FnOnce(&ISpreadsheetItemPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SpreadsheetItemPatternIdentifiers, ISpreadsheetItemPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SpreadsheetItemPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpreadsheetItemPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpreadsheetItemPatternIdentifiers {}
impl ::core::fmt::Debug for SpreadsheetItemPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpreadsheetItemPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SpreadsheetItemPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers;{84347e19-ca4b-46a2-a794-c87928a3b1ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SpreadsheetItemPatternIdentifiers {
    type Vtable = ISpreadsheetItemPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <ISpreadsheetItemPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SpreadsheetItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers";
}
impl ::core::convert::From<SpreadsheetItemPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: SpreadsheetItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpreadsheetItemPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &SpreadsheetItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SpreadsheetItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SpreadsheetItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpreadsheetItemPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: SpreadsheetItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpreadsheetItemPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &SpreadsheetItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SpreadsheetItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SpreadsheetItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpreadsheetItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for SpreadsheetItemPatternIdentifiers {}
#[repr(transparent)]
pub struct StylesPatternIdentifiers(::windows_core::IUnknown);
impl StylesPatternIdentifiers {
    pub fn ExtendedPropertiesProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedPropertiesProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn FillColorProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FillColorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn FillPatternColorProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FillPatternColorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn FillPatternStyleProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FillPatternStyleProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ShapeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShapeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn StyleIdProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StyleIdProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn StyleNameProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IStylesPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StyleNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IStylesPatternIdentifiersStatics<R, F: FnOnce(&IStylesPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<StylesPatternIdentifiers, IStylesPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StylesPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StylesPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StylesPatternIdentifiers {}
impl ::core::fmt::Debug for StylesPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StylesPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for StylesPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.StylesPatternIdentifiers;{b0e4e201-e89d-436b-8287-4f7903466879})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for StylesPatternIdentifiers {
    type Vtable = IStylesPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IStylesPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for StylesPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.StylesPatternIdentifiers";
}
impl ::core::convert::From<StylesPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: StylesPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StylesPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &StylesPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for StylesPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a StylesPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StylesPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: StylesPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StylesPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &StylesPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for StylesPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a StylesPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StylesPatternIdentifiers {}
unsafe impl ::core::marker::Sync for StylesPatternIdentifiers {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SupportedTextSelection(pub i32);
impl SupportedTextSelection {
    pub const None: Self = Self(0i32);
    pub const Single: Self = Self(1i32);
    pub const Multiple: Self = Self(2i32);
}
impl ::core::marker::Copy for SupportedTextSelection {}
impl ::core::clone::Clone for SupportedTextSelection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SupportedTextSelection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SupportedTextSelection {
    type Abi = Self;
}
impl ::core::fmt::Debug for SupportedTextSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SupportedTextSelection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SupportedTextSelection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.SupportedTextSelection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SynchronizedInputType(pub i32);
impl SynchronizedInputType {
    pub const KeyUp: Self = Self(1i32);
    pub const KeyDown: Self = Self(2i32);
    pub const LeftMouseUp: Self = Self(4i32);
    pub const LeftMouseDown: Self = Self(8i32);
    pub const RightMouseUp: Self = Self(16i32);
    pub const RightMouseDown: Self = Self(32i32);
}
impl ::core::marker::Copy for SynchronizedInputType {}
impl ::core::clone::Clone for SynchronizedInputType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SynchronizedInputType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SynchronizedInputType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SynchronizedInputType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SynchronizedInputType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SynchronizedInputType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.SynchronizedInputType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct TableItemPatternIdentifiers(::windows_core::IUnknown);
impl TableItemPatternIdentifiers {
    pub fn ColumnHeaderItemsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITableItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ColumnHeaderItemsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn RowHeaderItemsProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITableItemPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RowHeaderItemsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITableItemPatternIdentifiersStatics<R, F: FnOnce(&ITableItemPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TableItemPatternIdentifiers, ITableItemPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TableItemPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TableItemPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TableItemPatternIdentifiers {}
impl ::core::fmt::Debug for TableItemPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TableItemPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TableItemPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TableItemPatternIdentifiers;{c326e5ad-8077-4c64-98e4-e83bcf1b4389})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TableItemPatternIdentifiers {
    type Vtable = ITableItemPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <ITableItemPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TableItemPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TableItemPatternIdentifiers";
}
impl ::core::convert::From<TableItemPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: TableItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TableItemPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &TableItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TableItemPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: TableItemPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TableItemPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &TableItemPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TableItemPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TableItemPatternIdentifiers {}
unsafe impl ::core::marker::Sync for TableItemPatternIdentifiers {}
#[repr(transparent)]
pub struct TablePatternIdentifiers(::windows_core::IUnknown);
impl TablePatternIdentifiers {
    pub fn ColumnHeadersProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ColumnHeadersProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn RowHeadersProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RowHeadersProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn RowOrColumnMajorProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITablePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RowOrColumnMajorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITablePatternIdentifiersStatics<R, F: FnOnce(&ITablePatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TablePatternIdentifiers, ITablePatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TablePatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TablePatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TablePatternIdentifiers {}
impl ::core::fmt::Debug for TablePatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TablePatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TablePatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TablePatternIdentifiers;{38d104fe-0d0c-412a-bf8d-51ede683baf5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TablePatternIdentifiers {
    type Vtable = ITablePatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <ITablePatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TablePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TablePatternIdentifiers";
}
impl ::core::convert::From<TablePatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: TablePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TablePatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &TablePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TablePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TablePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TablePatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: TablePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TablePatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &TablePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TablePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TablePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TablePatternIdentifiers {}
unsafe impl ::core::marker::Sync for TablePatternIdentifiers {}
#[repr(transparent)]
pub struct TogglePatternIdentifiers(::windows_core::IUnknown);
impl TogglePatternIdentifiers {
    pub fn ToggleStateProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITogglePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ToggleStateProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITogglePatternIdentifiersStatics<R, F: FnOnce(&ITogglePatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TogglePatternIdentifiers, ITogglePatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TogglePatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TogglePatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TogglePatternIdentifiers {}
impl ::core::fmt::Debug for TogglePatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TogglePatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TogglePatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TogglePatternIdentifiers;{7e191f6b-34d4-4ae7-83ac-29f88882d985})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TogglePatternIdentifiers {
    type Vtable = ITogglePatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <ITogglePatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TogglePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TogglePatternIdentifiers";
}
impl ::core::convert::From<TogglePatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: TogglePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TogglePatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &TogglePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TogglePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TogglePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TogglePatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: TogglePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TogglePatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &TogglePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TogglePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TogglePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TogglePatternIdentifiers {}
unsafe impl ::core::marker::Sync for TogglePatternIdentifiers {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ToggleState(pub i32);
impl ToggleState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Indeterminate: Self = Self(2i32);
}
impl ::core::marker::Copy for ToggleState {}
impl ::core::clone::Clone for ToggleState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ToggleState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ToggleState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ToggleState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToggleState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToggleState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.ToggleState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct TransformPattern2Identifiers(::windows_core::IUnknown);
impl TransformPattern2Identifiers {
    pub fn CanZoomProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CanZoomProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ZoomLevelProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ZoomLevelProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn MaxZoomProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxZoomProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn MinZoomProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPattern2IdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MinZoomProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITransformPattern2IdentifiersStatics<R, F: FnOnce(&ITransformPattern2IdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TransformPattern2Identifiers, ITransformPattern2IdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TransformPattern2Identifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TransformPattern2Identifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TransformPattern2Identifiers {}
impl ::core::fmt::Debug for TransformPattern2Identifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransformPattern2Identifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TransformPattern2Identifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TransformPattern2Identifiers;{08aaa03d-dea7-402f-8097-9a2783d60e5d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TransformPattern2Identifiers {
    type Vtable = ITransformPattern2Identifiers_Vtbl;
    const IID: ::windows_core::GUID = <ITransformPattern2Identifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TransformPattern2Identifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TransformPattern2Identifiers";
}
impl ::core::convert::From<TransformPattern2Identifiers> for ::windows_core::IUnknown {
    fn from(value: TransformPattern2Identifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TransformPattern2Identifiers> for ::windows_core::IUnknown {
    fn from(value: &TransformPattern2Identifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TransformPattern2Identifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TransformPattern2Identifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TransformPattern2Identifiers> for ::windows_core::IInspectable {
    fn from(value: TransformPattern2Identifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TransformPattern2Identifiers> for ::windows_core::IInspectable {
    fn from(value: &TransformPattern2Identifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TransformPattern2Identifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TransformPattern2Identifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TransformPattern2Identifiers {}
unsafe impl ::core::marker::Sync for TransformPattern2Identifiers {}
#[repr(transparent)]
pub struct TransformPatternIdentifiers(::windows_core::IUnknown);
impl TransformPatternIdentifiers {
    pub fn CanMoveProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CanMoveProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn CanResizeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CanResizeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn CanRotateProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::ITransformPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CanRotateProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ITransformPatternIdentifiersStatics<R, F: FnOnce(&ITransformPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TransformPatternIdentifiers, ITransformPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TransformPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TransformPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TransformPatternIdentifiers {}
impl ::core::fmt::Debug for TransformPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransformPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TransformPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.TransformPatternIdentifiers;{e4115b8c-c3c8-4a37-b994-2709a7811665})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TransformPatternIdentifiers {
    type Vtable = ITransformPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <ITransformPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TransformPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.TransformPatternIdentifiers";
}
impl ::core::convert::From<TransformPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: TransformPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TransformPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &TransformPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TransformPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TransformPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TransformPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: TransformPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TransformPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &TransformPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TransformPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TransformPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TransformPatternIdentifiers {}
unsafe impl ::core::marker::Sync for TransformPatternIdentifiers {}
#[repr(transparent)]
pub struct ValuePatternIdentifiers(::windows_core::IUnknown);
impl ValuePatternIdentifiers {
    pub fn IsReadOnlyProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnlyProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn ValueProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IValuePatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ValueProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IValuePatternIdentifiersStatics<R, F: FnOnce(&IValuePatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ValuePatternIdentifiers, IValuePatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ValuePatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ValuePatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ValuePatternIdentifiers {}
impl ::core::fmt::Debug for ValuePatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ValuePatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ValuePatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.ValuePatternIdentifiers;{425bf64c-5333-4e41-b470-2bad14ecd085})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ValuePatternIdentifiers {
    type Vtable = IValuePatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IValuePatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ValuePatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.ValuePatternIdentifiers";
}
impl ::core::convert::From<ValuePatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: ValuePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ValuePatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &ValuePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ValuePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ValuePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ValuePatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: ValuePatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ValuePatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &ValuePatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ValuePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ValuePatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ValuePatternIdentifiers {}
unsafe impl ::core::marker::Sync for ValuePatternIdentifiers {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WindowInteractionState(pub i32);
impl WindowInteractionState {
    pub const Running: Self = Self(0i32);
    pub const Closing: Self = Self(1i32);
    pub const ReadyForUserInteraction: Self = Self(2i32);
    pub const BlockedByModalWindow: Self = Self(3i32);
    pub const NotResponding: Self = Self(4i32);
}
impl ::core::marker::Copy for WindowInteractionState {}
impl ::core::clone::Clone for WindowInteractionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowInteractionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WindowInteractionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WindowInteractionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowInteractionState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WindowInteractionState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.WindowInteractionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WindowPatternIdentifiers(::windows_core::IUnknown);
impl WindowPatternIdentifiers {
    pub fn CanMaximizeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CanMaximizeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn CanMinimizeProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CanMinimizeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsModalProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsModalProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IsTopmostProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsTopmostProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn WindowInteractionStateProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WindowInteractionStateProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn WindowVisualStateProperty() -> ::windows_core::Result<AutomationProperty> {
        Self::IWindowPatternIdentifiersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WindowVisualStateProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationProperty>(result__)
        })
    }
    pub fn IWindowPatternIdentifiersStatics<R, F: FnOnce(&IWindowPatternIdentifiersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WindowPatternIdentifiers, IWindowPatternIdentifiersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WindowPatternIdentifiers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowPatternIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowPatternIdentifiers {}
impl ::core::fmt::Debug for WindowPatternIdentifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowPatternIdentifiers").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WindowPatternIdentifiers {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.WindowPatternIdentifiers;{39f78bb4-7032-41e2-b79e-27b74a8628de})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WindowPatternIdentifiers {
    type Vtable = IWindowPatternIdentifiers_Vtbl;
    const IID: ::windows_core::GUID = <IWindowPatternIdentifiers as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WindowPatternIdentifiers {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.WindowPatternIdentifiers";
}
impl ::core::convert::From<WindowPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: WindowPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowPatternIdentifiers> for ::windows_core::IUnknown {
    fn from(value: &WindowPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WindowPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WindowPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: WindowPatternIdentifiers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowPatternIdentifiers> for ::windows_core::IInspectable {
    fn from(value: &WindowPatternIdentifiers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WindowPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WindowPatternIdentifiers {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WindowPatternIdentifiers {}
unsafe impl ::core::marker::Sync for WindowPatternIdentifiers {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WindowVisualState(pub i32);
impl WindowVisualState {
    pub const Normal: Self = Self(0i32);
    pub const Maximized: Self = Self(1i32);
    pub const Minimized: Self = Self(2i32);
}
impl ::core::marker::Copy for WindowVisualState {}
impl ::core::clone::Clone for WindowVisualState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WindowVisualState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WindowVisualState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WindowVisualState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowVisualState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WindowVisualState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.WindowVisualState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ZoomUnit(pub i32);
impl ZoomUnit {
    pub const NoAmount: Self = Self(0i32);
    pub const LargeDecrement: Self = Self(1i32);
    pub const SmallDecrement: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const SmallIncrement: Self = Self(4i32);
}
impl ::core::marker::Copy for ZoomUnit {}
impl ::core::clone::Clone for ZoomUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ZoomUnit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ZoomUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for ZoomUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ZoomUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ZoomUnit {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.ZoomUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}