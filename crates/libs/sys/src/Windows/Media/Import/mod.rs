#[repr(C)]
pub struct IPhotoImportDeleteImportedItemsFromSourceResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasSucceeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DeletedItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeletedItems: usize,
    pub PhotosCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PhotosSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub VideosCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub VideosSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SidecarsCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SidecarsSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SiblingsCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SiblingsSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub TotalCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TotalSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhotoImportDeleteImportedItemsFromSourceResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4108391160, data2: 33853, data3: 17034, data4: [161, 166, 129, 81, 2, 146, 176, 174] };
}
#[repr(C)]
pub struct IPhotoImportFindItemsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasSucceeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FoundItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FoundItems: usize,
    pub PhotosCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PhotosSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub VideosCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub VideosSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SidecarsCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SidecarsSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SiblingsCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SiblingsSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub TotalCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TotalSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SelectAll: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub SelectNone: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectNewAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectNewAsync: usize,
    pub SetImportMode: unsafe extern "system" fn(this: *mut *mut Self, value: PhotoImportImportMode) -> ::windows_sys::core::HRESULT,
    pub ImportMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoImportImportMode) -> ::windows_sys::core::HRESULT,
    pub SelectedPhotosCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SelectedPhotosSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SelectedVideosCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SelectedVideosSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SelectedSidecarsCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SelectedSidecarsSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SelectedSiblingsCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SelectedSiblingsSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SelectedTotalCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SelectedTotalSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ImportItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportItemsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ItemImported: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemImported: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemImported: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemImported: usize,
}
impl ::windows_sys::core::Interface for IPhotoImportFindItemsResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 957736519, data2: 27768, data3: 18731, data4: [132, 78, 143, 229, 232, 246, 191, 185] };
}
#[repr(C)]
pub struct IPhotoImportFindItemsResult2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub AddItemsInDateRangeToSelection: unsafe extern "system" fn(this: *mut *mut Self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddItemsInDateRangeToSelection: usize,
}
impl ::windows_sys::core::Interface for IPhotoImportFindItemsResult2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4225591867, data2: 60665, data3: 16490, data4: [129, 94, 80, 21, 98, 91, 10, 136] };
}
#[repr(C)]
pub struct IPhotoImportImportItemsResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasSucceeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportedItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportedItems: usize,
    pub PhotosCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PhotosSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub VideosCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub VideosSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SidecarsCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SidecarsSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SiblingsCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SiblingsSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub TotalCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub TotalSizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteImportedItemsFromSourceAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteImportedItemsFromSourceAsync: usize,
}
impl ::windows_sys::core::Interface for IPhotoImportImportItemsResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3839161464, data2: 54297, data3: 17475, data4: [168, 78, 240, 106, 133, 12, 11, 0] };
}
#[repr(C)]
pub struct IPhotoImportItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ItemKey: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub ContentType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoImportContentType) -> ::windows_sys::core::HRESULT,
    pub SizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    pub Sibling: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Sidecars: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sidecars: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub VideoSegments: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VideoSegments: usize,
    pub IsSelected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsSelected: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportedFileNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportedFileNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeletedFileNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeletedFileNames: usize,
}
impl ::windows_sys::core::Interface for IPhotoImportItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2849013366, data2: 39932, data3: 17336, data4: [179, 86, 99, 59, 106, 152, 140, 158] };
}
#[repr(C)]
pub struct IPhotoImportItem2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhotoImportItem2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4043650309, data2: 62779, data3: 18083, data4: [158, 48, 54, 16, 121, 26, 145, 16] };
}
#[repr(C)]
pub struct IPhotoImportItemImportedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub ImportedItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhotoImportItemImportedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1120612317, data2: 32104, data3: 18357, data4: [188, 124, 206, 183, 62, 12, 119, 220] };
}
#[repr(C)]
pub struct IPhotoImportManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub IsSupportedAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllSourcesAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllSourcesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPendingOperations: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPendingOperations: usize,
}
impl ::windows_sys::core::Interface for IPhotoImportManagerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 661753917, data2: 41030, data3: 20230, data4: [155, 156, 191, 214, 98, 232, 50, 135] };
}
#[repr(C)]
pub struct IPhotoImportOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Stage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoImportStage) -> ::windows_sys::core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContinueFindingItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContinueFindingItemsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ContinueImportingItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContinueImportingItemsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ContinueDeletingImportedItemsFromSourceAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContinueDeletingImportedItemsFromSourceAsync: usize,
}
impl ::windows_sys::core::Interface for IPhotoImportOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3656882148, data2: 41114, data3: 20196, data4: [164, 177, 32, 148, 2, 119, 165, 190] };
}
#[repr(C)]
pub struct IPhotoImportSelectionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsSelectionEmpty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhotoImportSelectionChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 273028994, data2: 64157, data3: 19504, data4: [139, 201, 77, 100, 145, 21, 114, 213] };
}
#[repr(C)]
pub struct IPhotoImportSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub Source: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SetDestinationFolder: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetDestinationFolder: usize,
    #[cfg(feature = "Storage")]
    pub DestinationFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    DestinationFolder: usize,
    pub SetAppendSessionDateToDestinationFolder: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub AppendSessionDateToDestinationFolder: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetSubfolderCreationMode: unsafe extern "system" fn(this: *mut *mut Self, value: PhotoImportSubfolderCreationMode) -> ::windows_sys::core::HRESULT,
    pub SubfolderCreationMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoImportSubfolderCreationMode) -> ::windows_sys::core::HRESULT,
    pub SetDestinationFileNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DestinationFileNamePrefix: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FindItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, contenttypefilter: PhotoImportContentTypeFilter, itemselectionmode: PhotoImportItemSelectionMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindItemsAsync: usize,
}
impl ::windows_sys::core::Interface for IPhotoImportSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2858652014, data2: 60635, data3: 20222, data4: [148, 198, 95, 92, 175, 227, 76, 251] };
}
#[repr(C)]
pub struct IPhotoImportSession2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetSubfolderDateFormat: unsafe extern "system" fn(this: *mut *mut Self, value: PhotoImportSubfolderDateFormat) -> ::windows_sys::core::HRESULT,
    pub SubfolderDateFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoImportSubfolderDateFormat) -> ::windows_sys::core::HRESULT,
    pub SetRememberDeselectedItems: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub RememberDeselectedItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhotoImportSession2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 710043408, data2: 16070, data3: 18077, data4: [163, 117, 43, 159, 71, 133, 57, 30] };
}
#[repr(C)]
pub struct IPhotoImportSidecar {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
}
impl ::windows_sys::core::Interface for IPhotoImportSidecar {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1188550487, data2: 63490, data3: 17607, data4: [156, 152, 122, 113, 244, 188, 20, 134] };
}
#[repr(C)]
pub struct IPhotoImportSource {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Manufacturer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Model: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ConnectionProtocol: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ConnectionTransport: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoImportConnectionTransport) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoImportSourceType) -> ::windows_sys::core::HRESULT,
    pub PowerSource: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoImportPowerSource) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BatteryLevelPercent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BatteryLevelPercent: usize,
    #[cfg(feature = "Foundation")]
    pub DateTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StorageMedia: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorageMedia: usize,
    #[cfg(feature = "Foundation")]
    pub IsLocked: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsLocked: usize,
    pub IsMassStorage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    pub CreateImportSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhotoImportSource {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 529441630, data2: 5211, data3: 19670, data4: [135, 241, 84, 150, 90, 152, 47, 239] };
}
#[repr(C)]
pub struct IPhotoImportSourceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, sourceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub FromFolderAsync: unsafe extern "system" fn(this: *mut *mut Self, sourcerootfolder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    FromFolderAsync: usize,
}
impl ::windows_sys::core::Interface for IPhotoImportSourceStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 86566278, data2: 13016, data3: 18044, data4: [140, 238, 35, 161, 178, 244, 62, 133] };
}
#[repr(C)]
pub struct IPhotoImportStorageMedium {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StorageMediumType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoImportStorageMediumType) -> ::windows_sys::core::HRESULT,
    pub SupportedAccessMode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PhotoImportAccessMode) -> ::windows_sys::core::HRESULT,
    pub CapacityInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub AvailableSpaceInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPhotoImportStorageMedium {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4072255635, data2: 64645, data3: 18559, data4: [135, 194, 88, 214, 117, 208, 91, 7] };
}
#[repr(C)]
pub struct IPhotoImportVideoSegment {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SizeInBytes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    pub Sibling: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Sidecars: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sidecars: usize,
}
impl ::windows_sys::core::Interface for IPhotoImportVideoSegment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1648099977, data2: 12826, data3: 16856, data4: [145, 102, 140, 98, 163, 51, 39, 108] };
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportAccessMode(pub i32);
impl PhotoImportAccessMode {
    pub const ReadWrite: Self = Self(0i32);
    pub const ReadOnly: Self = Self(1i32);
    pub const ReadAndDelete: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportAccessMode {}
impl ::core::clone::Clone for PhotoImportAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportConnectionTransport(pub i32);
impl PhotoImportConnectionTransport {
    pub const Unknown: Self = Self(0i32);
    pub const Usb: Self = Self(1i32);
    pub const IP: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportConnectionTransport {}
impl ::core::clone::Clone for PhotoImportConnectionTransport {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportContentType(pub i32);
impl PhotoImportContentType {
    pub const Unknown: Self = Self(0i32);
    pub const Image: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportContentType {}
impl ::core::clone::Clone for PhotoImportContentType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportContentTypeFilter(pub i32);
impl PhotoImportContentTypeFilter {
    pub const OnlyImages: Self = Self(0i32);
    pub const OnlyVideos: Self = Self(1i32);
    pub const ImagesAndVideos: Self = Self(2i32);
    pub const ImagesAndVideosFromCameraRoll: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportContentTypeFilter {}
impl ::core::clone::Clone for PhotoImportContentTypeFilter {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhotoImportDeleteImportedItemsFromSourceResult = *mut ::core::ffi::c_void;
pub type PhotoImportFindItemsResult = *mut ::core::ffi::c_void;
pub type PhotoImportImportItemsResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportImportMode(pub i32);
impl PhotoImportImportMode {
    pub const ImportEverything: Self = Self(0i32);
    pub const IgnoreSidecars: Self = Self(1i32);
    pub const IgnoreSiblings: Self = Self(2i32);
    pub const IgnoreSidecarsAndSiblings: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportImportMode {}
impl ::core::clone::Clone for PhotoImportImportMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhotoImportItem = *mut ::core::ffi::c_void;
pub type PhotoImportItemImportedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportItemSelectionMode(pub i32);
impl PhotoImportItemSelectionMode {
    pub const SelectAll: Self = Self(0i32);
    pub const SelectNone: Self = Self(1i32);
    pub const SelectNew: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportItemSelectionMode {}
impl ::core::clone::Clone for PhotoImportItemSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhotoImportOperation = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportPowerSource(pub i32);
impl PhotoImportPowerSource {
    pub const Unknown: Self = Self(0i32);
    pub const Battery: Self = Self(1i32);
    pub const External: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportPowerSource {}
impl ::core::clone::Clone for PhotoImportPowerSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Media_Import\"`*"]
pub struct PhotoImportProgress {
    pub ItemsImported: u32,
    pub TotalItemsToImport: u32,
    pub BytesImported: u64,
    pub TotalBytesToImport: u64,
    pub ImportProgress: f64,
}
impl ::core::marker::Copy for PhotoImportProgress {}
impl ::core::clone::Clone for PhotoImportProgress {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhotoImportSelectionChangedEventArgs = *mut ::core::ffi::c_void;
pub type PhotoImportSession = *mut ::core::ffi::c_void;
pub type PhotoImportSidecar = *mut ::core::ffi::c_void;
pub type PhotoImportSource = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportSourceType(pub i32);
impl PhotoImportSourceType {
    pub const Generic: Self = Self(0i32);
    pub const Camera: Self = Self(1i32);
    pub const MediaPlayer: Self = Self(2i32);
    pub const Phone: Self = Self(3i32);
    pub const Video: Self = Self(4i32);
    pub const PersonalInfoManager: Self = Self(5i32);
    pub const AudioRecorder: Self = Self(6i32);
}
impl ::core::marker::Copy for PhotoImportSourceType {}
impl ::core::clone::Clone for PhotoImportSourceType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportStage(pub i32);
impl PhotoImportStage {
    pub const NotStarted: Self = Self(0i32);
    pub const FindingItems: Self = Self(1i32);
    pub const ImportingItems: Self = Self(2i32);
    pub const DeletingImportedItemsFromSource: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportStage {}
impl ::core::clone::Clone for PhotoImportStage {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhotoImportStorageMedium = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportStorageMediumType(pub i32);
impl PhotoImportStorageMediumType {
    pub const Undefined: Self = Self(0i32);
    pub const Fixed: Self = Self(1i32);
    pub const Removable: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportStorageMediumType {}
impl ::core::clone::Clone for PhotoImportStorageMediumType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportSubfolderCreationMode(pub i32);
impl PhotoImportSubfolderCreationMode {
    pub const DoNotCreateSubfolders: Self = Self(0i32);
    pub const CreateSubfoldersFromFileDate: Self = Self(1i32);
    pub const CreateSubfoldersFromExifDate: Self = Self(2i32);
    pub const KeepOriginalFolderStructure: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportSubfolderCreationMode {}
impl ::core::clone::Clone for PhotoImportSubfolderCreationMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportSubfolderDateFormat(pub i32);
impl PhotoImportSubfolderDateFormat {
    pub const Year: Self = Self(0i32);
    pub const YearMonth: Self = Self(1i32);
    pub const YearMonthDay: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportSubfolderDateFormat {}
impl ::core::clone::Clone for PhotoImportSubfolderDateFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PhotoImportVideoSegment = *mut ::core::ffi::c_void;
