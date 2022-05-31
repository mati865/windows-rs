#[cfg(feature = "HardwareCounterProfiling")]
pub mod HardwareCounterProfiling;
#[link(name = "windows")]
extern "system" {
    pub fn BackupPerfRegistryToFileW(szfilename: ::windows_core_sys::PCWSTR, szcommentstring: ::windows_core_sys::PCWSTR) -> u32;
    pub fn InstallPerfDllA(szcomputername: ::windows_core_sys::PCSTR, lpinifile: ::windows_core_sys::PCSTR, dwflags: usize) -> u32;
    pub fn InstallPerfDllW(szcomputername: ::windows_core_sys::PCWSTR, lpinifile: ::windows_core_sys::PCWSTR, dwflags: usize) -> u32;
    pub fn LoadPerfCounterTextStringsA(lpcommandline: ::windows_core_sys::PCSTR, bquietmodearg: ::win32_foundation_sys::BOOL) -> u32;
    pub fn LoadPerfCounterTextStringsW(lpcommandline: ::windows_core_sys::PCWSTR, bquietmodearg: ::win32_foundation_sys::BOOL) -> u32;
    pub fn PdhAddCounterA(hquery: isize, szfullcounterpath: ::windows_core_sys::PCSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    pub fn PdhAddCounterW(hquery: isize, szfullcounterpath: ::windows_core_sys::PCWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    pub fn PdhAddEnglishCounterA(hquery: isize, szfullcounterpath: ::windows_core_sys::PCSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    pub fn PdhAddEnglishCounterW(hquery: isize, szfullcounterpath: ::windows_core_sys::PCWSTR, dwuserdata: usize, phcounter: *mut isize) -> i32;
    pub fn PdhBindInputDataSourceA(phdatasource: *mut isize, logfilenamelist: ::windows_core_sys::PCSTR) -> i32;
    pub fn PdhBindInputDataSourceW(phdatasource: *mut isize, logfilenamelist: ::windows_core_sys::PCWSTR) -> i32;
    pub fn PdhBrowseCountersA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_A) -> i32;
    pub fn PdhBrowseCountersHA(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HA) -> i32;
    pub fn PdhBrowseCountersHW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_HW) -> i32;
    pub fn PdhBrowseCountersW(pbrowsedlgdata: *const PDH_BROWSE_DLG_CONFIG_W) -> i32;
    pub fn PdhCalculateCounterFromRawValue(hcounter: isize, dwformat: PDH_FMT, rawvalue1: *const PDH_RAW_COUNTER, rawvalue2: *const PDH_RAW_COUNTER, fmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
    pub fn PdhCloseLog(hlog: isize, dwflags: u32) -> i32;
    pub fn PdhCloseQuery(hquery: isize) -> i32;
    pub fn PdhCollectQueryData(hquery: isize) -> i32;
    pub fn PdhCollectQueryDataEx(hquery: isize, dwintervaltime: u32, hnewdataevent: ::win32_foundation_sys::HANDLE) -> i32;
    pub fn PdhCollectQueryDataWithTime(hquery: isize, plltimestamp: *mut i64) -> i32;
    pub fn PdhComputeCounterStatistics(hcounter: isize, dwformat: PDH_FMT, dwfirstentry: u32, dwnumentries: u32, lprawvaluearray: *const PDH_RAW_COUNTER, data: *mut PDH_STATISTICS) -> i32;
    pub fn PdhConnectMachineA(szmachinename: ::windows_core_sys::PCSTR) -> i32;
    pub fn PdhConnectMachineW(szmachinename: ::windows_core_sys::PCWSTR) -> i32;
    pub fn PdhCreateSQLTablesA(szdatasource: ::windows_core_sys::PCSTR) -> i32;
    pub fn PdhCreateSQLTablesW(szdatasource: ::windows_core_sys::PCWSTR) -> i32;
    pub fn PdhEnumLogSetNamesA(szdatasource: ::windows_core_sys::PCSTR, mszdatasetnamelist: ::windows_core_sys::PSTR, pcchbufferlength: *mut u32) -> i32;
    pub fn PdhEnumLogSetNamesW(szdatasource: ::windows_core_sys::PCWSTR, mszdatasetnamelist: ::windows_core_sys::PWSTR, pcchbufferlength: *mut u32) -> i32;
    pub fn PdhEnumMachinesA(szdatasource: ::windows_core_sys::PCSTR, mszmachinelist: ::windows_core_sys::PSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhEnumMachinesHA(hdatasource: isize, mszmachinelist: ::windows_core_sys::PSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhEnumMachinesHW(hdatasource: isize, mszmachinelist: ::windows_core_sys::PWSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhEnumMachinesW(szdatasource: ::windows_core_sys::PCWSTR, mszmachinelist: ::windows_core_sys::PWSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhEnumObjectItemsA(szdatasource: ::windows_core_sys::PCSTR, szmachinename: ::windows_core_sys::PCSTR, szobjectname: ::windows_core_sys::PCSTR, mszcounterlist: ::windows_core_sys::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows_core_sys::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    pub fn PdhEnumObjectItemsHA(hdatasource: isize, szmachinename: ::windows_core_sys::PCSTR, szobjectname: ::windows_core_sys::PCSTR, mszcounterlist: ::windows_core_sys::PSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows_core_sys::PSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    pub fn PdhEnumObjectItemsHW(hdatasource: isize, szmachinename: ::windows_core_sys::PCWSTR, szobjectname: ::windows_core_sys::PCWSTR, mszcounterlist: ::windows_core_sys::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows_core_sys::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    pub fn PdhEnumObjectItemsW(szdatasource: ::windows_core_sys::PCWSTR, szmachinename: ::windows_core_sys::PCWSTR, szobjectname: ::windows_core_sys::PCWSTR, mszcounterlist: ::windows_core_sys::PWSTR, pcchcounterlistlength: *mut u32, mszinstancelist: ::windows_core_sys::PWSTR, pcchinstancelistlength: *mut u32, dwdetaillevel: PERF_DETAIL, dwflags: u32) -> i32;
    pub fn PdhEnumObjectsA(szdatasource: ::windows_core_sys::PCSTR, szmachinename: ::windows_core_sys::PCSTR, mszobjectlist: ::windows_core_sys::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: ::win32_foundation_sys::BOOL) -> i32;
    pub fn PdhEnumObjectsHA(hdatasource: isize, szmachinename: ::windows_core_sys::PCSTR, mszobjectlist: ::windows_core_sys::PSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: ::win32_foundation_sys::BOOL) -> i32;
    pub fn PdhEnumObjectsHW(hdatasource: isize, szmachinename: ::windows_core_sys::PCWSTR, mszobjectlist: ::windows_core_sys::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: ::win32_foundation_sys::BOOL) -> i32;
    pub fn PdhEnumObjectsW(szdatasource: ::windows_core_sys::PCWSTR, szmachinename: ::windows_core_sys::PCWSTR, mszobjectlist: ::windows_core_sys::PWSTR, pcchbuffersize: *mut u32, dwdetaillevel: PERF_DETAIL, brefresh: ::win32_foundation_sys::BOOL) -> i32;
    pub fn PdhExpandCounterPathA(szwildcardpath: ::windows_core_sys::PCSTR, mszexpandedpathlist: ::windows_core_sys::PSTR, pcchpathlistlength: *mut u32) -> i32;
    pub fn PdhExpandCounterPathW(szwildcardpath: ::windows_core_sys::PCWSTR, mszexpandedpathlist: ::windows_core_sys::PWSTR, pcchpathlistlength: *mut u32) -> i32;
    pub fn PdhExpandWildCardPathA(szdatasource: ::windows_core_sys::PCSTR, szwildcardpath: ::windows_core_sys::PCSTR, mszexpandedpathlist: ::windows_core_sys::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    pub fn PdhExpandWildCardPathHA(hdatasource: isize, szwildcardpath: ::windows_core_sys::PCSTR, mszexpandedpathlist: ::windows_core_sys::PSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    pub fn PdhExpandWildCardPathHW(hdatasource: isize, szwildcardpath: ::windows_core_sys::PCWSTR, mszexpandedpathlist: ::windows_core_sys::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    pub fn PdhExpandWildCardPathW(szdatasource: ::windows_core_sys::PCWSTR, szwildcardpath: ::windows_core_sys::PCWSTR, mszexpandedpathlist: ::windows_core_sys::PWSTR, pcchpathlistlength: *mut u32, dwflags: u32) -> i32;
    pub fn PdhFormatFromRawValue(dwcountertype: u32, dwformat: PDH_FMT, ptimebase: *const i64, prawvalue1: *const PDH_RAW_COUNTER, prawvalue2: *const PDH_RAW_COUNTER, pfmtvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
    pub fn PdhGetCounterInfoA(hcounter: isize, bretrieveexplaintext: ::win32_foundation_sys::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_A) -> i32;
    pub fn PdhGetCounterInfoW(hcounter: isize, bretrieveexplaintext: ::win32_foundation_sys::BOOLEAN, pdwbuffersize: *mut u32, lpbuffer: *mut PDH_COUNTER_INFO_W) -> i32;
    pub fn PdhGetCounterTimeBase(hcounter: isize, ptimebase: *mut i64) -> i32;
    pub fn PdhGetDataSourceTimeRangeA(szdatasource: ::windows_core_sys::PCSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
    pub fn PdhGetDataSourceTimeRangeH(hdatasource: isize, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
    pub fn PdhGetDataSourceTimeRangeW(szdatasource: ::windows_core_sys::PCWSTR, pdwnumentries: *mut u32, pinfo: *mut PDH_TIME_INFO, pdwbuffersize: *mut u32) -> i32;
    pub fn PdhGetDefaultPerfCounterA(szdatasource: ::windows_core_sys::PCSTR, szmachinename: ::windows_core_sys::PCSTR, szobjectname: ::windows_core_sys::PCSTR, szdefaultcountername: ::windows_core_sys::PSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhGetDefaultPerfCounterHA(hdatasource: isize, szmachinename: ::windows_core_sys::PCSTR, szobjectname: ::windows_core_sys::PCSTR, szdefaultcountername: ::windows_core_sys::PSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhGetDefaultPerfCounterHW(hdatasource: isize, szmachinename: ::windows_core_sys::PCWSTR, szobjectname: ::windows_core_sys::PCWSTR, szdefaultcountername: ::windows_core_sys::PWSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhGetDefaultPerfCounterW(szdatasource: ::windows_core_sys::PCWSTR, szmachinename: ::windows_core_sys::PCWSTR, szobjectname: ::windows_core_sys::PCWSTR, szdefaultcountername: ::windows_core_sys::PWSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhGetDefaultPerfObjectA(szdatasource: ::windows_core_sys::PCSTR, szmachinename: ::windows_core_sys::PCSTR, szdefaultobjectname: ::windows_core_sys::PSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhGetDefaultPerfObjectHA(hdatasource: isize, szmachinename: ::windows_core_sys::PCSTR, szdefaultobjectname: ::windows_core_sys::PSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhGetDefaultPerfObjectHW(hdatasource: isize, szmachinename: ::windows_core_sys::PCWSTR, szdefaultobjectname: ::windows_core_sys::PWSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhGetDefaultPerfObjectW(szdatasource: ::windows_core_sys::PCWSTR, szmachinename: ::windows_core_sys::PCWSTR, szdefaultobjectname: ::windows_core_sys::PWSTR, pcchbuffersize: *mut u32) -> i32;
    pub fn PdhGetDllVersion(lpdwversion: *mut PDH_DLL_VERSION) -> i32;
    pub fn PdhGetFormattedCounterArrayA(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_A) -> i32;
    pub fn PdhGetFormattedCounterArrayW(hcounter: isize, dwformat: PDH_FMT, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_FMT_COUNTERVALUE_ITEM_W) -> i32;
    pub fn PdhGetFormattedCounterValue(hcounter: isize, dwformat: PDH_FMT, lpdwtype: *mut u32, pvalue: *mut PDH_FMT_COUNTERVALUE) -> i32;
    pub fn PdhGetLogFileSize(hlog: isize, llsize: *mut i64) -> i32;
    pub fn PdhGetLogSetGUID(hlog: isize, pguid: *mut ::windows_core_sys::GUID, prunid: *mut i32) -> i32;
    pub fn PdhGetRawCounterArrayA(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_A) -> i32;
    pub fn PdhGetRawCounterArrayW(hcounter: isize, lpdwbuffersize: *mut u32, lpdwitemcount: *mut u32, itembuffer: *mut PDH_RAW_COUNTER_ITEM_W) -> i32;
    pub fn PdhGetRawCounterValue(hcounter: isize, lpdwtype: *mut u32, pvalue: *mut PDH_RAW_COUNTER) -> i32;
    pub fn PdhIsRealTimeQuery(hquery: isize) -> ::win32_foundation_sys::BOOL;
    pub fn PdhLookupPerfIndexByNameA(szmachinename: ::windows_core_sys::PCSTR, sznamebuffer: ::windows_core_sys::PCSTR, pdwindex: *mut u32) -> i32;
    pub fn PdhLookupPerfIndexByNameW(szmachinename: ::windows_core_sys::PCWSTR, sznamebuffer: ::windows_core_sys::PCWSTR, pdwindex: *mut u32) -> i32;
    pub fn PdhLookupPerfNameByIndexA(szmachinename: ::windows_core_sys::PCSTR, dwnameindex: u32, sznamebuffer: ::windows_core_sys::PSTR, pcchnamebuffersize: *mut u32) -> i32;
    pub fn PdhLookupPerfNameByIndexW(szmachinename: ::windows_core_sys::PCWSTR, dwnameindex: u32, sznamebuffer: ::windows_core_sys::PWSTR, pcchnamebuffersize: *mut u32) -> i32;
    pub fn PdhMakeCounterPathA(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_A, szfullpathbuffer: ::windows_core_sys::PSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
    pub fn PdhMakeCounterPathW(pcounterpathelements: *const PDH_COUNTER_PATH_ELEMENTS_W, szfullpathbuffer: ::windows_core_sys::PWSTR, pcchbuffersize: *mut u32, dwflags: PDH_PATH_FLAGS) -> i32;
    pub fn PdhOpenLogA(szlogfilename: ::windows_core_sys::PCSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: ::windows_core_sys::PCSTR, phlog: *mut isize) -> i32;
    pub fn PdhOpenLogW(szlogfilename: ::windows_core_sys::PCWSTR, dwaccessflags: PDH_LOG, lpdwlogtype: *mut PDH_LOG_TYPE, hquery: isize, dwmaxsize: u32, szusercaption: ::windows_core_sys::PCWSTR, phlog: *mut isize) -> i32;
    pub fn PdhOpenQueryA(szdatasource: ::windows_core_sys::PCSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
    pub fn PdhOpenQueryH(hdatasource: isize, dwuserdata: usize, phquery: *mut isize) -> i32;
    pub fn PdhOpenQueryW(szdatasource: ::windows_core_sys::PCWSTR, dwuserdata: usize, phquery: *mut isize) -> i32;
    pub fn PdhParseCounterPathA(szfullpathbuffer: ::windows_core_sys::PCSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_A, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
    pub fn PdhParseCounterPathW(szfullpathbuffer: ::windows_core_sys::PCWSTR, pcounterpathelements: *mut PDH_COUNTER_PATH_ELEMENTS_W, pdwbuffersize: *mut u32, dwflags: u32) -> i32;
    pub fn PdhParseInstanceNameA(szinstancestring: ::windows_core_sys::PCSTR, szinstancename: ::windows_core_sys::PSTR, pcchinstancenamelength: *mut u32, szparentname: ::windows_core_sys::PSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
    pub fn PdhParseInstanceNameW(szinstancestring: ::windows_core_sys::PCWSTR, szinstancename: ::windows_core_sys::PWSTR, pcchinstancenamelength: *mut u32, szparentname: ::windows_core_sys::PWSTR, pcchparentnamelength: *mut u32, lpindex: *mut u32) -> i32;
    pub fn PdhReadRawLogRecord(hlog: isize, ftrecord: ::win32_foundation_sys::FILETIME, prawlogrecord: *mut PDH_RAW_LOG_RECORD, pdwbufferlength: *mut u32) -> i32;
    pub fn PdhRemoveCounter(hcounter: isize) -> i32;
    pub fn PdhSelectDataSourceA(hwndowner: ::win32_foundation_sys::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: ::windows_core_sys::PSTR, pcchbufferlength: *mut u32) -> i32;
    pub fn PdhSelectDataSourceW(hwndowner: ::win32_foundation_sys::HWND, dwflags: PDH_SELECT_DATA_SOURCE_FLAGS, szdatasource: ::windows_core_sys::PWSTR, pcchbufferlength: *mut u32) -> i32;
    pub fn PdhSetCounterScaleFactor(hcounter: isize, lfactor: i32) -> i32;
    pub fn PdhSetDefaultRealTimeDataSource(dwdatasourceid: REAL_TIME_DATA_SOURCE_ID_FLAGS) -> i32;
    pub fn PdhSetLogSetRunID(hlog: isize, runid: i32) -> i32;
    pub fn PdhSetQueryTimeRange(hquery: isize, pinfo: *const PDH_TIME_INFO) -> i32;
    pub fn PdhUpdateLogA(hlog: isize, szuserstring: ::windows_core_sys::PCSTR) -> i32;
    pub fn PdhUpdateLogFileCatalog(hlog: isize) -> i32;
    pub fn PdhUpdateLogW(hlog: isize, szuserstring: ::windows_core_sys::PCWSTR) -> i32;
    pub fn PdhValidatePathA(szfullpathbuffer: ::windows_core_sys::PCSTR) -> i32;
    pub fn PdhValidatePathExA(hdatasource: isize, szfullpathbuffer: ::windows_core_sys::PCSTR) -> i32;
    pub fn PdhValidatePathExW(hdatasource: isize, szfullpathbuffer: ::windows_core_sys::PCWSTR) -> i32;
    pub fn PdhValidatePathW(szfullpathbuffer: ::windows_core_sys::PCWSTR) -> i32;
    pub fn PdhVerifySQLDBA(szdatasource: ::windows_core_sys::PCSTR) -> i32;
    pub fn PdhVerifySQLDBW(szdatasource: ::windows_core_sys::PCWSTR) -> i32;
    pub fn PerfAddCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
    pub fn PerfCloseQueryHandle(hquery: ::win32_foundation_sys::HANDLE) -> u32;
    pub fn PerfCreateInstance(providerhandle: PerfProviderHandle, countersetguid: *const ::windows_core_sys::GUID, name: ::windows_core_sys::PCWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
    pub fn PerfDecrementULongCounterValue(provider: ::win32_foundation_sys::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
    pub fn PerfDecrementULongLongCounterValue(provider: ::win32_foundation_sys::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
    pub fn PerfDeleteCounters(hquery: PerfQueryHandle, pcounters: *const PERF_COUNTER_IDENTIFIER, cbcounters: u32) -> u32;
    pub fn PerfDeleteInstance(provider: PerfProviderHandle, instanceblock: *const PERF_COUNTERSET_INSTANCE) -> u32;
    pub fn PerfEnumerateCounterSet(szmachine: ::windows_core_sys::PCWSTR, pcountersetids: *mut ::windows_core_sys::GUID, ccountersetids: u32, pccountersetidsactual: *mut u32) -> u32;
    pub fn PerfEnumerateCounterSetInstances(szmachine: ::windows_core_sys::PCWSTR, pcountersetid: *const ::windows_core_sys::GUID, pinstances: *mut PERF_INSTANCE_HEADER, cbinstances: u32, pcbinstancesactual: *mut u32) -> u32;
    pub fn PerfIncrementULongCounterValue(provider: ::win32_foundation_sys::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
    pub fn PerfIncrementULongLongCounterValue(provider: ::win32_foundation_sys::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
    pub fn PerfOpenQueryHandle(szmachine: ::windows_core_sys::PCWSTR, phquery: *mut PerfQueryHandle) -> u32;
    pub fn PerfQueryCounterData(hquery: PerfQueryHandle, pcounterblock: *mut PERF_DATA_HEADER, cbcounterblock: u32, pcbcounterblockactual: *mut u32) -> u32;
    pub fn PerfQueryCounterInfo(hquery: PerfQueryHandle, pcounters: *mut PERF_COUNTER_IDENTIFIER, cbcounters: u32, pcbcountersactual: *mut u32) -> u32;
    pub fn PerfQueryCounterSetRegistrationInfo(szmachine: ::windows_core_sys::PCWSTR, pcountersetid: *const ::windows_core_sys::GUID, requestcode: PerfRegInfoType, requestlangid: u32, pbreginfo: *mut u8, cbreginfo: u32, pcbreginfoactual: *mut u32) -> u32;
    pub fn PerfQueryInstance(providerhandle: ::win32_foundation_sys::HANDLE, countersetguid: *const ::windows_core_sys::GUID, name: ::windows_core_sys::PCWSTR, id: u32) -> *mut PERF_COUNTERSET_INSTANCE;
    pub fn PerfSetCounterRefValue(provider: ::win32_foundation_sys::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, address: *const ::core::ffi::c_void) -> u32;
    pub fn PerfSetCounterSetInfo(providerhandle: ::win32_foundation_sys::HANDLE, template: *mut PERF_COUNTERSET_INFO, templatesize: u32) -> u32;
    pub fn PerfSetULongCounterValue(provider: ::win32_foundation_sys::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u32) -> u32;
    pub fn PerfSetULongLongCounterValue(provider: ::win32_foundation_sys::HANDLE, instance: *mut PERF_COUNTERSET_INSTANCE, counterid: u32, value: u64) -> u32;
    pub fn PerfStartProvider(providerguid: *const ::windows_core_sys::GUID, controlcallback: PERFLIBREQUEST, phprovider: *mut PerfProviderHandle) -> u32;
    pub fn PerfStartProviderEx(providerguid: *const ::windows_core_sys::GUID, providercontext: *const PERF_PROVIDER_CONTEXT, provider: *mut PerfProviderHandle) -> u32;
    pub fn PerfStopProvider(providerhandle: PerfProviderHandle) -> u32;
    pub fn QueryPerformanceCounter(lpperformancecount: *mut i64) -> ::win32_foundation_sys::BOOL;
    pub fn QueryPerformanceFrequency(lpfrequency: *mut i64) -> ::win32_foundation_sys::BOOL;
    pub fn RestorePerfRegistryFromFileW(szfilename: ::windows_core_sys::PCWSTR, szlangid: ::windows_core_sys::PCWSTR) -> u32;
    pub fn SetServiceAsTrustedA(szreserved: ::windows_core_sys::PCSTR, szservicename: ::windows_core_sys::PCSTR) -> u32;
    pub fn SetServiceAsTrustedW(szreserved: ::windows_core_sys::PCWSTR, szservicename: ::windows_core_sys::PCWSTR) -> u32;
    pub fn UnloadPerfCounterTextStringsA(lpcommandline: ::windows_core_sys::PCSTR, bquietmodearg: ::win32_foundation_sys::BOOL) -> u32;
    pub fn UnloadPerfCounterTextStringsW(lpcommandline: ::windows_core_sys::PCWSTR, bquietmodearg: ::win32_foundation_sys::BOOL) -> u32;
    pub fn UpdatePerfNameFilesA(sznewctrfilepath: ::windows_core_sys::PCSTR, sznewhlpfilepath: ::windows_core_sys::PCSTR, szlanguageid: ::windows_core_sys::PCSTR, dwflags: usize) -> u32;
    pub fn UpdatePerfNameFilesW(sznewctrfilepath: ::windows_core_sys::PCWSTR, sznewhlpfilepath: ::windows_core_sys::PCWSTR, szlanguageid: ::windows_core_sys::PCWSTR, dwflags: usize) -> u32;
}
pub const AppearPropPage: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3835118057, data2: 37800, data3: 19121, data4: [142, 150, 191, 68, 130, 40, 46, 156] };
pub type AutoPathFormat = i32;
pub const plaNone: AutoPathFormat = 0i32;
pub const plaPattern: AutoPathFormat = 1i32;
pub const plaComputer: AutoPathFormat = 2i32;
pub const plaMonthDayHour: AutoPathFormat = 256i32;
pub const plaSerialNumber: AutoPathFormat = 512i32;
pub const plaYearDayOfYear: AutoPathFormat = 1024i32;
pub const plaYearMonth: AutoPathFormat = 2048i32;
pub const plaYearMonthDay: AutoPathFormat = 4096i32;
pub const plaYearMonthDayHour: AutoPathFormat = 8192i32;
pub const plaMonthDayHourMinute: AutoPathFormat = 16384i32;
pub const BootTraceSession: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946872, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const BootTraceSessionCollection: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946873, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub type ClockType = i32;
pub const plaTimeStamp: ClockType = 0i32;
pub const plaPerformance: ClockType = 1i32;
pub const plaSystem: ClockType = 2i32;
pub const plaCycle: ClockType = 3i32;
pub type CommitMode = i32;
pub const plaCreateNew: CommitMode = 1i32;
pub const plaModify: CommitMode = 2i32;
pub const plaCreateOrModify: CommitMode = 3i32;
pub const plaUpdateRunningInstance: CommitMode = 16i32;
pub const plaFlushTrace: CommitMode = 32i32;
pub const plaValidateOnly: CommitMode = 4096i32;
pub const CounterItem: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3302152416, data2: 53725, data3: 4558, data4: [148, 15, 0, 128, 41, 0, 67, 72] };
pub const CounterItem2: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 1125739618, data2: 49951, data3: 19683, data4: [160, 46, 121, 239, 224, 246, 165, 37] };
pub type CounterPathCallBack = ::core::option::Option<unsafe extern "system" fn(param0: usize) -> i32>;
pub const CounterPropPage: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3482617185, data2: 60904, data3: 4558, data4: [148, 30, 0, 128, 41, 0, 67, 71] };
pub const Counters: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2997905106, data2: 10924, data3: 4559, data4: [148, 47, 0, 128, 41, 0, 67, 71] };
pub type DICounterItem = *mut ::core::ffi::c_void;
pub const DIID_DICounterItem: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3230420978, data2: 3630, data3: 4559, data4: [148, 44, 0, 128, 41, 0, 67, 71] };
pub const DIID_DILogFileItem: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2366193660, data2: 63351, data3: 18711, data4: [130, 209, 131, 63, 188, 84, 197, 143] };
pub const DIID_DISystemMonitor: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 332873089, data2: 49966, data3: 4559, data4: [147, 152, 0, 170, 0, 163, 221, 234] };
pub const DIID_DISystemMonitorEvents: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2224527664, data2: 19123, data3: 4559, data4: [148, 58, 0, 128, 41, 0, 67, 71] };
pub const DIID_DISystemMonitorInternal: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 424587842, data2: 49964, data3: 4559, data4: [147, 152, 0, 170, 0, 163, 221, 234] };
pub type DILogFileItem = *mut ::core::ffi::c_void;
pub type DISystemMonitor = *mut ::core::ffi::c_void;
pub type DISystemMonitorEvents = *mut ::core::ffi::c_void;
pub type DISystemMonitorInternal = *mut ::core::ffi::c_void;
pub const DataCollectorSet: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946849, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const DataCollectorSetCollection: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946853, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub type DataCollectorSetStatus = i32;
pub const plaStopped: DataCollectorSetStatus = 0i32;
pub const plaRunning: DataCollectorSetStatus = 1i32;
pub const plaCompiling: DataCollectorSetStatus = 2i32;
pub const plaPending: DataCollectorSetStatus = 3i32;
pub const plaUndefined: DataCollectorSetStatus = 4i32;
pub type DataCollectorType = i32;
pub const plaPerformanceCounter: DataCollectorType = 0i32;
pub const plaTrace: DataCollectorType = 1i32;
pub const plaConfiguration: DataCollectorType = 2i32;
pub const plaAlert: DataCollectorType = 3i32;
pub const plaApiTrace: DataCollectorType = 4i32;
pub type DataManagerSteps = i32;
pub const plaCreateReport: DataManagerSteps = 1i32;
pub const plaRunRules: DataManagerSteps = 2i32;
pub const plaCreateHtml: DataManagerSteps = 4i32;
pub const plaFolderActions: DataManagerSteps = 8i32;
pub const plaResourceFreeing: DataManagerSteps = 16i32;
pub type DataSourceTypeConstants = i32;
pub const sysmonNullDataSource: DataSourceTypeConstants = -1i32;
pub const sysmonCurrentActivity: DataSourceTypeConstants = 1i32;
pub const sysmonLogFiles: DataSourceTypeConstants = 2i32;
pub const sysmonSqlLog: DataSourceTypeConstants = 3i32;
pub type DisplayTypeConstants = i32;
pub const sysmonLineGraph: DisplayTypeConstants = 1i32;
pub const sysmonHistogram: DisplayTypeConstants = 2i32;
pub const sysmonReport: DisplayTypeConstants = 3i32;
pub const sysmonChartArea: DisplayTypeConstants = 4i32;
pub const sysmonChartStackedArea: DisplayTypeConstants = 5i32;
pub type FileFormat = i32;
pub const plaCommaSeparated: FileFormat = 0i32;
pub const plaTabSeparated: FileFormat = 1i32;
pub const plaSql: FileFormat = 2i32;
pub const plaBinary: FileFormat = 3i32;
pub type FolderActionSteps = i32;
pub const plaCreateCab: FolderActionSteps = 1i32;
pub const plaDeleteData: FolderActionSteps = 2i32;
pub const plaSendCab: FolderActionSteps = 4i32;
pub const plaDeleteCab: FolderActionSteps = 8i32;
pub const plaDeleteReport: FolderActionSteps = 16i32;
pub const GeneralPropPage: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3286619090, data2: 6659, data3: 4559, data4: [148, 45, 0, 128, 41, 0, 67, 71] };
pub const GraphPropPage: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3286619091, data2: 6659, data3: 4559, data4: [148, 45, 0, 128, 41, 0, 67, 71] };
pub const H_WBEM_DATASOURCE: i32 = -1i32;
pub type IAlertDataCollector = *mut ::core::ffi::c_void;
pub type IApiTracingDataCollector = *mut ::core::ffi::c_void;
pub type IConfigurationDataCollector = *mut ::core::ffi::c_void;
pub type ICounterItem = *mut ::core::ffi::c_void;
pub type ICounterItem2 = *mut ::core::ffi::c_void;
pub type ICounters = *mut ::core::ffi::c_void;
pub type IDataCollector = *mut ::core::ffi::c_void;
pub type IDataCollectorCollection = *mut ::core::ffi::c_void;
pub type IDataCollectorSet = *mut ::core::ffi::c_void;
pub type IDataCollectorSetCollection = *mut ::core::ffi::c_void;
pub type IDataManager = *mut ::core::ffi::c_void;
pub type IFolderAction = *mut ::core::ffi::c_void;
pub type IFolderActionCollection = *mut ::core::ffi::c_void;
pub type ILogFileItem = *mut ::core::ffi::c_void;
pub type ILogFiles = *mut ::core::ffi::c_void;
pub type IPerformanceCounterDataCollector = *mut ::core::ffi::c_void;
pub type ISchedule = *mut ::core::ffi::c_void;
pub type IScheduleCollection = *mut ::core::ffi::c_void;
pub type ISystemMonitor = *mut ::core::ffi::c_void;
pub type ISystemMonitor2 = *mut ::core::ffi::c_void;
pub type ISystemMonitorEvents = *mut ::core::ffi::c_void;
pub type ITraceDataCollector = *mut ::core::ffi::c_void;
pub type ITraceDataProvider = *mut ::core::ffi::c_void;
pub type ITraceDataProviderCollection = *mut ::core::ffi::c_void;
pub type IValueMap = *mut ::core::ffi::c_void;
pub type IValueMapItem = *mut ::core::ffi::c_void;
pub const LIBID_SystemMonitor: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 460799554, data2: 9481, data3: 4559, data4: [148, 47, 0, 128, 41, 0, 67, 71] };
pub const LegacyDataCollectorSet: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946854, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const LegacyDataCollectorSetCollection: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946855, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const LegacyTraceSession: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946856, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const LegacyTraceSessionCollection: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946857, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const LogFileItem: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 384588776, data2: 57235, data3: 16951, data4: [148, 228, 158, 233, 24, 17, 29, 113] };
pub const LogFiles: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 657840637, data2: 63161, data3: 20249, data4: [165, 217, 226, 208, 104, 88, 75, 197] };
pub const MAX_COUNTER_PATH: u32 = 256u32;
pub const MAX_PERF_OBJECTS_IN_QUERY_FUNCTION: i32 = 64i32;
pub const PDH_ACCESS_DENIED: i32 = -1073738789i32;
pub const PDH_ASYNC_QUERY_TIMEOUT: i32 = -2147481637i32;
pub const PDH_BINARY_LOG_CORRUPT: i32 = -1073738761i32;
#[repr(C)]
pub struct PDH_BROWSE_DLG_CONFIG_A {
    pub _bitfield: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub szDataSource: ::windows_core_sys::PSTR,
    pub szReturnPathBuffer: ::windows_core_sys::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_A {}
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_BROWSE_DLG_CONFIG_HA {
    pub _bitfield: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub hDataSource: isize,
    pub szReturnPathBuffer: ::windows_core_sys::PSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_HA {}
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_HA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_BROWSE_DLG_CONFIG_HW {
    pub _bitfield: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub hDataSource: isize,
    pub szReturnPathBuffer: ::windows_core_sys::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_HW {}
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_HW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_BROWSE_DLG_CONFIG_W {
    pub _bitfield: u32,
    pub hWndOwner: ::win32_foundation_sys::HWND,
    pub szDataSource: ::windows_core_sys::PWSTR,
    pub szReturnPathBuffer: ::windows_core_sys::PWSTR,
    pub cchReturnPathLength: u32,
    pub pCallBack: CounterPathCallBack,
    pub dwCallBackArg: usize,
    pub CallBackStatus: i32,
    pub dwDefaultDetailLevel: PERF_DETAIL,
    pub szDialogBoxCaption: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for PDH_BROWSE_DLG_CONFIG_W {}
impl ::core::clone::Clone for PDH_BROWSE_DLG_CONFIG_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PDH_CALC_NEGATIVE_DENOMINATOR: i32 = -2147481642i32;
pub const PDH_CALC_NEGATIVE_TIMEBASE: i32 = -2147481641i32;
pub const PDH_CALC_NEGATIVE_VALUE: i32 = -2147481640i32;
pub const PDH_CANNOT_CONNECT_MACHINE: i32 = -1073738813i32;
pub const PDH_CANNOT_CONNECT_WMI_SERVER: i32 = -1073738776i32;
pub const PDH_CANNOT_READ_NAME_STRINGS: i32 = -1073738808i32;
pub const PDH_CANNOT_SET_DEFAULT_REALTIME_DATASOURCE: i32 = -2147481636i32;
pub const PDH_COUNTER_ALREADY_IN_QUERY: i32 = -1073738762i32;
#[repr(C)]
pub struct PDH_COUNTER_INFO_A {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: ::windows_core_sys::PSTR,
    pub Anonymous: PDH_COUNTER_INFO_A_0,
    pub szExplainText: ::windows_core_sys::PSTR,
    pub DataBuffer: [u32; 1],
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_A {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union PDH_COUNTER_INFO_A_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_A,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_A,
    pub Anonymous: PDH_COUNTER_INFO_A_0_0,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_A_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_COUNTER_INFO_A_0_0 {
    pub szMachineName: ::windows_core_sys::PSTR,
    pub szObjectName: ::windows_core_sys::PSTR,
    pub szInstanceName: ::windows_core_sys::PSTR,
    pub szParentInstance: ::windows_core_sys::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_A_0_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_A_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_COUNTER_INFO_W {
    pub dwLength: u32,
    pub dwType: u32,
    pub CVersion: u32,
    pub CStatus: u32,
    pub lScale: i32,
    pub lDefaultScale: i32,
    pub dwUserData: usize,
    pub dwQueryUserData: usize,
    pub szFullPath: ::windows_core_sys::PWSTR,
    pub Anonymous: PDH_COUNTER_INFO_W_0,
    pub szExplainText: ::windows_core_sys::PWSTR,
    pub DataBuffer: [u32; 1],
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_W {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union PDH_COUNTER_INFO_W_0 {
    pub DataItemPath: PDH_DATA_ITEM_PATH_ELEMENTS_W,
    pub CounterPath: PDH_COUNTER_PATH_ELEMENTS_W,
    pub Anonymous: PDH_COUNTER_INFO_W_0_0,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_W_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_COUNTER_INFO_W_0_0 {
    pub szMachineName: ::windows_core_sys::PWSTR,
    pub szObjectName: ::windows_core_sys::PWSTR,
    pub szInstanceName: ::windows_core_sys::PWSTR,
    pub szParentInstance: ::windows_core_sys::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_INFO_W_0_0 {}
impl ::core::clone::Clone for PDH_COUNTER_INFO_W_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_COUNTER_PATH_ELEMENTS_A {
    pub szMachineName: ::windows_core_sys::PSTR,
    pub szObjectName: ::windows_core_sys::PSTR,
    pub szInstanceName: ::windows_core_sys::PSTR,
    pub szParentInstance: ::windows_core_sys::PSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_PATH_ELEMENTS_A {}
impl ::core::clone::Clone for PDH_COUNTER_PATH_ELEMENTS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_COUNTER_PATH_ELEMENTS_W {
    pub szMachineName: ::windows_core_sys::PWSTR,
    pub szObjectName: ::windows_core_sys::PWSTR,
    pub szInstanceName: ::windows_core_sys::PWSTR,
    pub szParentInstance: ::windows_core_sys::PWSTR,
    pub dwInstanceIndex: u32,
    pub szCounterName: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for PDH_COUNTER_PATH_ELEMENTS_W {}
impl ::core::clone::Clone for PDH_COUNTER_PATH_ELEMENTS_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PDH_CSTATUS_BAD_COUNTERNAME: i32 = -1073738816i32;
pub const PDH_CSTATUS_INVALID_DATA: i32 = -1073738822i32;
pub const PDH_CSTATUS_ITEM_NOT_VALIDATED: i32 = -2147481645i32;
pub const PDH_CSTATUS_NEW_DATA: i32 = 1i32;
pub const PDH_CSTATUS_NO_COUNTER: i32 = -1073738823i32;
pub const PDH_CSTATUS_NO_COUNTERNAME: i32 = -1073738817i32;
pub const PDH_CSTATUS_NO_INSTANCE: i32 = -2147481647i32;
pub const PDH_CSTATUS_NO_MACHINE: i32 = -2147481648i32;
pub const PDH_CSTATUS_NO_OBJECT: i32 = -1073738824i32;
pub const PDH_CSTATUS_VALID_DATA: i32 = 0i32;
#[repr(C)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_A {
    pub szMachineName: ::windows_core_sys::PSTR,
    pub ObjectGUID: ::windows_core_sys::GUID,
    pub dwItemId: u32,
    pub szInstanceName: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for PDH_DATA_ITEM_PATH_ELEMENTS_A {}
impl ::core::clone::Clone for PDH_DATA_ITEM_PATH_ELEMENTS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_DATA_ITEM_PATH_ELEMENTS_W {
    pub szMachineName: ::windows_core_sys::PWSTR,
    pub ObjectGUID: ::windows_core_sys::GUID,
    pub dwItemId: u32,
    pub szInstanceName: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for PDH_DATA_ITEM_PATH_ELEMENTS_W {}
impl ::core::clone::Clone for PDH_DATA_ITEM_PATH_ELEMENTS_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PDH_DATA_SOURCE_IS_LOG_FILE: i32 = -1073738802i32;
pub const PDH_DATA_SOURCE_IS_REAL_TIME: i32 = -1073738801i32;
pub const PDH_DIALOG_CANCELLED: i32 = -2147481639i32;
pub type PDH_DLL_VERSION = u32;
pub const PDH_CVERSION_WIN50: PDH_DLL_VERSION = 1280u32;
pub const PDH_VERSION: PDH_DLL_VERSION = 1283u32;
pub const PDH_END_OF_LOG_FILE: i32 = -2147481638i32;
pub const PDH_ENTRY_NOT_IN_LOG_FILE: i32 = -1073738803i32;
pub const PDH_FILE_ALREADY_EXISTS: i32 = -1073738798i32;
pub const PDH_FILE_NOT_FOUND: i32 = -1073738799i32;
pub type PDH_FMT = u32;
pub const PDH_FMT_DOUBLE: PDH_FMT = 512u32;
pub const PDH_FMT_LARGE: PDH_FMT = 1024u32;
pub const PDH_FMT_LONG: PDH_FMT = 256u32;
#[repr(C)]
pub struct PDH_FMT_COUNTERVALUE {
    pub CStatus: u32,
    pub Anonymous: PDH_FMT_COUNTERVALUE_0,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union PDH_FMT_COUNTERVALUE_0 {
    pub longValue: i32,
    pub doubleValue: f64,
    pub largeValue: i64,
    pub AnsiStringValue: ::windows_core_sys::PCSTR,
    pub WideStringValue: ::windows_core_sys::PCWSTR,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_0 {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_FMT_COUNTERVALUE_ITEM_A {
    pub szName: ::windows_core_sys::PSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_ITEM_A {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_ITEM_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_FMT_COUNTERVALUE_ITEM_W {
    pub szName: ::windows_core_sys::PWSTR,
    pub FmtValue: PDH_FMT_COUNTERVALUE,
}
impl ::core::marker::Copy for PDH_FMT_COUNTERVALUE_ITEM_W {}
impl ::core::clone::Clone for PDH_FMT_COUNTERVALUE_ITEM_W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PDH_FUNCTION_NOT_FOUND: i32 = -1073738818i32;
pub const PDH_INCORRECT_APPEND_TIME: i32 = -1073738757i32;
pub const PDH_INSUFFICIENT_BUFFER: i32 = -1073738814i32;
pub const PDH_INVALID_ARGUMENT: i32 = -1073738819i32;
pub const PDH_INVALID_BUFFER: i32 = -1073738815i32;
pub const PDH_INVALID_DATA: i32 = -1073738810i32;
pub const PDH_INVALID_DATASOURCE: i32 = -1073738787i32;
pub const PDH_INVALID_HANDLE: i32 = -1073738820i32;
pub const PDH_INVALID_INSTANCE: i32 = -1073738811i32;
pub const PDH_INVALID_PATH: i32 = -1073738812i32;
pub const PDH_INVALID_SQLDB: i32 = -1073738786i32;
pub const PDH_INVALID_SQL_LOG_FORMAT: i32 = -1073738763i32;
pub type PDH_LOG = u32;
pub const PDH_LOG_READ_ACCESS: PDH_LOG = 65536u32;
pub const PDH_LOG_WRITE_ACCESS: PDH_LOG = 131072u32;
pub const PDH_LOG_UPDATE_ACCESS: PDH_LOG = 262144u32;
pub const PDH_LOGSVC_NOT_OPENED: i32 = -1073738791i32;
pub const PDH_LOGSVC_QUERY_NOT_FOUND: i32 = -1073738792i32;
pub const PDH_LOG_FILE_CREATE_ERROR: i32 = -1073738807i32;
pub const PDH_LOG_FILE_OPEN_ERROR: i32 = -1073738806i32;
pub const PDH_LOG_FILE_TOO_SMALL: i32 = -1073738788i32;
pub const PDH_LOG_SAMPLE_TOO_SMALL: i32 = -1073738760i32;
#[repr(C)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: ::windows_core_sys::PSTR,
    pub szDefaultDir: ::windows_core_sys::PSTR,
    pub szBaseFileName: ::windows_core_sys::PSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_A_0,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    pub Anonymous1: PDH_LOG_SERVICE_QUERY_INFO_A_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_A_0_1,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0 {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: ::windows_core_sys::PSTR,
    pub PdlCounterList: ::windows_core_sys::PSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: ::win32_foundation_sys::FILETIME,
    pub PdlLogEndTime: ::win32_foundation_sys::FILETIME,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: ::windows_core_sys::PSTR,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_A_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwLogQuota: u32,
    pub szLogFileCaption: ::windows_core_sys::PWSTR,
    pub szDefaultDir: ::windows_core_sys::PWSTR,
    pub szBaseFileName: ::windows_core_sys::PWSTR,
    pub dwFileType: u32,
    pub dwReserved: u32,
    pub Anonymous: PDH_LOG_SERVICE_QUERY_INFO_W_0,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    pub Anonymous1: PDH_LOG_SERVICE_QUERY_INFO_W_0_0,
    pub Anonymous2: PDH_LOG_SERVICE_QUERY_INFO_W_0_1,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0 {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    pub PdlAutoNameInterval: u32,
    pub PdlAutoNameUnits: u32,
    pub PdlCommandFilename: ::windows_core_sys::PWSTR,
    pub PdlCounterList: ::windows_core_sys::PWSTR,
    pub PdlAutoNameFormat: u32,
    pub PdlSampleInterval: u32,
    pub PdlLogStartTime: ::win32_foundation_sys::FILETIME,
    pub PdlLogEndTime: ::win32_foundation_sys::FILETIME,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    pub TlNumberOfBuffers: u32,
    pub TlMinimumBuffers: u32,
    pub TlMaximumBuffers: u32,
    pub TlFreeBuffers: u32,
    pub TlBufferSize: u32,
    pub TlEventsLost: u32,
    pub TlLoggerThreadId: u32,
    pub TlBuffersWritten: u32,
    pub TlLogHandle: u32,
    pub TlLogFileName: ::windows_core_sys::PWSTR,
}
impl ::core::marker::Copy for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {}
impl ::core::clone::Clone for PDH_LOG_SERVICE_QUERY_INFO_W_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PDH_LOG_TYPE = u32;
pub const PDH_LOG_TYPE_UNDEFINED: PDH_LOG_TYPE = 0u32;
pub const PDH_LOG_TYPE_CSV: PDH_LOG_TYPE = 1u32;
pub const PDH_LOG_TYPE_SQL: PDH_LOG_TYPE = 7u32;
pub const PDH_LOG_TYPE_TSV: PDH_LOG_TYPE = 2u32;
pub const PDH_LOG_TYPE_BINARY: PDH_LOG_TYPE = 8u32;
pub const PDH_LOG_TYPE_PERFMON: PDH_LOG_TYPE = 6u32;
pub const PDH_LOG_TYPE_NOT_FOUND: i32 = -1073738805i32;
pub const PDH_LOG_TYPE_RETIRED_BIN: u32 = 3u32;
pub const PDH_LOG_TYPE_TRACE_GENERIC: u32 = 5u32;
pub const PDH_LOG_TYPE_TRACE_KERNEL: u32 = 4u32;
pub const PDH_MAX_COUNTER_NAME: u32 = 1024u32;
pub const PDH_MAX_COUNTER_PATH: u32 = 2048u32;
pub const PDH_MAX_DATASOURCE_PATH: u32 = 1024u32;
pub const PDH_MAX_INSTANCE_NAME: u32 = 1024u32;
pub const PDH_MAX_SCALE: i32 = 7i32;
pub const PDH_MEMORY_ALLOCATION_FAILURE: i32 = -1073738821i32;
pub const PDH_MIN_SCALE: i32 = -7i32;
pub const PDH_MORE_DATA: i32 = -2147481646i32;
pub const PDH_NOEXPANDCOUNTERS: u32 = 1u32;
pub const PDH_NOEXPANDINSTANCES: u32 = 2u32;
pub const PDH_NOT_IMPLEMENTED: i32 = -1073738797i32;
pub const PDH_NO_COUNTERS: i32 = -1073738785i32;
pub const PDH_NO_DATA: i32 = -2147481643i32;
pub const PDH_NO_DIALOG_DATA: i32 = -1073738809i32;
pub const PDH_NO_MORE_DATA: i32 = -1073738804i32;
pub const PDH_OS_EARLIER_VERSION: i32 = -1073738758i32;
pub const PDH_OS_LATER_VERSION: i32 = -1073738759i32;
pub type PDH_PATH_FLAGS = u32;
pub const PDH_PATH_WBEM_RESULT: PDH_PATH_FLAGS = 1u32;
pub const PDH_PATH_WBEM_INPUT: PDH_PATH_FLAGS = 2u32;
pub const PDH_PATH_WBEM_NONE: PDH_PATH_FLAGS = 0u32;
pub const PDH_PLA_COLLECTION_ALREADY_RUNNING: i32 = -1073738775i32;
pub const PDH_PLA_COLLECTION_NOT_FOUND: i32 = -1073738773i32;
pub const PDH_PLA_ERROR_ALREADY_EXISTS: i32 = -1073738770i32;
pub const PDH_PLA_ERROR_FILEPATH: i32 = -1073738768i32;
pub const PDH_PLA_ERROR_NAME_TOO_LONG: i32 = -1073738764i32;
pub const PDH_PLA_ERROR_NOSTART: i32 = -1073738771i32;
pub const PDH_PLA_ERROR_SCHEDULE_ELAPSED: i32 = -1073738772i32;
pub const PDH_PLA_ERROR_SCHEDULE_OVERLAP: i32 = -1073738774i32;
pub const PDH_PLA_ERROR_TYPE_MISMATCH: i32 = -1073738769i32;
pub const PDH_PLA_SERVICE_ERROR: i32 = -1073738767i32;
pub const PDH_PLA_VALIDATION_ERROR: i32 = -1073738766i32;
pub const PDH_PLA_VALIDATION_WARNING: i32 = -2147480589i32;
pub const PDH_QUERY_PERF_DATA_TIMEOUT: i32 = -1073738754i32;
#[repr(C)]
pub struct PDH_RAW_COUNTER {
    pub CStatus: u32,
    pub TimeStamp: ::win32_foundation_sys::FILETIME,
    pub FirstValue: i64,
    pub SecondValue: i64,
    pub MultiCount: u32,
}
impl ::core::marker::Copy for PDH_RAW_COUNTER {}
impl ::core::clone::Clone for PDH_RAW_COUNTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_RAW_COUNTER_ITEM_A {
    pub szName: ::windows_core_sys::PSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
impl ::core::marker::Copy for PDH_RAW_COUNTER_ITEM_A {}
impl ::core::clone::Clone for PDH_RAW_COUNTER_ITEM_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_RAW_COUNTER_ITEM_W {
    pub szName: ::windows_core_sys::PWSTR,
    pub RawValue: PDH_RAW_COUNTER,
}
impl ::core::marker::Copy for PDH_RAW_COUNTER_ITEM_W {}
impl ::core::clone::Clone for PDH_RAW_COUNTER_ITEM_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PDH_RAW_LOG_RECORD {
    pub dwStructureSize: u32,
    pub dwRecordType: PDH_LOG_TYPE,
    pub dwItems: u32,
    pub RawBytes: [u8; 1],
}
impl ::core::marker::Copy for PDH_RAW_LOG_RECORD {}
impl ::core::clone::Clone for PDH_RAW_LOG_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PDH_REFRESHCOUNTERS: u32 = 4u32;
pub const PDH_RETRY: i32 = -2147481644i32;
pub type PDH_SELECT_DATA_SOURCE_FLAGS = u32;
pub const PDH_FLAGS_FILE_BROWSER_ONLY: PDH_SELECT_DATA_SOURCE_FLAGS = 1u32;
pub const PDH_FLAGS_NONE: PDH_SELECT_DATA_SOURCE_FLAGS = 0u32;
pub const PDH_SQL_ALLOCCON_FAILED: i32 = -1073738783i32;
pub const PDH_SQL_ALLOC_FAILED: i32 = -1073738784i32;
pub const PDH_SQL_ALTER_DETAIL_FAILED: i32 = -1073738755i32;
pub const PDH_SQL_BIND_FAILED: i32 = -1073738777i32;
pub const PDH_SQL_CONNECT_FAILED: i32 = -1073738778i32;
pub const PDH_SQL_EXEC_DIRECT_FAILED: i32 = -1073738782i32;
pub const PDH_SQL_FETCH_FAILED: i32 = -1073738781i32;
pub const PDH_SQL_MORE_RESULTS_FAILED: i32 = -1073738779i32;
pub const PDH_SQL_ROWCOUNT_FAILED: i32 = -1073738780i32;
#[repr(C)]
pub struct PDH_STATISTICS {
    pub dwFormat: u32,
    pub count: u32,
    pub min: PDH_FMT_COUNTERVALUE,
    pub max: PDH_FMT_COUNTERVALUE,
    pub mean: PDH_FMT_COUNTERVALUE,
}
impl ::core::marker::Copy for PDH_STATISTICS {}
impl ::core::clone::Clone for PDH_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PDH_STRING_NOT_FOUND: i32 = -1073738796i32;
#[repr(C)]
pub struct PDH_TIME_INFO {
    pub StartTime: i64,
    pub EndTime: i64,
    pub SampleCount: u32,
}
impl ::core::marker::Copy for PDH_TIME_INFO {}
impl ::core::clone::Clone for PDH_TIME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PDH_UNABLE_MAP_NAME_FILES: i32 = -2147480619i32;
pub const PDH_UNABLE_READ_LOG_HEADER: i32 = -1073738800i32;
pub const PDH_UNKNOWN_LOGSVC_COMMAND: i32 = -1073738793i32;
pub const PDH_UNKNOWN_LOG_FORMAT: i32 = -1073738794i32;
pub const PDH_UNMATCHED_APPEND_COUNTER: i32 = -1073738756i32;
pub const PDH_WBEM_ERROR: i32 = -1073738790i32;
pub type PERFLIBREQUEST = ::core::option::Option<unsafe extern "system" fn(requestcode: u32, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32>;
pub const PERF_ADD_COUNTER: u32 = 1u32;
pub const PERF_AGGREGATE_INSTANCE: &str = "_Total";
pub const PERF_AGGREGATE_MAX: u32 = 4u32;
pub const PERF_ATTRIB_BY_REFERENCE: u64 = 1u64;
pub const PERF_ATTRIB_DISPLAY_AS_HEX: u64 = 16u64;
pub const PERF_ATTRIB_DISPLAY_AS_REAL: u64 = 8u64;
pub const PERF_ATTRIB_NO_DISPLAYABLE: u64 = 2u64;
pub const PERF_ATTRIB_NO_GROUP_SEPARATOR: u64 = 4u64;
pub const PERF_COLLECT_END: u32 = 6u32;
pub const PERF_COLLECT_START: u32 = 5u32;
pub const PERF_COUNTERSET_FLAG_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_FLAG_HISTORY: u32 = 8u32;
pub const PERF_COUNTERSET_FLAG_INSTANCE: u32 = 16u32;
pub const PERF_COUNTERSET_FLAG_MULTIPLE: u32 = 2u32;
#[repr(C)]
pub struct PERF_COUNTERSET_INFO {
    pub CounterSetGuid: ::windows_core_sys::GUID,
    pub ProviderGuid: ::windows_core_sys::GUID,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_INFO {}
impl ::core::clone::Clone for PERF_COUNTERSET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PERF_COUNTERSET_INSTANCE {
    pub CounterSetGuid: ::windows_core_sys::GUID,
    pub dwSize: u32,
    pub InstanceId: u32,
    pub InstanceNameOffset: u32,
    pub InstanceNameSize: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_INSTANCE {}
impl ::core::clone::Clone for PERF_COUNTERSET_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PERF_COUNTERSET_MULTI_INSTANCES: u32 = 2u32;
#[repr(C)]
pub struct PERF_COUNTERSET_REG_INFO {
    pub CounterSetGuid: ::windows_core_sys::GUID,
    pub CounterSetType: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub InstanceType: u32,
}
impl ::core::marker::Copy for PERF_COUNTERSET_REG_INFO {}
impl ::core::clone::Clone for PERF_COUNTERSET_REG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PERF_COUNTERSET_SINGLE_AGGREGATE: u32 = 4u32;
pub const PERF_COUNTERSET_SINGLE_INSTANCE: u32 = 0u32;
pub type PERF_COUNTER_AGGREGATE_FUNC = u32;
pub const PERF_AGGREGATE_UNDEFINED: PERF_COUNTER_AGGREGATE_FUNC = 0u32;
pub const PERF_AGGREGATE_TOTAL: PERF_COUNTER_AGGREGATE_FUNC = 1u32;
pub const PERF_AGGREGATE_AVG: PERF_COUNTER_AGGREGATE_FUNC = 2u32;
pub const PERF_AGGREGATE_MIN: PERF_COUNTER_AGGREGATE_FUNC = 3u32;
pub const PERF_COUNTER_BASE: u32 = 196608u32;
#[repr(C)]
pub struct PERF_COUNTER_BLOCK {
    pub ByteLength: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_BLOCK {}
impl ::core::clone::Clone for PERF_COUNTER_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PERF_COUNTER_DATA {
    pub dwDataSize: u32,
    pub dwSize: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_DATA {}
impl ::core::clone::Clone for PERF_COUNTER_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: u32,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: u32,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for PERF_COUNTER_DEFINITION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for PERF_COUNTER_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
pub struct PERF_COUNTER_DEFINITION {
    pub ByteLength: u32,
    pub CounterNameTitleIndex: u32,
    pub CounterNameTitle: ::windows_core_sys::PWSTR,
    pub CounterHelpTitleIndex: u32,
    pub CounterHelpTitle: ::windows_core_sys::PWSTR,
    pub DefaultScale: i32,
    pub DetailLevel: u32,
    pub CounterType: u32,
    pub CounterSize: u32,
    pub CounterOffset: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for PERF_COUNTER_DEFINITION {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for PERF_COUNTER_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PERF_COUNTER_ELAPSED: u32 = 262144u32;
pub const PERF_COUNTER_FRACTION: u32 = 131072u32;
#[repr(C)]
pub struct PERF_COUNTER_HEADER {
    pub dwStatus: u32,
    pub dwType: PerfCounterDataType,
    pub dwSize: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_HEADER {}
impl ::core::clone::Clone for PERF_COUNTER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PERF_COUNTER_HISTOGRAM: u32 = 393216u32;
pub const PERF_COUNTER_HISTOGRAM_TYPE: u32 = 2147483648u32;
#[repr(C)]
pub struct PERF_COUNTER_IDENTIFIER {
    pub CounterSetGuid: ::windows_core_sys::GUID,
    pub Status: u32,
    pub Size: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub Index: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_IDENTIFIER {}
impl ::core::clone::Clone for PERF_COUNTER_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PERF_COUNTER_IDENTITY {
    pub CounterSetGuid: ::windows_core_sys::GUID,
    pub BufferSize: u32,
    pub CounterId: u32,
    pub InstanceId: u32,
    pub MachineOffset: u32,
    pub NameOffset: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_IDENTITY {}
impl ::core::clone::Clone for PERF_COUNTER_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PERF_COUNTER_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub Size: u32,
    pub DetailLevel: u32,
    pub Scale: i32,
    pub Offset: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_INFO {}
impl ::core::clone::Clone for PERF_COUNTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PERF_COUNTER_PRECISION: u32 = 458752u32;
pub const PERF_COUNTER_QUEUELEN: u32 = 327680u32;
pub const PERF_COUNTER_RATE: u32 = 65536u32;
#[repr(C)]
pub struct PERF_COUNTER_REG_INFO {
    pub CounterId: u32,
    pub Type: u32,
    pub Attrib: u64,
    pub DetailLevel: u32,
    pub DefaultScale: i32,
    pub BaseCounterId: u32,
    pub PerfTimeId: u32,
    pub PerfFreqId: u32,
    pub MultiId: u32,
    pub AggregateFunc: PERF_COUNTER_AGGREGATE_FUNC,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PERF_COUNTER_REG_INFO {}
impl ::core::clone::Clone for PERF_COUNTER_REG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PERF_COUNTER_VALUE: u32 = 0u32;
#[repr(C)]
pub struct PERF_DATA_BLOCK {
    pub Signature: [u16; 4],
    pub LittleEndian: u32,
    pub Version: u32,
    pub Revision: u32,
    pub TotalByteLength: u32,
    pub HeaderLength: u32,
    pub NumObjectTypes: u32,
    pub DefaultObject: i32,
    pub SystemTime: ::win32_foundation_sys::SYSTEMTIME,
    pub PerfTime: i64,
    pub PerfFreq: i64,
    pub PerfTime100nSec: i64,
    pub SystemNameLength: u32,
    pub SystemNameOffset: u32,
}
impl ::core::marker::Copy for PERF_DATA_BLOCK {}
impl ::core::clone::Clone for PERF_DATA_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PERF_DATA_HEADER {
    pub dwTotalSize: u32,
    pub dwNumCounters: u32,
    pub PerfTimeStamp: i64,
    pub PerfTime100NSec: i64,
    pub PerfFreq: i64,
    pub SystemTime: ::win32_foundation_sys::SYSTEMTIME,
}
impl ::core::marker::Copy for PERF_DATA_HEADER {}
impl ::core::clone::Clone for PERF_DATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PERF_DATA_REVISION: u32 = 1u32;
pub const PERF_DATA_VERSION: u32 = 1u32;
pub const PERF_DELTA_BASE: u32 = 8388608u32;
pub const PERF_DELTA_COUNTER: u32 = 4194304u32;
pub type PERF_DETAIL = u32;
pub const PERF_DETAIL_NOVICE: PERF_DETAIL = 100u32;
pub const PERF_DETAIL_ADVANCED: PERF_DETAIL = 200u32;
pub const PERF_DETAIL_EXPERT: PERF_DETAIL = 300u32;
pub const PERF_DETAIL_WIZARD: PERF_DETAIL = 400u32;
pub const PERF_DISPLAY_NOSHOW: u32 = 1073741824u32;
pub const PERF_DISPLAY_NO_SUFFIX: u32 = 0u32;
pub const PERF_DISPLAY_PERCENT: u32 = 536870912u32;
pub const PERF_DISPLAY_PER_SEC: u32 = 268435456u32;
pub const PERF_DISPLAY_SECONDS: u32 = 805306368u32;
pub const PERF_ENUM_INSTANCES: u32 = 3u32;
pub const PERF_FILTER: u32 = 9u32;
#[repr(C)]
pub struct PERF_INSTANCE_DEFINITION {
    pub ByteLength: u32,
    pub ParentObjectTitleIndex: u32,
    pub ParentObjectInstance: u32,
    pub UniqueID: i32,
    pub NameOffset: u32,
    pub NameLength: u32,
}
impl ::core::marker::Copy for PERF_INSTANCE_DEFINITION {}
impl ::core::clone::Clone for PERF_INSTANCE_DEFINITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PERF_INSTANCE_HEADER {
    pub Size: u32,
    pub InstanceId: u32,
}
impl ::core::marker::Copy for PERF_INSTANCE_HEADER {}
impl ::core::clone::Clone for PERF_INSTANCE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PERF_INVERSE_COUNTER: u32 = 16777216u32;
pub const PERF_MAX_INSTANCE_NAME: u32 = 1024u32;
pub type PERF_MEM_ALLOC = ::core::option::Option<unsafe extern "system" fn(allocsize: usize, pcontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
pub type PERF_MEM_FREE = ::core::option::Option<unsafe extern "system" fn(pbuffer: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void)>;
pub const PERF_METADATA_MULTIPLE_INSTANCES: i32 = -2i32;
pub const PERF_METADATA_NO_INSTANCES: i32 = -3i32;
pub const PERF_MULTI_COUNTER: u32 = 33554432u32;
#[repr(C)]
pub struct PERF_MULTI_COUNTERS {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl ::core::marker::Copy for PERF_MULTI_COUNTERS {}
impl ::core::clone::Clone for PERF_MULTI_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PERF_MULTI_INSTANCES {
    pub dwTotalSize: u32,
    pub dwInstances: u32,
}
impl ::core::marker::Copy for PERF_MULTI_INSTANCES {}
impl ::core::clone::Clone for PERF_MULTI_INSTANCES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PERF_NO_INSTANCES: i32 = -1i32;
pub const PERF_NO_UNIQUE_ID: i32 = -1i32;
pub const PERF_NUMBER_DECIMAL: u32 = 65536u32;
pub const PERF_NUMBER_DEC_1000: u32 = 131072u32;
pub const PERF_NUMBER_HEX: u32 = 0u32;
pub const PERF_OBJECT_TIMER: u32 = 2097152u32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: u32,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: u32,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for PERF_OBJECT_TYPE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for PERF_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
pub struct PERF_OBJECT_TYPE {
    pub TotalByteLength: u32,
    pub DefinitionLength: u32,
    pub HeaderLength: u32,
    pub ObjectNameTitleIndex: u32,
    pub ObjectNameTitle: ::windows_core_sys::PWSTR,
    pub ObjectHelpTitleIndex: u32,
    pub ObjectHelpTitle: ::windows_core_sys::PWSTR,
    pub DetailLevel: u32,
    pub NumCounters: u32,
    pub DefaultCounter: i32,
    pub NumInstances: i32,
    pub CodePage: u32,
    pub PerfTime: i64,
    pub PerfFreq: i64,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for PERF_OBJECT_TYPE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for PERF_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PERF_PROVIDER_CONTEXT {
    pub ContextSize: u32,
    pub Reserved: u32,
    pub ControlCallback: PERFLIBREQUEST,
    pub MemAllocRoutine: PERF_MEM_ALLOC,
    pub MemFreeRoutine: PERF_MEM_FREE,
    pub pMemContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PERF_PROVIDER_CONTEXT {}
impl ::core::clone::Clone for PERF_PROVIDER_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PERF_PROVIDER_DRIVER: u32 = 2u32;
pub const PERF_PROVIDER_KERNEL_MODE: u32 = 1u32;
pub const PERF_PROVIDER_USER_MODE: u32 = 0u32;
pub const PERF_REMOVE_COUNTER: u32 = 2u32;
pub const PERF_SIZE_DWORD: u32 = 0u32;
pub const PERF_SIZE_LARGE: u32 = 256u32;
pub const PERF_SIZE_VARIABLE_LEN: u32 = 768u32;
pub const PERF_SIZE_ZERO: u32 = 512u32;
#[repr(C)]
pub struct PERF_STRING_BUFFER_HEADER {
    pub dwSize: u32,
    pub dwCounters: u32,
}
impl ::core::marker::Copy for PERF_STRING_BUFFER_HEADER {}
impl ::core::clone::Clone for PERF_STRING_BUFFER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PERF_STRING_COUNTER_HEADER {
    pub dwCounterId: u32,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for PERF_STRING_COUNTER_HEADER {}
impl ::core::clone::Clone for PERF_STRING_COUNTER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PERF_TEXT_ASCII: u32 = 65536u32;
pub const PERF_TEXT_UNICODE: u32 = 0u32;
pub const PERF_TIMER_100NS: u32 = 1048576u32;
pub const PERF_TIMER_TICK: u32 = 0u32;
pub const PERF_TYPE_COUNTER: u32 = 1024u32;
pub const PERF_TYPE_NUMBER: u32 = 0u32;
pub const PERF_TYPE_TEXT: u32 = 2048u32;
pub const PERF_TYPE_ZERO: u32 = 3072u32;
pub const PERF_WILDCARD_COUNTER: u32 = 4294967295u32;
pub const PERF_WILDCARD_INSTANCE: &str = "*";
pub type PLA_CABEXTRACT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(filename: ::windows_core_sys::PCWSTR, context: *mut ::core::ffi::c_void)>;
pub const PLA_CAPABILITY_AUTOLOGGER: u32 = 32u32;
pub const PLA_CAPABILITY_LEGACY_SESSION: u32 = 8u32;
pub const PLA_CAPABILITY_LEGACY_SVC: u32 = 16u32;
pub const PLA_CAPABILITY_LOCAL: u32 = 268435456u32;
pub const PLA_CAPABILITY_V1_SESSION: u32 = 2u32;
pub const PLA_CAPABILITY_V1_SVC: u32 = 1u32;
pub const PLA_CAPABILITY_V1_SYSTEM: u32 = 4u32;
pub type PM_CLOSE_PROC = ::core::option::Option<unsafe extern "system" fn() -> u32>;
pub type PM_COLLECT_PROC = ::core::option::Option<unsafe extern "system" fn(pvaluename: ::windows_core_sys::PCWSTR, ppdata: *mut *mut ::core::ffi::c_void, pcbtotalbytes: *mut u32, pnumobjecttypes: *mut u32) -> u32>;
pub type PM_OPEN_PROC = ::core::option::Option<unsafe extern "system" fn(pcontext: ::windows_core_sys::PCWSTR) -> u32>;
pub type PerfCounterDataType = i32;
pub const PERF_ERROR_RETURN: PerfCounterDataType = 0i32;
pub const PERF_SINGLE_COUNTER: PerfCounterDataType = 1i32;
pub const PERF_MULTIPLE_COUNTERS: PerfCounterDataType = 2i32;
pub const PERF_MULTIPLE_INSTANCES: PerfCounterDataType = 4i32;
pub const PERF_COUNTERSET: PerfCounterDataType = 6i32;
pub type PerfProviderHandle = isize;
pub type PerfQueryHandle = isize;
pub type PerfRegInfoType = i32;
pub const PERF_REG_COUNTERSET_STRUCT: PerfRegInfoType = 1i32;
pub const PERF_REG_COUNTER_STRUCT: PerfRegInfoType = 2i32;
pub const PERF_REG_COUNTERSET_NAME_STRING: PerfRegInfoType = 3i32;
pub const PERF_REG_COUNTERSET_HELP_STRING: PerfRegInfoType = 4i32;
pub const PERF_REG_COUNTER_NAME_STRINGS: PerfRegInfoType = 5i32;
pub const PERF_REG_COUNTER_HELP_STRINGS: PerfRegInfoType = 6i32;
pub const PERF_REG_PROVIDER_NAME: PerfRegInfoType = 7i32;
pub const PERF_REG_PROVIDER_GUID: PerfRegInfoType = 8i32;
pub const PERF_REG_COUNTERSET_ENGLISH_NAME: PerfRegInfoType = 9i32;
pub const PERF_REG_COUNTER_ENGLISH_NAMES: PerfRegInfoType = 10i32;
pub type REAL_TIME_DATA_SOURCE_ID_FLAGS = u32;
pub const DATA_SOURCE_REGISTRY: REAL_TIME_DATA_SOURCE_ID_FLAGS = 1u32;
pub const DATA_SOURCE_WBEM: REAL_TIME_DATA_SOURCE_ID_FLAGS = 4u32;
pub type ReportValueTypeConstants = i32;
pub const sysmonDefaultValue: ReportValueTypeConstants = 0i32;
pub const sysmonCurrentValue: ReportValueTypeConstants = 1i32;
pub const sysmonAverage: ReportValueTypeConstants = 2i32;
pub const sysmonMinimum: ReportValueTypeConstants = 3i32;
pub const sysmonMaximum: ReportValueTypeConstants = 4i32;
pub type ResourcePolicy = i32;
pub const plaDeleteLargest: ResourcePolicy = 0i32;
pub const plaDeleteOldest: ResourcePolicy = 1i32;
pub const S_PDH: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 81159000, data2: 50337, data3: 16795, data4: [128, 35, 35, 183, 57, 2, 222, 44] };
pub const ServerDataCollectorSet: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946865, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const ServerDataCollectorSetCollection: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946866, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const SourcePropPage: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 217262753, data2: 30065, data3: 4560, data4: [147, 196, 0, 170, 0, 163, 221, 234] };
pub type StreamMode = i32;
pub const plaFile: StreamMode = 1i32;
pub const plaRealTime: StreamMode = 2i32;
pub const plaBoth: StreamMode = 3i32;
pub const plaBuffering: StreamMode = 4i32;
pub type SysmonBatchReason = i32;
pub const sysmonBatchNone: SysmonBatchReason = 0i32;
pub const sysmonBatchAddFiles: SysmonBatchReason = 1i32;
pub const sysmonBatchAddCounters: SysmonBatchReason = 2i32;
pub const sysmonBatchAddFilesAutoCounters: SysmonBatchReason = 3i32;
pub type SysmonDataType = i32;
pub const sysmonDataAvg: SysmonDataType = 1i32;
pub const sysmonDataMin: SysmonDataType = 2i32;
pub const sysmonDataMax: SysmonDataType = 3i32;
pub const sysmonDataTime: SysmonDataType = 4i32;
pub const sysmonDataCount: SysmonDataType = 5i32;
pub type SysmonFileType = i32;
pub const sysmonFileHtml: SysmonFileType = 1i32;
pub const sysmonFileReport: SysmonFileType = 2i32;
pub const sysmonFileCsv: SysmonFileType = 3i32;
pub const sysmonFileTsv: SysmonFileType = 4i32;
pub const sysmonFileBlg: SysmonFileType = 5i32;
pub const sysmonFileRetiredBlg: SysmonFileType = 6i32;
pub const sysmonFileGif: SysmonFileType = 7i32;
pub const SystemDataCollectorSet: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946886, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const SystemDataCollectorSetCollection: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946887, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const SystemMonitor: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 3302152416, data2: 53725, data3: 4558, data4: [148, 15, 0, 128, 41, 0, 67, 71] };
pub const SystemMonitor2: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 2133874572, data2: 24376, data3: 17938, data4: [172, 254, 110, 208, 76, 123, 122, 248] };
pub const TraceDataProvider: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946835, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const TraceDataProviderCollection: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946833, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const TraceSession: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946844, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub const TraceSessionCollection: ::windows_core_sys::GUID = ::windows_core_sys::GUID { data1: 58946864, data2: 2443, data3: 4568, data4: [148, 20, 80, 80, 84, 80, 48, 48] };
pub type ValueMapType = i32;
pub const plaIndex: ValueMapType = 1i32;
pub const plaFlag: ValueMapType = 2i32;
pub const plaFlagArray: ValueMapType = 3i32;
pub const plaValidation: ValueMapType = 4i32;
pub const WINPERF_LOG_DEBUG: u32 = 2u32;
pub const WINPERF_LOG_NONE: u32 = 0u32;
pub const WINPERF_LOG_USER: u32 = 1u32;
pub const WINPERF_LOG_VERBOSE: u32 = 3u32;
pub type WeekDays = i32;
pub const plaRunOnce: WeekDays = 0i32;
pub const plaSunday: WeekDays = 1i32;
pub const plaMonday: WeekDays = 2i32;
pub const plaTuesday: WeekDays = 4i32;
pub const plaWednesday: WeekDays = 8i32;
pub const plaThursday: WeekDays = 16i32;
pub const plaFriday: WeekDays = 32i32;
pub const plaSaturday: WeekDays = 64i32;
pub const plaEveryday: WeekDays = 127i32;
pub type _ICounterItemUnion = *mut ::core::ffi::c_void;
pub type _ISystemMonitorUnion = *mut ::core::ffi::c_void;
