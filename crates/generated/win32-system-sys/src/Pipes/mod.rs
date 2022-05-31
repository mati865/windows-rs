#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallNamedPipeA(lpnamedpipename: ::windows_sys_core::PCSTR, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallNamedPipeW(lpnamedpipename: ::windows_sys_core::PCWSTR, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ConnectNamedPipe(hnamedpipe: super::super::Foundation::HANDLE, lpoverlapped: *mut super::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
    pub fn CreateNamedPipeA(lpname: ::windows_sys_core::PCSTR, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
    pub fn CreateNamedPipeW(lpname: ::windows_sys_core::PCWSTR, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreatePipe(hreadpipe: *mut super::super::Foundation::HANDLE, hwritepipe: *mut super::super::Foundation::HANDLE, lppipeattributes: *const super::super::Security::SECURITY_ATTRIBUTES, nsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisconnectNamedPipe(hnamedpipe: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeClientComputerNameA(pipe: super::super::Foundation::HANDLE, clientcomputername: ::windows_sys_core::PSTR, clientcomputernamelength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeClientComputerNameW(pipe: super::super::Foundation::HANDLE, clientcomputername: ::windows_sys_core::PWSTR, clientcomputernamelength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeClientProcessId(pipe: super::super::Foundation::HANDLE, clientprocessid: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeClientSessionId(pipe: super::super::Foundation::HANDLE, clientsessionid: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeHandleStateA(hnamedpipe: super::super::Foundation::HANDLE, lpstate: *mut NAMED_PIPE_MODE, lpcurinstances: *mut u32, lpmaxcollectioncount: *mut u32, lpcollectdatatimeout: *mut u32, lpusername: ::windows_sys_core::PSTR, nmaxusernamesize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeHandleStateW(hnamedpipe: super::super::Foundation::HANDLE, lpstate: *mut NAMED_PIPE_MODE, lpcurinstances: *mut u32, lpmaxcollectioncount: *mut u32, lpcollectdatatimeout: *mut u32, lpusername: ::windows_sys_core::PWSTR, nmaxusernamesize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeInfo(hnamedpipe: super::super::Foundation::HANDLE, lpflags: *mut NAMED_PIPE_MODE, lpoutbuffersize: *mut u32, lpinbuffersize: *mut u32, lpmaxinstances: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeServerProcessId(pipe: super::super::Foundation::HANDLE, serverprocessid: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedPipeServerSessionId(pipe: super::super::Foundation::HANDLE, serversessionid: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateNamedPipeClient(hnamedpipe: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeekNamedPipe(hnamedpipe: super::super::Foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nbuffersize: u32, lpbytesread: *mut u32, lptotalbytesavail: *mut u32, lpbytesleftthismessage: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetNamedPipeHandleState(hnamedpipe: super::super::Foundation::HANDLE, lpmode: *const NAMED_PIPE_MODE, lpmaxcollectioncount: *const u32, lpcollectdatatimeout: *const u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn TransactNamedPipe(hnamedpipe: super::super::Foundation::HANDLE, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, lpoverlapped: *mut super::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitNamedPipeA(lpnamedpipename: ::windows_sys_core::PCSTR, ntimeout: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitNamedPipeW(lpnamedpipename: ::windows_sys_core::PCWSTR, ntimeout: u32) -> super::super::Foundation::BOOL;
}
pub type NAMED_PIPE_MODE = u32;
pub const PIPE_WAIT: NAMED_PIPE_MODE = 0u32;
pub const PIPE_NOWAIT: NAMED_PIPE_MODE = 1u32;
pub const PIPE_READMODE_BYTE: NAMED_PIPE_MODE = 0u32;
pub const PIPE_READMODE_MESSAGE: NAMED_PIPE_MODE = 2u32;
pub const PIPE_CLIENT_END: NAMED_PIPE_MODE = 0u32;
pub const PIPE_SERVER_END: NAMED_PIPE_MODE = 1u32;
pub const PIPE_TYPE_BYTE: NAMED_PIPE_MODE = 0u32;
pub const PIPE_TYPE_MESSAGE: NAMED_PIPE_MODE = 4u32;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: NAMED_PIPE_MODE = 0u32;
pub const PIPE_REJECT_REMOTE_CLIENTS: NAMED_PIPE_MODE = 8u32;
pub const NMPWAIT_NOWAIT: u32 = 1u32;
pub const NMPWAIT_USE_DEFAULT_WAIT: u32 = 0u32;
pub const NMPWAIT_WAIT_FOREVER: u32 = 4294967295u32;
pub const PIPE_UNLIMITED_INSTANCES: u32 = 255u32;