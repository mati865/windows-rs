use super::*;

#[repr(transparent)]
pub struct VARIANT(imp::VARIANT);

#[repr(transparent)]
pub struct PROPVARIANT(imp::PROPVARIANT);

impl Default for VARIANT {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PROPVARIANT {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for VARIANT {
    fn clone(&self) -> Self {
        unsafe {
            let mut value = Self::new();
            imp::VariantCopy(&mut value.0, &self.0);
            value
        }
    }
}

impl Clone for PROPVARIANT {
    fn clone(&self) -> Self {
        unsafe {
            let mut value = Self::new();
            imp::PropVariantCopy(&mut value.0, &self.0);
            value
        }
    }
}

impl Drop for VARIANT {
    fn drop(&mut self) {
        unsafe { imp::VariantClear(&mut self.0) };
    }
}

impl Drop for PROPVARIANT {
    fn drop(&mut self) {
        unsafe { imp::PropVariantClear(&mut self.0) };
    }
}

impl TypeKind for VARIANT {
    type TypeKind = ValueType;
}

impl TypeKind for PROPVARIANT {
    type TypeKind = ValueType;
}

impl std::fmt::Debug for VARIANT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("VARIANT");
        debug.field("type", &unsafe { self.0.Anonymous.Anonymous.vt });

        if let Ok(value) = BSTR::try_from(self) {
            debug.field("value", &value);
        }

        debug.finish()
    }
}

impl std::fmt::Debug for PROPVARIANT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("PROPVARIANT");
        debug.field("type", &unsafe { self.0.Anonymous.Anonymous.vt });

        if let Ok(value) = BSTR::try_from(self) {
            debug.field("value", &value);
        }

        debug.finish()
    }
}

impl PartialEq for VARIANT {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            if self.0.Anonymous.Anonymous.vt != other.0.Anonymous.Anonymous.vt {
                return false;
            }
            let this = PROPVARIANT::try_from(self);
            let other = PROPVARIANT::try_from(other);

            if let (Ok(this), Ok(other)) = (this, other) {
                this.eq(&other)
            } else {
                false
            }
        }
    }
}

impl PartialEq for PROPVARIANT {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            if self.0.Anonymous.Anonymous.vt != other.0.Anonymous.Anonymous.vt {
                return false;
            }

            imp::PropVariantCompareEx(&self.0, &other.0, 0, 0) == 0
        }
    }
}

impl Eq for VARIANT {}
impl Eq for PROPVARIANT {}

impl VARIANT {
    pub fn new() -> Self {
        unsafe { std::mem::zeroed() }
    }

    pub const fn is_empty(&self) -> bool {
        unsafe { self.0.Anonymous.Anonymous.vt == imp::VT_EMPTY }
    }
}

impl PROPVARIANT {
    pub fn new() -> Self {
        unsafe { std::mem::zeroed() }
    }

    pub const fn is_empty(&self) -> bool {
        unsafe { self.0.Anonymous.Anonymous.vt == imp::VT_EMPTY }
    }
}

impl TryFrom<&VARIANT> for PROPVARIANT {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        unsafe {
            let mut value = Self::new();
            HRESULT(imp::VariantToPropVariant(&from.0, &mut value.0)).and_then(|| value)
        }
    }
}

impl TryFrom<&PROPVARIANT> for VARIANT {
    type Error = Error;
    fn try_from(from: &PROPVARIANT) -> Result<Self> {
        unsafe {
            let mut value = Self::new();
            HRESULT(imp::PropVariantToVariant(&from.0, &mut value.0)).and_then(|| value)
        }
    }
}

// VT_UNKNOWN

impl From<IUnknown> for VARIANT {
    fn from(value: IUnknown) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 {
                Anonymous: imp::VARIANT_0_0 { vt: imp::VT_UNKNOWN, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { punkVal: value.into_raw() } },
            },
        })
    }
}

impl From<IUnknown> for PROPVARIANT {
    fn from(value: IUnknown) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_UNKNOWN, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { punkVal: value.into_raw() } },
            },
        })
    }
}

impl TryFrom<&VARIANT> for IUnknown {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        unsafe {
            if from.0.Anonymous.Anonymous.vt == imp::VT_UNKNOWN && !from.0.Anonymous.Anonymous.Anonymous.punkVal.is_null() {
                let unknown: &IUnknown = std::mem::transmute(&from.0.Anonymous.Anonymous.Anonymous.punkVal);
                Ok(unknown.clone())
            } else {
                Err(Error { code: imp::TYPE_E_TYPEMISMATCH, info: None })
            }
        }
    }
}

