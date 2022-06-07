#[cfg(feature = "Devices_Printers_Extensions")]
pub mod Extensions;
#[repr(C)]
pub struct IIppAttributeError {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IppAttributeErrorReason) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnsupportedValues: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnsupportedValues: usize,
}
impl ::windows_sys::core::Interface for IIppAttributeError {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1963978145, data2: 40687, data3: 23609, data4: [147, 228, 70, 20, 155, 188, 239, 39] };
}
#[repr(C)]
pub struct IIppAttributeValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IppAttributeValueKind) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntegerArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntegerArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBooleanArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBooleanArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEnumArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEnumArray: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetOctetStringArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetOctetStringArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDateTimeArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDateTimeArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetResolutionArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetResolutionArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRangeOfIntegerArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRangeOfIntegerArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCollectionArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCollectionArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextWithLanguageArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextWithLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNameWithLanguageArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNameWithLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextWithoutLanguageArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextWithoutLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNameWithoutLanguageArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNameWithoutLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetKeywordArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetKeywordArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUriArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUriArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUriSchemaArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUriSchemaArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCharsetArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCharsetArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNaturalLanguageArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNaturalLanguageArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMimeMediaTypeArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMimeMediaTypeArray: usize,
}
impl ::windows_sys::core::Interface for IIppAttributeValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2571141101, data2: 58043, data3: 22947, data4: [152, 139, 40, 169, 116, 5, 42, 38] };
}
#[repr(C)]
pub struct IIppAttributeValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateUnsupported: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateUnknown: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateNoValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInteger: unsafe extern "system" fn(this: *mut *mut Self, value: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateIntegerArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateIntegerArray: usize,
    pub CreateBoolean: unsafe extern "system" fn(this: *mut *mut Self, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateBooleanArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateBooleanArray: usize,
    pub CreateEnum: unsafe extern "system" fn(this: *mut *mut Self, value: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateEnumArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateEnumArray: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateOctetString: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateOctetString: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub CreateOctetStringArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    CreateOctetStringArray: usize,
    #[cfg(feature = "Foundation")]
    pub CreateDateTime: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDateTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateDateTimeArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateDateTimeArray: usize,
    pub CreateResolution: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateResolutionArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateResolutionArray: usize,
    pub CreateRangeOfInteger: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateRangeOfIntegerArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateRangeOfIntegerArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCollection: unsafe extern "system" fn(this: *mut *mut Self, memberattributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCollection: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCollectionArray: unsafe extern "system" fn(this: *mut *mut Self, memberattributesarray: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCollectionArray: usize,
    pub CreateTextWithLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateTextWithLanguageArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateTextWithLanguageArray: usize,
    pub CreateNameWithLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNameWithLanguageArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNameWithLanguageArray: usize,
    pub CreateTextWithoutLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateTextWithoutLanguageArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateTextWithoutLanguageArray: usize,
    pub CreateNameWithoutLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNameWithoutLanguageArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNameWithoutLanguageArray: usize,
    pub CreateKeyword: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateKeywordArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateKeywordArray: usize,
    #[cfg(feature = "Foundation")]
    pub CreateUri: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateUri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUriArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUriArray: usize,
    pub CreateUriSchema: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateUriSchemaArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateUriSchemaArray: usize,
    pub CreateCharset: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCharsetArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCharsetArray: usize,
    pub CreateNaturalLanguage: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNaturalLanguageArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNaturalLanguageArray: usize,
    pub CreateMimeMedia: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateMimeMediaArray: unsafe extern "system" fn(this: *mut *mut Self, values: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateMimeMediaArray: usize,
}
impl ::windows_sys::core::Interface for IIppAttributeValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 282343746, data2: 56724, data3: 22936, data4: [178, 53, 175, 175, 182, 250, 121, 53] };
}
#[repr(C)]
pub struct IIppIntegerRange {
    pub base__: ::windows_sys::core::IInspectable,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIppIntegerRange {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2458940230, data2: 50154, data3: 24278, data4: [189, 177, 55, 82, 198, 44, 111, 127] };
}
#[repr(C)]
pub struct IIppIntegerRangeFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, start: i32, end: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIppIntegerRangeFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1976888494, data2: 63614, data3: 21677, data4: [181, 208, 70, 82, 4, 219, 117, 83] };
}
#[repr(C)]
pub struct IIppPrintDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrinterName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PrinterUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrinterUri: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetPrinterAttributesAsBuffer: unsafe extern "system" fn(this: *mut *mut Self, attributenames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetPrinterAttributesAsBuffer: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPrinterAttributes: unsafe extern "system" fn(this: *mut *mut Self, attributenames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPrinterAttributes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPrinterAttributesFromBuffer: unsafe extern "system" fn(this: *mut *mut Self, printerattributesbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPrinterAttributesFromBuffer: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPrinterAttributes: unsafe extern "system" fn(this: *mut *mut Self, printerattributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPrinterAttributes: usize,
}
impl ::windows_sys::core::Interface for IIppPrintDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3611864150, data2: 30451, data3: 24006, data4: [175, 212, 194, 168, 104, 107, 147, 89] };
}
#[repr(C)]
pub struct IIppResolution {
    pub base__: ::windows_sys::core::IInspectable,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Unit: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut IppResolutionUnit) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIppResolution {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3410575238, data2: 27635, data3: 22261, data4: [134, 206, 38, 61, 8, 174, 173, 99] };
}
#[repr(C)]
pub struct IIppResolutionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, width: i32, height: i32, unit: IppResolutionUnit, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIppResolutionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3833709230, data2: 9498, data3: 21286, data4: [177, 115, 149, 84, 62, 217, 154, 53] };
}
#[repr(C)]
pub struct IIppSetAttributesResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AttributeErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AttributeErrors: usize,
}
impl ::windows_sys::core::Interface for IIppSetAttributesResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2099019605, data2: 43677, data3: 22691, data4: [144, 233, 23, 189, 197, 40, 31, 7] };
}
#[repr(C)]
pub struct IIppTextWithLanguage {
    pub base__: ::windows_sys::core::IInspectable,
    pub Language: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIppTextWithLanguage {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 845432742, data2: 20809, data3: 22838, data4: [144, 232, 12, 115, 96, 54, 191, 119] };
}
#[repr(C)]
pub struct IIppTextWithLanguageFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, language: ::windows_sys::core::HSTRING, text: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IIppTextWithLanguageFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3393855117, data2: 10600, data3: 22389, data4: [153, 124, 138, 70, 241, 165, 116, 237] };
}
#[repr(C)]
pub struct IPrint3DDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub PrintSchema: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrint3DDevice {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 68959513, data2: 38675, data3: 17058, data4: [152, 19, 125, 195, 51, 116, 40, 211] };
}
#[repr(C)]
pub struct IPrint3DDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPrint3DDeviceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4259537418, data2: 26573, data3: 16823, data4: [163, 68, 81, 80, 161, 253, 117, 181] };
}
#[repr(C)]
pub struct IPrintSchema {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetDefaultPrintTicketAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetDefaultPrintTicketAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetCapabilitiesAsync: unsafe extern "system" fn(this: *mut *mut Self, constrainticket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetCapabilitiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub MergeAndValidateWithDefaultPrintTicketAsync: unsafe extern "system" fn(this: *mut *mut Self, deltaticket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    MergeAndValidateWithDefaultPrintTicketAsync: usize,
}
impl ::windows_sys::core::Interface for IPrintSchema {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3266937622, data2: 9912, data3: 19451, data4: [129, 56, 159, 150, 44, 34, 163, 91] };
}
pub type IppAttributeError = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppAttributeErrorReason(pub i32);
impl IppAttributeErrorReason {
    pub const RequestEntityTooLarge: Self = Self(0i32);
    pub const AttributeNotSupported: Self = Self(1i32);
    pub const AttributeValuesNotSupported: Self = Self(2i32);
    pub const AttributeNotSettable: Self = Self(3i32);
    pub const ConflictingAttributes: Self = Self(4i32);
}
impl ::core::marker::Copy for IppAttributeErrorReason {}
impl ::core::clone::Clone for IppAttributeErrorReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IppAttributeValue = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppAttributeValueKind(pub i32);
impl IppAttributeValueKind {
    pub const Unsupported: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const NoValue: Self = Self(2i32);
    pub const Integer: Self = Self(3i32);
    pub const Boolean: Self = Self(4i32);
    pub const Enum: Self = Self(5i32);
    pub const OctetString: Self = Self(6i32);
    pub const DateTime: Self = Self(7i32);
    pub const Resolution: Self = Self(8i32);
    pub const RangeOfInteger: Self = Self(9i32);
    pub const Collection: Self = Self(10i32);
    pub const TextWithLanguage: Self = Self(11i32);
    pub const NameWithLanguage: Self = Self(12i32);
    pub const TextWithoutLanguage: Self = Self(13i32);
    pub const NameWithoutLanguage: Self = Self(14i32);
    pub const Keyword: Self = Self(15i32);
    pub const Uri: Self = Self(16i32);
    pub const UriSchema: Self = Self(17i32);
    pub const Charset: Self = Self(18i32);
    pub const NaturalLanguage: Self = Self(19i32);
    pub const MimeMediaType: Self = Self(20i32);
}
impl ::core::marker::Copy for IppAttributeValueKind {}
impl ::core::clone::Clone for IppAttributeValueKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IppIntegerRange = *mut ::core::ffi::c_void;
pub type IppPrintDevice = *mut ::core::ffi::c_void;
pub type IppResolution = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Printers\"`*"]
#[repr(transparent)]
pub struct IppResolutionUnit(pub i32);
impl IppResolutionUnit {
    pub const DotsPerInch: Self = Self(0i32);
    pub const DotsPerCentimeter: Self = Self(1i32);
}
impl ::core::marker::Copy for IppResolutionUnit {}
impl ::core::clone::Clone for IppResolutionUnit {
    fn clone(&self) -> Self {
        *self
    }
}
pub type IppSetAttributesResult = *mut ::core::ffi::c_void;
pub type IppTextWithLanguage = *mut ::core::ffi::c_void;
pub type Print3DDevice = *mut ::core::ffi::c_void;
pub type PrintSchema = *mut ::core::ffi::c_void;
