pub const ALTNUMPAD_BIT: u32 = 67108864u32;
pub const ATTACH_PARENT_PROCESS: u32 = 4294967295u32;
#[inline]
pub unsafe fn AddConsoleAliasA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(source: Param0, target: Param1, exename: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddConsoleAliasA(source: ::windows_core::PCSTR, target: ::windows_core::PCSTR, exename: ::windows_core::PCSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AddConsoleAliasA(source.into_param().abi(), target.into_param().abi(), exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AddConsoleAliasW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(source: Param0, target: Param1, exename: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddConsoleAliasW(source: ::windows_core::PCWSTR, target: ::windows_core::PCWSTR, exename: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AddConsoleAliasW(source.into_param().abi(), target.into_param().abi(), exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AllocConsole() -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocConsole() -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AllocConsole())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AttachConsole(dwprocessid: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AttachConsole(dwprocessid: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AttachConsole(::core::mem::transmute(dwprocessid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const BACKGROUND_BLUE: u32 = 16u32;
pub const BACKGROUND_GREEN: u32 = 32u32;
pub const BACKGROUND_INTENSITY: u32 = 128u32;
pub const BACKGROUND_RED: u32 = 64u32;
pub const CAPSLOCK_ON: u32 = 128u32;
#[repr(C)]
pub struct CHAR_INFO {
    pub Char: CHAR_INFO_0,
    pub Attributes: u16,
}
impl ::core::marker::Copy for CHAR_INFO {}
impl ::core::clone::Clone for CHAR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for CHAR_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHAR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHAR_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHAR_INFO {}
impl ::core::default::Default for CHAR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union CHAR_INFO_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: ::win32_foundation::CHAR,
}
impl ::core::marker::Copy for CHAR_INFO_0 {}
impl ::core::clone::Clone for CHAR_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for CHAR_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CHAR_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHAR_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CHAR_INFO_0 {}
impl ::core::default::Default for CHAR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const COMMON_LVB_GRID_HORIZONTAL: u32 = 1024u32;
pub const COMMON_LVB_GRID_LVERTICAL: u32 = 2048u32;
pub const COMMON_LVB_GRID_RVERTICAL: u32 = 4096u32;
pub const COMMON_LVB_LEADING_BYTE: u32 = 256u32;
pub const COMMON_LVB_REVERSE_VIDEO: u32 = 16384u32;
pub const COMMON_LVB_SBCSDBCS: u32 = 768u32;
pub const COMMON_LVB_TRAILING_BYTE: u32 = 512u32;
pub const COMMON_LVB_UNDERSCORE: u32 = 32768u32;
#[repr(C)]
pub struct CONSOLE_CURSOR_INFO {
    pub dwSize: u32,
    pub bVisible: ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for CONSOLE_CURSOR_INFO {}
impl ::core::clone::Clone for CONSOLE_CURSOR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_CURSOR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_CURSOR_INFO").field("dwSize", &self.dwSize).field("bVisible", &self.bVisible).finish()
    }
}
unsafe impl ::windows_core::Abi for CONSOLE_CURSOR_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_CURSOR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_CURSOR_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_CURSOR_INFO {}
impl ::core::default::Default for CONSOLE_CURSOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CONSOLE_FONT_INFO {
    pub nFont: u32,
    pub dwFontSize: COORD,
}
impl ::core::marker::Copy for CONSOLE_FONT_INFO {}
impl ::core::clone::Clone for CONSOLE_FONT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_FONT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_FONT_INFO").field("nFont", &self.nFont).field("dwFontSize", &self.dwFontSize).finish()
    }
}
unsafe impl ::windows_core::Abi for CONSOLE_FONT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_FONT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_FONT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_FONT_INFO {}
impl ::core::default::Default for CONSOLE_FONT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CONSOLE_FONT_INFOEX {
    pub cbSize: u32,
    pub nFont: u32,
    pub dwFontSize: COORD,
    pub FontFamily: u32,
    pub FontWeight: u32,
    pub FaceName: [u16; 32],
}
impl ::core::marker::Copy for CONSOLE_FONT_INFOEX {}
impl ::core::clone::Clone for CONSOLE_FONT_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_FONT_INFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_FONT_INFOEX").field("cbSize", &self.cbSize).field("nFont", &self.nFont).field("dwFontSize", &self.dwFontSize).field("FontFamily", &self.FontFamily).field("FontWeight", &self.FontWeight).field("FaceName", &self.FaceName).finish()
    }
}
unsafe impl ::windows_core::Abi for CONSOLE_FONT_INFOEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_FONT_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_FONT_INFOEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_FONT_INFOEX {}
impl ::core::default::Default for CONSOLE_FONT_INFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CONSOLE_FULLSCREEN: u32 = 1u32;
pub const CONSOLE_FULLSCREEN_HARDWARE: u32 = 2u32;
pub const CONSOLE_FULLSCREEN_MODE: u32 = 1u32;
#[repr(C)]
pub struct CONSOLE_HISTORY_INFO {
    pub cbSize: u32,
    pub HistoryBufferSize: u32,
    pub NumberOfHistoryBuffers: u32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for CONSOLE_HISTORY_INFO {}
impl ::core::clone::Clone for CONSOLE_HISTORY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_HISTORY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_HISTORY_INFO").field("cbSize", &self.cbSize).field("HistoryBufferSize", &self.HistoryBufferSize).field("NumberOfHistoryBuffers", &self.NumberOfHistoryBuffers).field("dwFlags", &self.dwFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for CONSOLE_HISTORY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_HISTORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_HISTORY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_HISTORY_INFO {}
impl ::core::default::Default for CONSOLE_HISTORY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CONSOLE_MODE(pub u32);
pub const ENABLE_PROCESSED_INPUT: CONSOLE_MODE = CONSOLE_MODE(1u32);
pub const ENABLE_LINE_INPUT: CONSOLE_MODE = CONSOLE_MODE(2u32);
pub const ENABLE_ECHO_INPUT: CONSOLE_MODE = CONSOLE_MODE(4u32);
pub const ENABLE_WINDOW_INPUT: CONSOLE_MODE = CONSOLE_MODE(8u32);
pub const ENABLE_MOUSE_INPUT: CONSOLE_MODE = CONSOLE_MODE(16u32);
pub const ENABLE_INSERT_MODE: CONSOLE_MODE = CONSOLE_MODE(32u32);
pub const ENABLE_QUICK_EDIT_MODE: CONSOLE_MODE = CONSOLE_MODE(64u32);
pub const ENABLE_EXTENDED_FLAGS: CONSOLE_MODE = CONSOLE_MODE(128u32);
pub const ENABLE_AUTO_POSITION: CONSOLE_MODE = CONSOLE_MODE(256u32);
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: CONSOLE_MODE = CONSOLE_MODE(512u32);
pub const ENABLE_PROCESSED_OUTPUT: CONSOLE_MODE = CONSOLE_MODE(1u32);
pub const ENABLE_WRAP_AT_EOL_OUTPUT: CONSOLE_MODE = CONSOLE_MODE(2u32);
pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: CONSOLE_MODE = CONSOLE_MODE(4u32);
pub const DISABLE_NEWLINE_AUTO_RETURN: CONSOLE_MODE = CONSOLE_MODE(8u32);
pub const ENABLE_LVB_GRID_WORLDWIDE: CONSOLE_MODE = CONSOLE_MODE(16u32);
impl ::core::marker::Copy for CONSOLE_MODE {}
impl ::core::clone::Clone for CONSOLE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONSOLE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CONSOLE_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONSOLE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONSOLE_MODE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CONSOLE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CONSOLE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CONSOLE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CONSOLE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CONSOLE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CONSOLE_MOUSE_DOWN: u32 = 8u32;
pub const CONSOLE_MOUSE_SELECTION: u32 = 4u32;
pub const CONSOLE_NO_SELECTION: u32 = 0u32;
#[repr(C)]
pub struct CONSOLE_READCONSOLE_CONTROL {
    pub nLength: u32,
    pub nInitialChars: u32,
    pub dwCtrlWakeupMask: u32,
    pub dwControlKeyState: u32,
}
impl ::core::marker::Copy for CONSOLE_READCONSOLE_CONTROL {}
impl ::core::clone::Clone for CONSOLE_READCONSOLE_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_READCONSOLE_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_READCONSOLE_CONTROL").field("nLength", &self.nLength).field("nInitialChars", &self.nInitialChars).field("dwCtrlWakeupMask", &self.dwCtrlWakeupMask).field("dwControlKeyState", &self.dwControlKeyState).finish()
    }
}
unsafe impl ::windows_core::Abi for CONSOLE_READCONSOLE_CONTROL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_READCONSOLE_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_READCONSOLE_CONTROL>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_READCONSOLE_CONTROL {}
impl ::core::default::Default for CONSOLE_READCONSOLE_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: u16,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}
impl ::core::marker::Copy for CONSOLE_SCREEN_BUFFER_INFO {}
impl ::core::clone::Clone for CONSOLE_SCREEN_BUFFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_SCREEN_BUFFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_SCREEN_BUFFER_INFO").field("dwSize", &self.dwSize).field("dwCursorPosition", &self.dwCursorPosition).field("wAttributes", &self.wAttributes).field("srWindow", &self.srWindow).field("dwMaximumWindowSize", &self.dwMaximumWindowSize).finish()
    }
}
unsafe impl ::windows_core::Abi for CONSOLE_SCREEN_BUFFER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_SCREEN_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_SCREEN_BUFFER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_SCREEN_BUFFER_INFO {}
impl ::core::default::Default for CONSOLE_SCREEN_BUFFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CONSOLE_SCREEN_BUFFER_INFOEX {
    pub cbSize: u32,
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: u16,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
    pub wPopupAttributes: u16,
    pub bFullscreenSupported: ::win32_foundation::BOOL,
    pub ColorTable: [u32; 16],
}
impl ::core::marker::Copy for CONSOLE_SCREEN_BUFFER_INFOEX {}
impl ::core::clone::Clone for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_SCREEN_BUFFER_INFOEX").field("cbSize", &self.cbSize).field("dwSize", &self.dwSize).field("dwCursorPosition", &self.dwCursorPosition).field("wAttributes", &self.wAttributes).field("srWindow", &self.srWindow).field("dwMaximumWindowSize", &self.dwMaximumWindowSize).field("wPopupAttributes", &self.wPopupAttributes).field("bFullscreenSupported", &self.bFullscreenSupported).field("ColorTable", &self.ColorTable).finish()
    }
}
unsafe impl ::windows_core::Abi for CONSOLE_SCREEN_BUFFER_INFOEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_SCREEN_BUFFER_INFOEX {}
impl ::core::default::Default for CONSOLE_SCREEN_BUFFER_INFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: u32,
    pub dwSelectionAnchor: COORD,
    pub srSelection: SMALL_RECT,
}
impl ::core::marker::Copy for CONSOLE_SELECTION_INFO {}
impl ::core::clone::Clone for CONSOLE_SELECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONSOLE_SELECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONSOLE_SELECTION_INFO").field("dwFlags", &self.dwFlags).field("dwSelectionAnchor", &self.dwSelectionAnchor).field("srSelection", &self.srSelection).finish()
    }
}
unsafe impl ::windows_core::Abi for CONSOLE_SELECTION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONSOLE_SELECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONSOLE_SELECTION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONSOLE_SELECTION_INFO {}
impl ::core::default::Default for CONSOLE_SELECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CONSOLE_SELECTION_IN_PROGRESS: u32 = 1u32;
pub const CONSOLE_SELECTION_NOT_EMPTY: u32 = 2u32;
pub const CONSOLE_TEXTMODE_BUFFER: u32 = 1u32;
pub const CONSOLE_WINDOWED_MODE: u32 = 2u32;
#[repr(C)]
pub struct COORD {
    pub X: i16,
    pub Y: i16,
}
impl ::core::marker::Copy for COORD {}
impl ::core::clone::Clone for COORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COORD").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
unsafe impl ::windows_core::Abi for COORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for COORD {}
impl ::core::default::Default for COORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CTRL_BREAK_EVENT: u32 = 1u32;
pub const CTRL_CLOSE_EVENT: u32 = 2u32;
pub const CTRL_C_EVENT: u32 = 0u32;
pub const CTRL_LOGOFF_EVENT: u32 = 5u32;
pub const CTRL_SHUTDOWN_EVENT: u32 = 6u32;
#[inline]
pub unsafe fn ClosePseudoConsole<'a, Param0: ::windows_core::IntoParam<'a, HPCON>>(hpc: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClosePseudoConsole(hpc: HPCON);
        }
        ClosePseudoConsole(hpc.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateConsoleScreenBuffer(dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: *const ::win32_security::SECURITY_ATTRIBUTES, dwflags: u32, lpscreenbufferdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<::win32_foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateConsoleScreenBuffer(dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: *const ::win32_security::SECURITY_ATTRIBUTES, dwflags: u32, lpscreenbufferdata: *mut ::core::ffi::c_void) -> ::win32_foundation::HANDLE;
        }
        let result__ = CreateConsoleScreenBuffer(::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(dwsharemode), ::core::mem::transmute(lpsecurityattributes), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpscreenbufferdata));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreatePseudoConsole<'a, Param0: ::windows_core::IntoParam<'a, COORD>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(size: Param0, hinput: Param1, houtput: Param2, dwflags: u32) -> ::windows_core::Result<HPCON> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePseudoConsole(size: COORD, hinput: ::win32_foundation::HANDLE, houtput: ::win32_foundation::HANDLE, dwflags: u32, phpc: *mut HPCON) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<HPCON>::zeroed();
        CreatePseudoConsole(size.into_param().abi(), hinput.into_param().abi(), houtput.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HPCON>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DOUBLE_CLICK: u32 = 2u32;
pub const ENHANCED_KEY: u32 = 256u32;
#[inline]
pub unsafe fn ExpungeConsoleCommandHistoryA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(exename: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExpungeConsoleCommandHistoryA(exename: ::windows_core::PCSTR);
        }
        ExpungeConsoleCommandHistoryA(exename.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ExpungeConsoleCommandHistoryW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(exename: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExpungeConsoleCommandHistoryW(exename: ::windows_core::PCWSTR);
        }
        ExpungeConsoleCommandHistoryW(exename.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FOCUS_EVENT: u32 = 16u32;
#[repr(C)]
pub struct FOCUS_EVENT_RECORD {
    pub bSetFocus: ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for FOCUS_EVENT_RECORD {}
impl ::core::clone::Clone for FOCUS_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FOCUS_EVENT_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FOCUS_EVENT_RECORD").field("bSetFocus", &self.bSetFocus).finish()
    }
}
unsafe impl ::windows_core::Abi for FOCUS_EVENT_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FOCUS_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FOCUS_EVENT_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for FOCUS_EVENT_RECORD {}
impl ::core::default::Default for FOCUS_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const FOREGROUND_BLUE: u32 = 1u32;
pub const FOREGROUND_GREEN: u32 = 2u32;
pub const FOREGROUND_INTENSITY: u32 = 8u32;
pub const FOREGROUND_RED: u32 = 4u32;
pub const FROM_LEFT_1ST_BUTTON_PRESSED: u32 = 1u32;
pub const FROM_LEFT_2ND_BUTTON_PRESSED: u32 = 4u32;
pub const FROM_LEFT_3RD_BUTTON_PRESSED: u32 = 8u32;
pub const FROM_LEFT_4TH_BUTTON_PRESSED: u32 = 16u32;
#[inline]
pub unsafe fn FillConsoleOutputAttribute<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, wattribute: u16, nlength: u32, dwwritecoord: Param3, lpnumberofattrswritten: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FillConsoleOutputAttribute(hconsoleoutput: ::win32_foundation::HANDLE, wattribute: u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FillConsoleOutputAttribute(hconsoleoutput.into_param().abi(), ::core::mem::transmute(wattribute), ::core::mem::transmute(nlength), dwwritecoord.into_param().abi(), ::core::mem::transmute(lpnumberofattrswritten)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FillConsoleOutputCharacterA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::CHAR>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, ccharacter: Param1, nlength: u32, dwwritecoord: Param3, lpnumberofcharswritten: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FillConsoleOutputCharacterA(hconsoleoutput: ::win32_foundation::HANDLE, ccharacter: ::win32_foundation::CHAR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FillConsoleOutputCharacterA(hconsoleoutput.into_param().abi(), ccharacter.into_param().abi(), ::core::mem::transmute(nlength), dwwritecoord.into_param().abi(), ::core::mem::transmute(lpnumberofcharswritten)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FillConsoleOutputCharacterW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, ccharacter: u16, nlength: u32, dwwritecoord: Param3, lpnumberofcharswritten: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FillConsoleOutputCharacterW(hconsoleoutput: ::win32_foundation::HANDLE, ccharacter: u16, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FillConsoleOutputCharacterW(hconsoleoutput.into_param().abi(), ::core::mem::transmute(ccharacter), ::core::mem::transmute(nlength), dwwritecoord.into_param().abi(), ::core::mem::transmute(lpnumberofcharswritten)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FlushConsoleInputBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleinput: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushConsoleInputBuffer(hconsoleinput: ::win32_foundation::HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FlushConsoleInputBuffer(hconsoleinput.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FreeConsole() -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeConsole() -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FreeConsole())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GenerateConsoleCtrlEvent(dwctrlevent: u32, dwprocessgroupid: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GenerateConsoleCtrlEvent(dwctrlevent: u32, dwprocessgroupid: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GenerateConsoleCtrlEvent(::core::mem::transmute(dwctrlevent), ::core::mem::transmute(dwprocessgroupid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleAliasA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(source: Param0, targetbuffer: &mut [u8], exename: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasA(source: ::windows_core::PCSTR, targetbuffer: ::windows_core::PSTR, targetbufferlength: u32, exename: ::windows_core::PCSTR) -> u32;
        }
        ::core::mem::transmute(GetConsoleAliasA(source.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(targetbuffer)), targetbuffer.len() as _, exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleAliasExesA(exenamebuffer: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasExesA(exenamebuffer: ::windows_core::PSTR, exenamebufferlength: u32) -> u32;
        }
        ::core::mem::transmute(GetConsoleAliasExesA(::core::mem::transmute(::windows_core::as_mut_ptr_or_null(exenamebuffer)), exenamebuffer.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleAliasExesLengthA() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasExesLengthA() -> u32;
        }
        ::core::mem::transmute(GetConsoleAliasExesLengthA())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleAliasExesLengthW() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasExesLengthW() -> u32;
        }
        ::core::mem::transmute(GetConsoleAliasExesLengthW())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleAliasExesW(exenamebuffer: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasExesW(exenamebuffer: ::windows_core::PWSTR, exenamebufferlength: u32) -> u32;
        }
        ::core::mem::transmute(GetConsoleAliasExesW(::core::mem::transmute(::windows_core::as_mut_ptr_or_null(exenamebuffer)), exenamebuffer.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleAliasW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(source: Param0, targetbuffer: &mut [u16], exename: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasW(source: ::windows_core::PCWSTR, targetbuffer: ::windows_core::PWSTR, targetbufferlength: u32, exename: ::windows_core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(GetConsoleAliasW(source.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(targetbuffer)), targetbuffer.len() as _, exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleAliasesA<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(aliasbuffer: &mut [u8], exename: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasesA(aliasbuffer: ::windows_core::PSTR, aliasbufferlength: u32, exename: ::windows_core::PCSTR) -> u32;
        }
        ::core::mem::transmute(GetConsoleAliasesA(::core::mem::transmute(::windows_core::as_mut_ptr_or_null(aliasbuffer)), aliasbuffer.len() as _, exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleAliasesLengthA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(exename: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasesLengthA(exename: ::windows_core::PCSTR) -> u32;
        }
        ::core::mem::transmute(GetConsoleAliasesLengthA(exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleAliasesLengthW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(exename: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasesLengthW(exename: ::windows_core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(GetConsoleAliasesLengthW(exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleAliasesW<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(aliasbuffer: &mut [u16], exename: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleAliasesW(aliasbuffer: ::windows_core::PWSTR, aliasbufferlength: u32, exename: ::windows_core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(GetConsoleAliasesW(::core::mem::transmute(::windows_core::as_mut_ptr_or_null(aliasbuffer)), aliasbuffer.len() as _, exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleCP() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleCP() -> u32;
        }
        ::core::mem::transmute(GetConsoleCP())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleCommandHistoryA<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(commands: ::windows_core::PSTR, commandbufferlength: u32, exename: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleCommandHistoryA(commands: ::windows_core::PSTR, commandbufferlength: u32, exename: ::windows_core::PCSTR) -> u32;
        }
        ::core::mem::transmute(GetConsoleCommandHistoryA(::core::mem::transmute(commands), ::core::mem::transmute(commandbufferlength), exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleCommandHistoryLengthA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(exename: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleCommandHistoryLengthA(exename: ::windows_core::PCSTR) -> u32;
        }
        ::core::mem::transmute(GetConsoleCommandHistoryLengthA(exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleCommandHistoryLengthW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(exename: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleCommandHistoryLengthW(exename: ::windows_core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(GetConsoleCommandHistoryLengthW(exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleCommandHistoryW<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(commands: ::windows_core::PWSTR, commandbufferlength: u32, exename: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleCommandHistoryW(commands: ::windows_core::PWSTR, commandbufferlength: u32, exename: ::windows_core::PCWSTR) -> u32;
        }
        ::core::mem::transmute(GetConsoleCommandHistoryW(::core::mem::transmute(commands), ::core::mem::transmute(commandbufferlength), exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleCursorInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleoutput: Param0, lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleCursorInfo(hconsoleoutput: ::win32_foundation::HANDLE, lpconsolecursorinfo: *mut CONSOLE_CURSOR_INFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetConsoleCursorInfo(hconsoleoutput.into_param().abi(), ::core::mem::transmute(lpconsolecursorinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleDisplayMode(lpmodeflags: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleDisplayMode(lpmodeflags: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetConsoleDisplayMode(::core::mem::transmute(lpmodeflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleFontSize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleoutput: Param0, nfont: u32) -> COORD {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleFontSize(hconsoleoutput: ::win32_foundation::HANDLE, nfont: u32) -> COORD;
        }
        ::core::mem::transmute(GetConsoleFontSize(hconsoleoutput.into_param().abi(), ::core::mem::transmute(nfont)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleHistoryInfo(lpconsolehistoryinfo: *mut CONSOLE_HISTORY_INFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleHistoryInfo(lpconsolehistoryinfo: *mut CONSOLE_HISTORY_INFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetConsoleHistoryInfo(::core::mem::transmute(lpconsolehistoryinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleMode<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsolehandle: Param0, lpmode: *mut CONSOLE_MODE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleMode(hconsolehandle: ::win32_foundation::HANDLE, lpmode: *mut CONSOLE_MODE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetConsoleMode(hconsolehandle.into_param().abi(), ::core::mem::transmute(lpmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleOriginalTitleA(lpconsoletitle: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleOriginalTitleA(lpconsoletitle: ::windows_core::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetConsoleOriginalTitleA(::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpconsoletitle)), lpconsoletitle.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleOriginalTitleW(lpconsoletitle: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleOriginalTitleW(lpconsoletitle: ::windows_core::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetConsoleOriginalTitleW(::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpconsoletitle)), lpconsoletitle.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleOutputCP() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleOutputCP() -> u32;
        }
        ::core::mem::transmute(GetConsoleOutputCP())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleProcessList(lpdwprocesslist: &mut [u32]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleProcessList(lpdwprocesslist: *mut u32, dwprocesscount: u32) -> u32;
        }
        ::core::mem::transmute(GetConsoleProcessList(::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpdwprocesslist)), lpdwprocesslist.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleScreenBufferInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleoutput: Param0, lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleScreenBufferInfo(hconsoleoutput: ::win32_foundation::HANDLE, lpconsolescreenbufferinfo: *mut CONSOLE_SCREEN_BUFFER_INFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetConsoleScreenBufferInfo(hconsoleoutput.into_param().abi(), ::core::mem::transmute(lpconsolescreenbufferinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleScreenBufferInfoEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleoutput: Param0, lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleScreenBufferInfoEx(hconsoleoutput: ::win32_foundation::HANDLE, lpconsolescreenbufferinfoex: *mut CONSOLE_SCREEN_BUFFER_INFOEX) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetConsoleScreenBufferInfoEx(hconsoleoutput.into_param().abi(), ::core::mem::transmute(lpconsolescreenbufferinfoex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleSelectionInfo(lpconsoleselectioninfo: *mut CONSOLE_SELECTION_INFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleSelectionInfo(lpconsoleselectioninfo: *mut CONSOLE_SELECTION_INFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetConsoleSelectionInfo(::core::mem::transmute(lpconsoleselectioninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleTitleA(lpconsoletitle: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleTitleA(lpconsoletitle: ::windows_core::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetConsoleTitleA(::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpconsoletitle)), lpconsoletitle.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleTitleW(lpconsoletitle: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleTitleW(lpconsoletitle: ::windows_core::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetConsoleTitleW(::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpconsoletitle)), lpconsoletitle.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetConsoleWindow() -> ::win32_foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetConsoleWindow() -> ::win32_foundation::HWND;
        }
        ::core::mem::transmute(GetConsoleWindow())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetCurrentConsoleFont<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hconsoleoutput: Param0, bmaximumwindow: Param1, lpconsolecurrentfont: *mut CONSOLE_FONT_INFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentConsoleFont(hconsoleoutput: ::win32_foundation::HANDLE, bmaximumwindow: ::win32_foundation::BOOL, lpconsolecurrentfont: *mut CONSOLE_FONT_INFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetCurrentConsoleFont(hconsoleoutput.into_param().abi(), bmaximumwindow.into_param().abi(), ::core::mem::transmute(lpconsolecurrentfont)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetCurrentConsoleFontEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hconsoleoutput: Param0, bmaximumwindow: Param1, lpconsolecurrentfontex: *mut CONSOLE_FONT_INFOEX) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentConsoleFontEx(hconsoleoutput: ::win32_foundation::HANDLE, bmaximumwindow: ::win32_foundation::BOOL, lpconsolecurrentfontex: *mut CONSOLE_FONT_INFOEX) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetCurrentConsoleFontEx(hconsoleoutput.into_param().abi(), bmaximumwindow.into_param().abi(), ::core::mem::transmute(lpconsolecurrentfontex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetLargestConsoleWindowSize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleoutput: Param0) -> COORD {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLargestConsoleWindowSize(hconsoleoutput: ::win32_foundation::HANDLE) -> COORD;
        }
        ::core::mem::transmute(GetLargestConsoleWindowSize(hconsoleoutput.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetNumberOfConsoleInputEvents<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleinput: Param0, lpnumberofevents: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumberOfConsoleInputEvents(hconsoleinput: ::win32_foundation::HANDLE, lpnumberofevents: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetNumberOfConsoleInputEvents(hconsoleinput.into_param().abi(), ::core::mem::transmute(lpnumberofevents)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumberOfConsoleMouseButtons(lpnumberofmousebuttons: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetNumberOfConsoleMouseButtons(::core::mem::transmute(lpnumberofmousebuttons)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetStdHandle(nstdhandle: STD_HANDLE) -> ::windows_core::Result<::win32_foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStdHandle(nstdhandle: STD_HANDLE) -> ::win32_foundation::HANDLE;
        }
        let result__ = GetStdHandle(::core::mem::transmute(nstdhandle));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const HISTORY_NO_DUP_FLAG: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HPCON(pub isize);
impl HPCON {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HPCON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HPCON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HPCON {}
impl ::core::fmt::Debug for HPCON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HPCON").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HPCON {
    type Abi = Self;
}
#[repr(C)]
pub struct INPUT_RECORD {
    pub EventType: u16,
    pub Event: INPUT_RECORD_0,
}
impl ::core::marker::Copy for INPUT_RECORD {}
impl ::core::clone::Clone for INPUT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for INPUT_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INPUT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUT_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for INPUT_RECORD {}
impl ::core::default::Default for INPUT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union INPUT_RECORD_0 {
    pub KeyEvent: KEY_EVENT_RECORD,
    pub MouseEvent: MOUSE_EVENT_RECORD,
    pub WindowBufferSizeEvent: WINDOW_BUFFER_SIZE_RECORD,
    pub MenuEvent: MENU_EVENT_RECORD,
    pub FocusEvent: FOCUS_EVENT_RECORD,
}
impl ::core::marker::Copy for INPUT_RECORD_0 {}
impl ::core::clone::Clone for INPUT_RECORD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for INPUT_RECORD_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INPUT_RECORD_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INPUT_RECORD_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for INPUT_RECORD_0 {}
impl ::core::default::Default for INPUT_RECORD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const KEY_EVENT: u32 = 1u32;
#[repr(C)]
pub struct KEY_EVENT_RECORD {
    pub bKeyDown: ::win32_foundation::BOOL,
    pub wRepeatCount: u16,
    pub wVirtualKeyCode: u16,
    pub wVirtualScanCode: u16,
    pub uChar: KEY_EVENT_RECORD_0,
    pub dwControlKeyState: u32,
}
impl ::core::marker::Copy for KEY_EVENT_RECORD {}
impl ::core::clone::Clone for KEY_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for KEY_EVENT_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KEY_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KEY_EVENT_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for KEY_EVENT_RECORD {}
impl ::core::default::Default for KEY_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union KEY_EVENT_RECORD_0 {
    pub UnicodeChar: u16,
    pub AsciiChar: ::win32_foundation::CHAR,
}
impl ::core::marker::Copy for KEY_EVENT_RECORD_0 {}
impl ::core::clone::Clone for KEY_EVENT_RECORD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for KEY_EVENT_RECORD_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KEY_EVENT_RECORD_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KEY_EVENT_RECORD_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for KEY_EVENT_RECORD_0 {}
impl ::core::default::Default for KEY_EVENT_RECORD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const LEFT_ALT_PRESSED: u32 = 2u32;
pub const LEFT_CTRL_PRESSED: u32 = 8u32;
pub const MENU_EVENT: u32 = 8u32;
#[repr(C)]
pub struct MENU_EVENT_RECORD {
    pub dwCommandId: u32,
}
impl ::core::marker::Copy for MENU_EVENT_RECORD {}
impl ::core::clone::Clone for MENU_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MENU_EVENT_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENU_EVENT_RECORD").field("dwCommandId", &self.dwCommandId).finish()
    }
}
unsafe impl ::windows_core::Abi for MENU_EVENT_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MENU_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MENU_EVENT_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for MENU_EVENT_RECORD {}
impl ::core::default::Default for MENU_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const MOUSE_EVENT: u32 = 2u32;
#[repr(C)]
pub struct MOUSE_EVENT_RECORD {
    pub dwMousePosition: COORD,
    pub dwButtonState: u32,
    pub dwControlKeyState: u32,
    pub dwEventFlags: u32,
}
impl ::core::marker::Copy for MOUSE_EVENT_RECORD {}
impl ::core::clone::Clone for MOUSE_EVENT_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MOUSE_EVENT_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSE_EVENT_RECORD").field("dwMousePosition", &self.dwMousePosition).field("dwButtonState", &self.dwButtonState).field("dwControlKeyState", &self.dwControlKeyState).field("dwEventFlags", &self.dwEventFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for MOUSE_EVENT_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MOUSE_EVENT_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MOUSE_EVENT_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for MOUSE_EVENT_RECORD {}
impl ::core::default::Default for MOUSE_EVENT_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const MOUSE_HWHEELED: u32 = 8u32;
pub const MOUSE_MOVED: u32 = 1u32;
pub const MOUSE_WHEELED: u32 = 4u32;
pub const NLS_ALPHANUMERIC: u32 = 0u32;
pub const NLS_DBCSCHAR: u32 = 65536u32;
pub const NLS_HIRAGANA: u32 = 262144u32;
pub const NLS_IME_CONVERSION: u32 = 8388608u32;
pub const NLS_IME_DISABLE: u32 = 536870912u32;
pub const NLS_KATAKANA: u32 = 131072u32;
pub const NLS_ROMAN: u32 = 4194304u32;
pub const NUMLOCK_ON: u32 = 32u32;
pub type PHANDLER_ROUTINE = ::core::option::Option<unsafe extern "system" fn(ctrltype: u32) -> ::win32_foundation::BOOL>;
pub const PSEUDOCONSOLE_INHERIT_CURSOR: u32 = 1u32;
#[inline]
pub unsafe fn PeekConsoleInputA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleinput: Param0, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeekConsoleInputA(hconsoleinput: ::win32_foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(PeekConsoleInputA(hconsoleinput.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofeventsread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn PeekConsoleInputW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleinput: Param0, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PeekConsoleInputW(hconsoleinput: ::win32_foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(PeekConsoleInputW(hconsoleinput.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofeventsread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const RIGHTMOST_BUTTON_PRESSED: u32 = 2u32;
pub const RIGHT_ALT_PRESSED: u32 = 1u32;
pub const RIGHT_CTRL_PRESSED: u32 = 4u32;
#[inline]
pub unsafe fn ReadConsoleA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleinput: Param0, lpbuffer: *mut ::core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleA(hconsoleinput: ::win32_foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ReadConsoleA(hconsoleinput.into_param().abi(), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nnumberofcharstoread), ::core::mem::transmute(lpnumberofcharsread), ::core::mem::transmute(pinputcontrol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReadConsoleInputA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleinput: Param0, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleInputA(hconsoleinput: ::win32_foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ReadConsoleInputA(hconsoleinput.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofeventsread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReadConsoleInputW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleinput: Param0, lpbuffer: &mut [INPUT_RECORD], lpnumberofeventsread: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleInputW(hconsoleinput: ::win32_foundation::HANDLE, lpbuffer: *mut INPUT_RECORD, nlength: u32, lpnumberofeventsread: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ReadConsoleInputW(hconsoleinput.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofeventsread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReadConsoleOutputA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param2: ::windows_core::IntoParam<'a, COORD>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, lpbuffer: *mut CHAR_INFO, dwbuffersize: Param2, dwbuffercoord: Param3, lpreadregion: *mut SMALL_RECT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleOutputA(hconsoleoutput: ::win32_foundation::HANDLE, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ReadConsoleOutputA(hconsoleoutput.into_param().abi(), ::core::mem::transmute(lpbuffer), dwbuffersize.into_param().abi(), dwbuffercoord.into_param().abi(), ::core::mem::transmute(lpreadregion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReadConsoleOutputAttribute<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, lpattribute: &mut [u16], dwreadcoord: Param3, lpnumberofattrsread: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleOutputAttribute(hconsoleoutput: ::win32_foundation::HANDLE, lpattribute: *mut u16, nlength: u32, dwreadcoord: COORD, lpnumberofattrsread: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ReadConsoleOutputAttribute(hconsoleoutput.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpattribute)), lpattribute.len() as _, dwreadcoord.into_param().abi(), ::core::mem::transmute(lpnumberofattrsread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReadConsoleOutputCharacterA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, lpcharacter: &mut [u8], dwreadcoord: Param3, lpnumberofcharsread: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleOutputCharacterA(hconsoleoutput: ::win32_foundation::HANDLE, lpcharacter: ::windows_core::PSTR, nlength: u32, dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ReadConsoleOutputCharacterA(hconsoleoutput.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpcharacter)), lpcharacter.len() as _, dwreadcoord.into_param().abi(), ::core::mem::transmute(lpnumberofcharsread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReadConsoleOutputCharacterW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, lpcharacter: &mut [u16], dwreadcoord: Param3, lpnumberofcharsread: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleOutputCharacterW(hconsoleoutput: ::win32_foundation::HANDLE, lpcharacter: ::windows_core::PWSTR, nlength: u32, dwreadcoord: COORD, lpnumberofcharsread: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ReadConsoleOutputCharacterW(hconsoleoutput.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpcharacter)), lpcharacter.len() as _, dwreadcoord.into_param().abi(), ::core::mem::transmute(lpnumberofcharsread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReadConsoleOutputW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param2: ::windows_core::IntoParam<'a, COORD>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, lpbuffer: *mut CHAR_INFO, dwbuffersize: Param2, dwbuffercoord: Param3, lpreadregion: *mut SMALL_RECT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleOutputW(hconsoleoutput: ::win32_foundation::HANDLE, lpbuffer: *mut CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpreadregion: *mut SMALL_RECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ReadConsoleOutputW(hconsoleoutput.into_param().abi(), ::core::mem::transmute(lpbuffer), dwbuffersize.into_param().abi(), dwbuffercoord.into_param().abi(), ::core::mem::transmute(lpreadregion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReadConsoleW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleinput: Param0, lpbuffer: *mut ::core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadConsoleW(hconsoleinput: ::win32_foundation::HANDLE, lpbuffer: *mut ::core::ffi::c_void, nnumberofcharstoread: u32, lpnumberofcharsread: *mut u32, pinputcontrol: *const CONSOLE_READCONSOLE_CONTROL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ReadConsoleW(hconsoleinput.into_param().abi(), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(nnumberofcharstoread), ::core::mem::transmute(lpnumberofcharsread), ::core::mem::transmute(pinputcontrol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ResizePseudoConsole<'a, Param0: ::windows_core::IntoParam<'a, HPCON>, Param1: ::windows_core::IntoParam<'a, COORD>>(hpc: Param0, size: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResizePseudoConsole(hpc: HPCON, size: COORD) -> ::windows_core::HRESULT;
        }
        ResizePseudoConsole(hpc.into_param().abi(), size.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SCROLLLOCK_ON: u32 = 64u32;
pub const SHIFT_PRESSED: u32 = 16u32;
#[repr(C)]
pub struct SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl ::core::marker::Copy for SMALL_RECT {}
impl ::core::clone::Clone for SMALL_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SMALL_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMALL_RECT").field("Left", &self.Left).field("Top", &self.Top).field("Right", &self.Right).field("Bottom", &self.Bottom).finish()
    }
}
unsafe impl ::windows_core::Abi for SMALL_RECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SMALL_RECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SMALL_RECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SMALL_RECT {}
impl ::core::default::Default for SMALL_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STD_HANDLE(pub u32);
pub const STD_INPUT_HANDLE: STD_HANDLE = STD_HANDLE(4294967286u32);
pub const STD_OUTPUT_HANDLE: STD_HANDLE = STD_HANDLE(4294967285u32);
pub const STD_ERROR_HANDLE: STD_HANDLE = STD_HANDLE(4294967284u32);
impl ::core::marker::Copy for STD_HANDLE {}
impl ::core::clone::Clone for STD_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STD_HANDLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for STD_HANDLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STD_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STD_HANDLE").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn ScrollConsoleScreenBufferA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: Param3, lpfill: *const CHAR_INFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScrollConsoleScreenBufferA(hconsoleoutput: ::win32_foundation::HANDLE, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ScrollConsoleScreenBufferA(hconsoleoutput.into_param().abi(), ::core::mem::transmute(lpscrollrectangle), ::core::mem::transmute(lpcliprectangle), dwdestinationorigin.into_param().abi(), ::core::mem::transmute(lpfill)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ScrollConsoleScreenBufferW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: Param3, lpfill: *const CHAR_INFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ScrollConsoleScreenBufferW(hconsoleoutput: ::win32_foundation::HANDLE, lpscrollrectangle: *const SMALL_RECT, lpcliprectangle: *const SMALL_RECT, dwdestinationorigin: COORD, lpfill: *const CHAR_INFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ScrollConsoleScreenBufferW(hconsoleoutput.into_param().abi(), ::core::mem::transmute(lpscrollrectangle), ::core::mem::transmute(lpcliprectangle), dwdestinationorigin.into_param().abi(), ::core::mem::transmute(lpfill)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleActiveScreenBuffer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleoutput: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleActiveScreenBuffer(hconsoleoutput: ::win32_foundation::HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleActiveScreenBuffer(hconsoleoutput.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleCP(wcodepageid: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleCP(wcodepageid: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleCP(::core::mem::transmute(wcodepageid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleCtrlHandler<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(handlerroutine: PHANDLER_ROUTINE, add: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleCtrlHandler(handlerroutine: ::windows_core::RawPtr, add: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleCtrlHandler(::core::mem::transmute(handlerroutine), add.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleCursorInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleoutput: Param0, lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleCursorInfo(hconsoleoutput: ::win32_foundation::HANDLE, lpconsolecursorinfo: *const CONSOLE_CURSOR_INFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleCursorInfo(hconsoleoutput.into_param().abi(), ::core::mem::transmute(lpconsolecursorinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleCursorPosition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, dwcursorposition: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleCursorPosition(hconsoleoutput: ::win32_foundation::HANDLE, dwcursorposition: COORD) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleCursorPosition(hconsoleoutput.into_param().abi(), dwcursorposition.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleDisplayMode<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleoutput: Param0, dwflags: u32, lpnewscreenbufferdimensions: *mut COORD) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleDisplayMode(hconsoleoutput: ::win32_foundation::HANDLE, dwflags: u32, lpnewscreenbufferdimensions: *mut COORD) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleDisplayMode(hconsoleoutput.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpnewscreenbufferdimensions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleHistoryInfo(lpconsolehistoryinfo: *const CONSOLE_HISTORY_INFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleHistoryInfo(lpconsolehistoryinfo: *const CONSOLE_HISTORY_INFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleHistoryInfo(::core::mem::transmute(lpconsolehistoryinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleMode<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsolehandle: Param0, dwmode: CONSOLE_MODE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleMode(hconsolehandle: ::win32_foundation::HANDLE, dwmode: CONSOLE_MODE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleMode(hconsolehandle.into_param().abi(), ::core::mem::transmute(dwmode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleNumberOfCommandsA<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(number: u32, exename: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleNumberOfCommandsA(number: u32, exename: ::windows_core::PCSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleNumberOfCommandsA(::core::mem::transmute(number), exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleNumberOfCommandsW<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(number: u32, exename: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleNumberOfCommandsW(number: u32, exename: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleNumberOfCommandsW(::core::mem::transmute(number), exename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleOutputCP(wcodepageid: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleOutputCP(wcodepageid: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleOutputCP(::core::mem::transmute(wcodepageid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleScreenBufferInfoEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleoutput: Param0, lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleScreenBufferInfoEx(hconsoleoutput: ::win32_foundation::HANDLE, lpconsolescreenbufferinfoex: *const CONSOLE_SCREEN_BUFFER_INFOEX) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleScreenBufferInfoEx(hconsoleoutput.into_param().abi(), ::core::mem::transmute(lpconsolescreenbufferinfoex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleScreenBufferSize<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, dwsize: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleScreenBufferSize(hconsoleoutput: ::win32_foundation::HANDLE, dwsize: COORD) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleScreenBufferSize(hconsoleoutput.into_param().abi(), dwsize.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleTextAttribute<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleoutput: Param0, wattributes: u16) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleTextAttribute(hconsoleoutput: ::win32_foundation::HANDLE, wattributes: u16) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleTextAttribute(hconsoleoutput.into_param().abi(), ::core::mem::transmute(wattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleTitleA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(lpconsoletitle: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleTitleA(lpconsoletitle: ::windows_core::PCSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleTitleA(lpconsoletitle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleTitleW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpconsoletitle: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleTitleW(lpconsoletitle: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleTitleW(lpconsoletitle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetConsoleWindowInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hconsoleoutput: Param0, babsolute: Param1, lpconsolewindow: *const SMALL_RECT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetConsoleWindowInfo(hconsoleoutput: ::win32_foundation::HANDLE, babsolute: ::win32_foundation::BOOL, lpconsolewindow: *const SMALL_RECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetConsoleWindowInfo(hconsoleoutput.into_param().abi(), babsolute.into_param().abi(), ::core::mem::transmute(lpconsolewindow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetCurrentConsoleFontEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hconsoleoutput: Param0, bmaximumwindow: Param1, lpconsolecurrentfontex: *const CONSOLE_FONT_INFOEX) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCurrentConsoleFontEx(hconsoleoutput: ::win32_foundation::HANDLE, bmaximumwindow: ::win32_foundation::BOOL, lpconsolecurrentfontex: *const CONSOLE_FONT_INFOEX) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetCurrentConsoleFontEx(hconsoleoutput.into_param().abi(), bmaximumwindow.into_param().abi(), ::core::mem::transmute(lpconsolecurrentfontex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetStdHandle<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(nstdhandle: STD_HANDLE, hhandle: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetStdHandle(nstdhandle: STD_HANDLE, hhandle: ::win32_foundation::HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetStdHandle(::core::mem::transmute(nstdhandle), hhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetStdHandleEx<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(nstdhandle: STD_HANDLE, hhandle: Param1, phprevvalue: *mut ::win32_foundation::HANDLE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetStdHandleEx(nstdhandle: STD_HANDLE, hhandle: ::win32_foundation::HANDLE, phprevvalue: *mut ::win32_foundation::HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetStdHandleEx(::core::mem::transmute(nstdhandle), hhandle.into_param().abi(), ::core::mem::transmute(phprevvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WINDOW_BUFFER_SIZE_EVENT: u32 = 4u32;
#[repr(C)]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub dwSize: COORD,
}
impl ::core::marker::Copy for WINDOW_BUFFER_SIZE_RECORD {}
impl ::core::clone::Clone for WINDOW_BUFFER_SIZE_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WINDOW_BUFFER_SIZE_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOW_BUFFER_SIZE_RECORD").field("dwSize", &self.dwSize).finish()
    }
}
unsafe impl ::windows_core::Abi for WINDOW_BUFFER_SIZE_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WINDOW_BUFFER_SIZE_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WINDOW_BUFFER_SIZE_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for WINDOW_BUFFER_SIZE_RECORD {}
impl ::core::default::Default for WINDOW_BUFFER_SIZE_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn WriteConsoleA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleoutput: Param0, lpbuffer: &[u8], lpnumberofcharswritten: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleA(hconsoleoutput: ::win32_foundation::HANDLE, lpbuffer: *const ::core::ffi::c_void, nnumberofcharstowrite: u32, lpnumberofcharswritten: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WriteConsoleA(hconsoleoutput.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofcharswritten), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WriteConsoleInputA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleinput: Param0, lpbuffer: &[INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleInputA(hconsoleinput: ::win32_foundation::HANDLE, lpbuffer: *const INPUT_RECORD, nlength: u32, lpnumberofeventswritten: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WriteConsoleInputA(hconsoleinput.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofeventswritten)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WriteConsoleInputW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleinput: Param0, lpbuffer: &[INPUT_RECORD], lpnumberofeventswritten: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleInputW(hconsoleinput: ::win32_foundation::HANDLE, lpbuffer: *const INPUT_RECORD, nlength: u32, lpnumberofeventswritten: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WriteConsoleInputW(hconsoleinput.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofeventswritten)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WriteConsoleOutputA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param2: ::windows_core::IntoParam<'a, COORD>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, lpbuffer: *const CHAR_INFO, dwbuffersize: Param2, dwbuffercoord: Param3, lpwriteregion: *mut SMALL_RECT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleOutputA(hconsoleoutput: ::win32_foundation::HANDLE, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WriteConsoleOutputA(hconsoleoutput.into_param().abi(), ::core::mem::transmute(lpbuffer), dwbuffersize.into_param().abi(), dwbuffercoord.into_param().abi(), ::core::mem::transmute(lpwriteregion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WriteConsoleOutputAttribute<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, lpattribute: &[u16], dwwritecoord: Param3, lpnumberofattrswritten: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleOutputAttribute(hconsoleoutput: ::win32_foundation::HANDLE, lpattribute: *const u16, nlength: u32, dwwritecoord: COORD, lpnumberofattrswritten: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WriteConsoleOutputAttribute(hconsoleoutput.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(lpattribute)), lpattribute.len() as _, dwwritecoord.into_param().abi(), ::core::mem::transmute(lpnumberofattrswritten)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WriteConsoleOutputCharacterA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, lpcharacter: &[u8], dwwritecoord: Param3, lpnumberofcharswritten: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleOutputCharacterA(hconsoleoutput: ::win32_foundation::HANDLE, lpcharacter: ::windows_core::PCSTR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WriteConsoleOutputCharacterA(hconsoleoutput.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(lpcharacter)), lpcharacter.len() as _, dwwritecoord.into_param().abi(), ::core::mem::transmute(lpnumberofcharswritten)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WriteConsoleOutputCharacterW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, lpcharacter: &[u16], dwwritecoord: Param3, lpnumberofcharswritten: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleOutputCharacterW(hconsoleoutput: ::win32_foundation::HANDLE, lpcharacter: ::windows_core::PCWSTR, nlength: u32, dwwritecoord: COORD, lpnumberofcharswritten: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WriteConsoleOutputCharacterW(hconsoleoutput.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(lpcharacter)), lpcharacter.len() as _, dwwritecoord.into_param().abi(), ::core::mem::transmute(lpnumberofcharswritten)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WriteConsoleOutputW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param2: ::windows_core::IntoParam<'a, COORD>, Param3: ::windows_core::IntoParam<'a, COORD>>(hconsoleoutput: Param0, lpbuffer: *const CHAR_INFO, dwbuffersize: Param2, dwbuffercoord: Param3, lpwriteregion: *mut SMALL_RECT) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleOutputW(hconsoleoutput: ::win32_foundation::HANDLE, lpbuffer: *const CHAR_INFO, dwbuffersize: COORD, dwbuffercoord: COORD, lpwriteregion: *mut SMALL_RECT) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WriteConsoleOutputW(hconsoleoutput.into_param().abi(), ::core::mem::transmute(lpbuffer), dwbuffersize.into_param().abi(), dwbuffercoord.into_param().abi(), ::core::mem::transmute(lpwriteregion)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WriteConsoleW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hconsoleoutput: Param0, lpbuffer: &[u8], lpnumberofcharswritten: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteConsoleW(hconsoleoutput: ::win32_foundation::HANDLE, lpbuffer: *const ::core::ffi::c_void, nnumberofcharstowrite: u32, lpnumberofcharswritten: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(WriteConsoleW(hconsoleoutput.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(lpbuffer)), lpbuffer.len() as _, ::core::mem::transmute(lpnumberofcharswritten), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