impl TryFrom<&PROPVARIANT> for IUnknown {
    type Error = Error;
    fn try_from(from: &PROPVARIANT) -> Result<Self> {
        unsafe {
            if from.0.Anonymous.Anonymous.vt == imp::VT_UNKNOWN && !from.0.Anonymous.Anonymous.Anonymous.punkVal.is_null() {
                let unknown: &IUnknown = std::mem::transmute(&from.0.Anonymous.Anonymous.Anonymous.punkVal);
                Ok(unknown.clone())
            } else {
                Err(Error { code: imp::TYPE_E_TYPEMISMATCH, info: None })
            }
        }
    }
}

// VT_BSTR

impl From<BSTR> for VARIANT {
    fn from(value: BSTR) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 {
                Anonymous: imp::VARIANT_0_0 { vt: imp::VT_BSTR, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { bstrVal: value.into_raw() } },
            },
        })
    }
}

impl From<BSTR> for PROPVARIANT {
    fn from(value: BSTR) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_BSTR, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { bstrVal: value.into_raw() } },
            },
        })
    }
}

impl TryFrom<&VARIANT> for BSTR {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        let pv = PROPVARIANT::try_from(from)?;
        BSTR::try_from(&pv)
    }
}

impl TryFrom<&PROPVARIANT> for BSTR {
    type Error = Error;
    fn try_from(from: &PROPVARIANT) -> Result<Self> {
        let mut value = Self::new();
        HRESULT(unsafe { imp::PropVariantToBSTR(&from.0, &mut value as *mut _ as *mut _) }).and_then(|| value)
    }
}

// VT_BOOL

impl From<bool> for VARIANT {
    fn from(value: bool) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 {
                Anonymous: imp::VARIANT_0_0 { vt: imp::VT_BOOL, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { boolVal: if value { -1 } else { 0 } } },
            },
        })
    }
}

impl From<bool> for PROPVARIANT {
    fn from(value: bool) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_BOOL, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { boolVal: if value { -1 } else { 0 } } },
            },
        })
    }
}

impl TryFrom<&VARIANT> for bool {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::VariantToBoolean(&from.0, &mut value) }).and_then(|| value != 0)
    }
}

impl TryFrom<&PROPVARIANT> for bool {
    type Error = Error;
    fn try_from(from: &PROPVARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::PropVariantToBoolean(&from.0, &mut value) }).and_then(|| value != 0)
    }
}

// VT_UI1

impl From<u8> for VARIANT {
    fn from(value: u8) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 { Anonymous: imp::VARIANT_0_0 { vt: imp::VT_UI1, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { bVal: value } } },
        })
    }
}

impl From<u8> for PROPVARIANT {
    fn from(value: u8) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_UI1, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { bVal: value } },
            },
        })
    }
}

// VT_I1

impl From<i8> for VARIANT {
    fn from(value: i8) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 { Anonymous: imp::VARIANT_0_0 { vt: imp::VT_I1, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { cVal: value } } },
        })
    }
}

impl From<i8> for PROPVARIANT {
    fn from(value: i8) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_I1, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { cVal: value } },
            },
        })
    }
}

// VT_UI2

impl From<u16> for VARIANT {
    fn from(value: u16) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 { Anonymous: imp::VARIANT_0_0 { vt: imp::VT_UI2, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { uiVal: value } } },
        })
    }
}

impl From<u16> for PROPVARIANT {
    fn from(value: u16) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_UI2, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { uiVal: value } },
            },
        })
    }
}

impl TryFrom<&VARIANT> for u16 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::VariantToUInt16(&from.0, &mut value) }).and_then(|| value)
    }
}

impl TryFrom<&PROPVARIANT> for u16 {
    type Error = Error;
    fn try_from(from: &PROPVARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::PropVariantToUInt16(&from.0, &mut value) }).and_then(|| value)
    }
}

// VT_I2

impl From<i16> for VARIANT {
    fn from(value: i16) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 { Anonymous: imp::VARIANT_0_0 { vt: imp::VT_I2, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { iVal: value } } },
        })
    }
}

impl From<i16> for PROPVARIANT {
    fn from(value: i16) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_I2, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { iVal: value } },
            },
        })
    }
}

impl TryFrom<&VARIANT> for i16 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::VariantToInt16(&from.0, &mut value) }).and_then(|| value)
    }
}

impl TryFrom<&PROPVARIANT> for i16 {
    type Error = Error;
    fn try_from(from: &PROPVARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::PropVariantToInt16(&from.0, &mut value) }).and_then(|| value)
    }
}

// VT_UI4

impl From<u32> for VARIANT {
    fn from(value: u32) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 { Anonymous: imp::VARIANT_0_0 { vt: imp::VT_UI4, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { ulVal: value } } },
        })
    }
}

impl From<u32> for PROPVARIANT {
    fn from(value: u32) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_UI4, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { ulVal: value } },
            },
        })
    }
}

impl TryFrom<&VARIANT> for u32 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::VariantToUInt32(&from.0, &mut value) }).and_then(|| value)
    }
}

impl TryFrom<&PROPVARIANT> for u32 {
    type Error = Error;
    fn try_from(from: &PROPVARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::PropVariantToUInt32(&from.0, &mut value) }).and_then(|| value)
    }
}

// VT_I4

impl From<i32> for VARIANT {
    fn from(value: i32) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 { Anonymous: imp::VARIANT_0_0 { vt: imp::VT_I4, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { lVal: value } } },
        })
    }
}

impl From<i32> for PROPVARIANT {
    fn from(value: i32) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_I4, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { lVal: value } },
            },
        })
    }
}

impl TryFrom<&VARIANT> for i32 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::VariantToInt32(&from.0, &mut value) }).and_then(|| value)
    }
}

impl TryFrom<&PROPVARIANT> for i32 {
    type Error = Error;
    fn try_from(from: &PROPVARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::PropVariantToInt32(&from.0, &mut value) }).and_then(|| value)
    }
}

// VT_UI8

impl From<u64> for VARIANT {
    fn from(value: u64) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 { Anonymous: imp::VARIANT_0_0 { vt: imp::VT_UI8, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { ullVal: value } } },
        })
    }
}

impl From<u64> for PROPVARIANT {
    fn from(value: u64) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_UI8, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { uhVal: value } },
            },
        })
    }
}

impl TryFrom<&VARIANT> for u64 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::VariantToUInt64(&from.0, &mut value) }).and_then(|| value)
    }
}

impl TryFrom<&PROPVARIANT> for u64 {
    type Error = Error;
    fn try_from(from: &PROPVARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::PropVariantToUInt64(&from.0, &mut value) }).and_then(|| value)
    }
}

// VT_I8

impl From<i64> for VARIANT {
    fn from(value: i64) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 { Anonymous: imp::VARIANT_0_0 { vt: imp::VT_I8, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { llVal: value } } },
        })
    }
}

impl From<i64> for PROPVARIANT {
    fn from(value: i64) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_I8, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { hVal: value } },
            },
        })
    }
}

impl TryFrom<&VARIANT> for i64 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::VariantToInt64(&from.0, &mut value) }).and_then(|| value)
    }
}

impl TryFrom<&PROPVARIANT> for i64 {
    type Error = Error;
    fn try_from(from: &PROPVARIANT) -> Result<Self> {
        let mut value = 0;
        HRESULT(unsafe { imp::PropVariantToInt64(&from.0, &mut value) }).and_then(|| value)
    }
}

// VT_R4

impl From<f32> for VARIANT {
    fn from(value: f32) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 { Anonymous: imp::VARIANT_0_0 { vt: imp::VT_R4, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { fltVal: value } } },
        })
    }
}

impl From<f32> for PROPVARIANT {
    fn from(value: f32) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_R4, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { fltVal: value } },
            },
        })
    }
}

// VT_R8

impl From<f64> for VARIANT {
    fn from(value: f64) -> Self {
        Self(imp::VARIANT {
            Anonymous: imp::VARIANT_0 { Anonymous: imp::VARIANT_0_0 { vt: imp::VT_R8, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::VARIANT_0_0_0 { dblVal: value } } },
        })
    }
}

impl From<f64> for PROPVARIANT {
    fn from(value: f64) -> Self {
        Self(imp::PROPVARIANT {
            Anonymous: imp::PROPVARIANT_0 {
                Anonymous: imp::PROPVARIANT_0_0 { vt: imp::VT_R8, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: imp::PROPVARIANT_0_0_0 { dblVal: value } },
            },
        })
    }
}

impl TryFrom<&VARIANT> for f64 {
    type Error = Error;
    fn try_from(from: &VARIANT) -> Result<Self> {
        let mut value = 0.0;
        HRESULT(unsafe { imp::VariantToDouble(&from.0, &mut value) }).and_then(|| value)
    }
}

impl TryFrom<&PROPVARIANT> for f64 {
    type Error = Error;
    fn try_from(from: &PROPVARIANT) -> Result<Self> {
        let mut value = 0.0;
        HRESULT(unsafe { imp::PropVariantToDouble(&from.0, &mut value) }).and_then(|| value)
    }
}