#[cfg(feature = "Dialogs")]
pub mod Dialogs;
#[cfg(feature = "RichEdit")]
pub mod RichEdit;
pub const ACM_ISPLAYING: u32 = 1128u32;
pub const ACM_OPEN: u32 = 1127u32;
pub const ACM_OPENA: u32 = 1124u32;
pub const ACM_OPENW: u32 = 1127u32;
pub const ACM_PLAY: u32 = 1125u32;
pub const ACM_STOP: u32 = 1126u32;
pub const ACN_START: u32 = 1u32;
pub const ACN_STOP: u32 = 2u32;
pub const ACS_AUTOPLAY: u32 = 4u32;
pub const ACS_CENTER: u32 = 1u32;
pub const ACS_TIMER: u32 = 8u32;
pub const ACS_TRANSPARENT: u32 = 2u32;
pub const ANIMATE_CLASS: &str = "SysAnimate32";
pub const ANIMATE_CLASSA: &str = "SysAnimate32";
pub const ANIMATE_CLASSW: &str = "SysAnimate32";
pub const BCM_FIRST: u32 = 5632u32;
pub const BCM_GETIDEALSIZE: u32 = 5633u32;
pub const BCM_GETIMAGELIST: u32 = 5635u32;
pub const BCM_GETNOTE: u32 = 5642u32;
pub const BCM_GETNOTELENGTH: u32 = 5643u32;
pub const BCM_GETSPLITINFO: u32 = 5640u32;
pub const BCM_GETTEXTMARGIN: u32 = 5637u32;
pub const BCM_SETDROPDOWNSTATE: u32 = 5638u32;
pub const BCM_SETIMAGELIST: u32 = 5634u32;
pub const BCM_SETNOTE: u32 = 5641u32;
pub const BCM_SETSHIELD: u32 = 5644u32;
pub const BCM_SETSPLITINFO: u32 = 5639u32;
pub const BCM_SETTEXTMARGIN: u32 = 5636u32;
pub const BCN_DROPDOWN: u32 = 4294966048u32;
pub const BCN_FIRST: u32 = 4294966046u32;
pub const BCN_HOTITEMCHANGE: u32 = 4294966047u32;
pub const BCSIF_GLYPH: u32 = 1u32;
pub const BCSIF_IMAGE: u32 = 2u32;
pub const BCSIF_SIZE: u32 = 8u32;
pub const BCSIF_STYLE: u32 = 4u32;
pub const BCSS_ALIGNLEFT: u32 = 4u32;
pub const BCSS_IMAGE: u32 = 8u32;
pub const BCSS_NOSPLIT: u32 = 1u32;
pub const BCSS_STRETCH: u32 = 2u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BGTYPE(pub i32);
pub const BT_IMAGEFILE: BGTYPE = BGTYPE(0i32);
pub const BT_BORDERFILL: BGTYPE = BGTYPE(1i32);
pub const BT_NONE: BGTYPE = BGTYPE(2i32);
impl ::core::marker::Copy for BGTYPE {}
impl ::core::clone::Clone for BGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BGTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for BGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BGTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BORDERTYPE(pub i32);
pub const BT_RECT: BORDERTYPE = BORDERTYPE(0i32);
pub const BT_ROUNDRECT: BORDERTYPE = BORDERTYPE(1i32);
pub const BT_ELLIPSE: BORDERTYPE = BORDERTYPE(2i32);
impl ::core::marker::Copy for BORDERTYPE {}
impl ::core::clone::Clone for BORDERTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BORDERTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BORDERTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for BORDERTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BORDERTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct BP_ANIMATIONPARAMS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub style: BP_ANIMATIONSTYLE,
    pub dwDuration: u32,
}
impl ::core::marker::Copy for BP_ANIMATIONPARAMS {}
impl ::core::clone::Clone for BP_ANIMATIONPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BP_ANIMATIONPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BP_ANIMATIONPARAMS").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("style", &self.style).field("dwDuration", &self.dwDuration).finish()
    }
}
unsafe impl ::windows_core::Abi for BP_ANIMATIONPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BP_ANIMATIONPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BP_ANIMATIONPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for BP_ANIMATIONPARAMS {}
impl ::core::default::Default for BP_ANIMATIONPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BP_ANIMATIONSTYLE(pub i32);
pub const BPAS_NONE: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(0i32);
pub const BPAS_LINEAR: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(1i32);
pub const BPAS_CUBIC: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(2i32);
pub const BPAS_SINE: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(3i32);
impl ::core::marker::Copy for BP_ANIMATIONSTYLE {}
impl ::core::clone::Clone for BP_ANIMATIONSTYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BP_ANIMATIONSTYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BP_ANIMATIONSTYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for BP_ANIMATIONSTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BP_ANIMATIONSTYLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BP_BUFFERFORMAT(pub i32);
pub const BPBF_COMPATIBLEBITMAP: BP_BUFFERFORMAT = BP_BUFFERFORMAT(0i32);
pub const BPBF_DIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(1i32);
pub const BPBF_TOPDOWNDIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(2i32);
pub const BPBF_TOPDOWNMONODIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(3i32);
impl ::core::marker::Copy for BP_BUFFERFORMAT {}
impl ::core::clone::Clone for BP_BUFFERFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BP_BUFFERFORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BP_BUFFERFORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for BP_BUFFERFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BP_BUFFERFORMAT").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct BP_PAINTPARAMS {
    pub cbSize: u32,
    pub dwFlags: BP_PAINTPARAMS_FLAGS,
    pub prcExclude: *const ::win32_foundation::RECT,
    pub pBlendFunction: *const ::win32_graphics::Gdi::BLENDFUNCTION,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for BP_PAINTPARAMS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for BP_PAINTPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for BP_PAINTPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BP_PAINTPARAMS").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("prcExclude", &self.prcExclude).field("pBlendFunction", &self.pBlendFunction).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for BP_PAINTPARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for BP_PAINTPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BP_PAINTPARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for BP_PAINTPARAMS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for BP_PAINTPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BP_PAINTPARAMS_FLAGS(pub u32);
pub const BPPF_ERASE: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(1u32);
pub const BPPF_NOCLIP: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(2u32);
pub const BPPF_NONCLIENT: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(4u32);
impl ::core::marker::Copy for BP_PAINTPARAMS_FLAGS {}
impl ::core::clone::Clone for BP_PAINTPARAMS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BP_PAINTPARAMS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BP_PAINTPARAMS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for BP_PAINTPARAMS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BP_PAINTPARAMS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for BP_PAINTPARAMS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for BP_PAINTPARAMS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for BP_PAINTPARAMS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for BP_PAINTPARAMS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for BP_PAINTPARAMS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const BST_DROPDOWNPUSHED: u32 = 1024u32;
pub const BST_HOT: u32 = 512u32;
pub const BS_COMMANDLINK: i32 = 14i32;
pub const BS_DEFCOMMANDLINK: i32 = 15i32;
pub const BS_DEFSPLITBUTTON: i32 = 13i32;
pub const BS_SPLITBUTTON: i32 = 12i32;
pub const BTNS_AUTOSIZE: u32 = 16u32;
pub const BTNS_BUTTON: u32 = 0u32;
pub const BTNS_CHECK: u32 = 2u32;
pub const BTNS_DROPDOWN: u32 = 8u32;
pub const BTNS_GROUP: u32 = 4u32;
pub const BTNS_NOPREFIX: u32 = 32u32;
pub const BTNS_SEP: u32 = 1u32;
pub const BTNS_SHOWTEXT: u32 = 64u32;
pub const BTNS_WHOLEDROPDOWN: u32 = 128u32;
#[repr(C)]
pub struct BUTTON_IMAGELIST {
    pub himl: HIMAGELIST,
    pub margin: ::win32_foundation::RECT,
    pub uAlign: BUTTON_IMAGELIST_ALIGN,
}
impl ::core::marker::Copy for BUTTON_IMAGELIST {}
impl ::core::clone::Clone for BUTTON_IMAGELIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BUTTON_IMAGELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BUTTON_IMAGELIST").field("himl", &self.himl).field("margin", &self.margin).field("uAlign", &self.uAlign).finish()
    }
}
unsafe impl ::windows_core::Abi for BUTTON_IMAGELIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BUTTON_IMAGELIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BUTTON_IMAGELIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for BUTTON_IMAGELIST {}
impl ::core::default::Default for BUTTON_IMAGELIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BUTTON_IMAGELIST_ALIGN(pub u32);
pub const BUTTON_IMAGELIST_ALIGN_LEFT: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(0u32);
pub const BUTTON_IMAGELIST_ALIGN_RIGHT: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(1u32);
pub const BUTTON_IMAGELIST_ALIGN_TOP: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(2u32);
pub const BUTTON_IMAGELIST_ALIGN_BOTTOM: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(3u32);
pub const BUTTON_IMAGELIST_ALIGN_CENTER: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(4u32);
impl ::core::marker::Copy for BUTTON_IMAGELIST_ALIGN {}
impl ::core::clone::Clone for BUTTON_IMAGELIST_ALIGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BUTTON_IMAGELIST_ALIGN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BUTTON_IMAGELIST_ALIGN {
    type Abi = Self;
}
impl ::core::fmt::Debug for BUTTON_IMAGELIST_ALIGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BUTTON_IMAGELIST_ALIGN").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct BUTTON_SPLITINFO {
    pub mask: u32,
    pub himlGlyph: HIMAGELIST,
    pub uSplitStyle: u32,
    pub size: ::win32_foundation::SIZE,
}
impl ::core::marker::Copy for BUTTON_SPLITINFO {}
impl ::core::clone::Clone for BUTTON_SPLITINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BUTTON_SPLITINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BUTTON_SPLITINFO").field("mask", &self.mask).field("himlGlyph", &self.himlGlyph).field("uSplitStyle", &self.uSplitStyle).field("size", &self.size).finish()
    }
}
unsafe impl ::windows_core::Abi for BUTTON_SPLITINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BUTTON_SPLITINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BUTTON_SPLITINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for BUTTON_SPLITINFO {}
impl ::core::default::Default for BUTTON_SPLITINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn BeginBufferedAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hwnd: Param0, hdctarget: Param1, prctarget: *const ::win32_foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: *const BP_PAINTPARAMS, panimationparams: *const BP_ANIMATIONPARAMS, phdcfrom: *mut ::win32_graphics::Gdi::HDC, phdcto: *mut ::win32_graphics::Gdi::HDC) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BeginBufferedAnimation(hwnd: ::win32_foundation::HWND, hdctarget: ::win32_graphics::Gdi::HDC, prctarget: *const ::win32_foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: *const BP_PAINTPARAMS, panimationparams: *const BP_ANIMATIONPARAMS, phdcfrom: *mut ::win32_graphics::Gdi::HDC, phdcto: *mut ::win32_graphics::Gdi::HDC) -> isize;
        }
        ::core::mem::transmute(BeginBufferedAnimation(hwnd.into_param().abi(), hdctarget.into_param().abi(), ::core::mem::transmute(prctarget), ::core::mem::transmute(dwformat), ::core::mem::transmute(ppaintparams), ::core::mem::transmute(panimationparams), ::core::mem::transmute(phdcfrom), ::core::mem::transmute(phdcto)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn BeginBufferedPaint<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hdctarget: Param0, prctarget: *const ::win32_foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: *const BP_PAINTPARAMS, phdc: *mut ::win32_graphics::Gdi::HDC) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BeginBufferedPaint(hdctarget: ::win32_graphics::Gdi::HDC, prctarget: *const ::win32_foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: *const BP_PAINTPARAMS, phdc: *mut ::win32_graphics::Gdi::HDC) -> isize;
        }
        ::core::mem::transmute(BeginBufferedPaint(hdctarget.into_param().abi(), ::core::mem::transmute(prctarget), ::core::mem::transmute(dwformat), ::core::mem::transmute(ppaintparams), ::core::mem::transmute(phdc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BeginPanningFeedback<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BeginPanningFeedback(hwnd: ::win32_foundation::HWND) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(BeginPanningFeedback(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BufferedPaintClear(hbufferedpaint: isize, prc: *const ::win32_foundation::RECT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferedPaintClear(hbufferedpaint: isize, prc: *const ::win32_foundation::RECT) -> ::windows_core::HRESULT;
        }
        BufferedPaintClear(::core::mem::transmute(hbufferedpaint), ::core::mem::transmute(prc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BufferedPaintInit() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferedPaintInit() -> ::windows_core::HRESULT;
        }
        BufferedPaintInit().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn BufferedPaintRenderAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hwnd: Param0, hdctarget: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferedPaintRenderAnimation(hwnd: ::win32_foundation::HWND, hdctarget: ::win32_graphics::Gdi::HDC) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(BufferedPaintRenderAnimation(hwnd.into_param().abi(), hdctarget.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BufferedPaintSetAlpha(hbufferedpaint: isize, prc: *const ::win32_foundation::RECT, alpha: u8) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferedPaintSetAlpha(hbufferedpaint: isize, prc: *const ::win32_foundation::RECT, alpha: u8) -> ::windows_core::HRESULT;
        }
        BufferedPaintSetAlpha(::core::mem::transmute(hbufferedpaint), ::core::mem::transmute(prc), ::core::mem::transmute(alpha)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BufferedPaintStopAllAnimations<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferedPaintStopAllAnimations(hwnd: ::win32_foundation::HWND) -> ::windows_core::HRESULT;
        }
        BufferedPaintStopAllAnimations(hwnd.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BufferedPaintUnInit() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferedPaintUnInit() -> ::windows_core::HRESULT;
        }
        BufferedPaintUnInit().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const CBEMAXSTRLEN: u32 = 260u32;
pub const CBEM_GETCOMBOCONTROL: u32 = 1030u32;
pub const CBEM_GETEDITCONTROL: u32 = 1031u32;
pub const CBEM_GETEXSTYLE: u32 = 1033u32;
pub const CBEM_GETEXTENDEDSTYLE: u32 = 1033u32;
pub const CBEM_GETIMAGELIST: u32 = 1027u32;
pub const CBEM_GETITEM: u32 = 1037u32;
pub const CBEM_GETITEMA: u32 = 1028u32;
pub const CBEM_GETITEMW: u32 = 1037u32;
pub const CBEM_GETUNICODEFORMAT: u32 = 8198u32;
pub const CBEM_HASEDITCHANGED: u32 = 1034u32;
pub const CBEM_INSERTITEM: u32 = 1035u32;
pub const CBEM_INSERTITEMA: u32 = 1025u32;
pub const CBEM_INSERTITEMW: u32 = 1035u32;
pub const CBEM_SETEXSTYLE: u32 = 1032u32;
pub const CBEM_SETEXTENDEDSTYLE: u32 = 1038u32;
pub const CBEM_SETIMAGELIST: u32 = 1026u32;
pub const CBEM_SETITEM: u32 = 1036u32;
pub const CBEM_SETITEMA: u32 = 1029u32;
pub const CBEM_SETITEMW: u32 = 1036u32;
pub const CBEM_SETUNICODEFORMAT: u32 = 8197u32;
pub const CBEM_SETWINDOWTHEME: u32 = 8203u32;
pub const CBENF_DROPDOWN: u32 = 4u32;
pub const CBENF_ESCAPE: u32 = 3u32;
pub const CBENF_KILLFOCUS: u32 = 1u32;
pub const CBENF_RETURN: u32 = 2u32;
pub const CBES_EX_CASESENSITIVE: u32 = 16u32;
pub const CBES_EX_NOEDITIMAGE: u32 = 1u32;
pub const CBES_EX_NOEDITIMAGEINDENT: u32 = 2u32;
pub const CBES_EX_NOSIZELIMIT: u32 = 8u32;
pub const CBES_EX_PATHWORDBREAKPROC: u32 = 4u32;
pub const CBES_EX_TEXTENDELLIPSIS: u32 = 32u32;
pub const CBM_FIRST: u32 = 5888u32;
pub const CB_GETCUEBANNER: u32 = 5892u32;
pub const CB_GETMINVISIBLE: u32 = 5890u32;
pub const CB_SETCUEBANNER: u32 = 5891u32;
pub const CB_SETMINVISIBLE: u32 = 5889u32;
pub const CCF_NOTEXT: u32 = 1u32;
pub const CCHCCCLASS: u32 = 32u32;
pub const CCHCCDESC: u32 = 32u32;
pub const CCHCCTEXT: u32 = 256u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct CCINFOA {
    pub szClass: [::win32_foundation::CHAR; 32],
    pub flOptions: u32,
    pub szDesc: [::win32_foundation::CHAR; 32],
    pub cxDefault: u32,
    pub cyDefault: u32,
    pub flStyleDefault: u32,
    pub flExtStyleDefault: u32,
    pub flCtrlTypeMask: u32,
    pub szTextDefault: [::win32_foundation::CHAR; 256],
    pub cStyleFlags: i32,
    pub aStyleFlags: *mut CCSTYLEFLAGA,
    pub lpfnStyle: LPFNCCSTYLEA,
    pub lpfnSizeToText: LPFNCCSIZETOTEXTA,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for CCINFOA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for CCINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for CCINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCINFOA")
            .field("szClass", &self.szClass)
            .field("flOptions", &self.flOptions)
            .field("szDesc", &self.szDesc)
            .field("cxDefault", &self.cxDefault)
            .field("cyDefault", &self.cyDefault)
            .field("flStyleDefault", &self.flStyleDefault)
            .field("flExtStyleDefault", &self.flExtStyleDefault)
            .field("flCtrlTypeMask", &self.flCtrlTypeMask)
            .field("szTextDefault", &self.szTextDefault)
            .field("cStyleFlags", &self.cStyleFlags)
            .field("aStyleFlags", &self.aStyleFlags)
            .field("lpfnStyle", &self.lpfnStyle.map(|f| f as usize))
            .field("lpfnSizeToText", &self.lpfnSizeToText.map(|f| f as usize))
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for CCINFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for CCINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CCINFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for CCINFOA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for CCINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct CCINFOW {
    pub szClass: [u16; 32],
    pub flOptions: u32,
    pub szDesc: [u16; 32],
    pub cxDefault: u32,
    pub cyDefault: u32,
    pub flStyleDefault: u32,
    pub flExtStyleDefault: u32,
    pub flCtrlTypeMask: u32,
    pub cStyleFlags: i32,
    pub aStyleFlags: *mut CCSTYLEFLAGW,
    pub szTextDefault: [u16; 256],
    pub lpfnStyle: LPFNCCSTYLEW,
    pub lpfnSizeToText: LPFNCCSIZETOTEXTW,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for CCINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for CCINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for CCINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCINFOW")
            .field("szClass", &self.szClass)
            .field("flOptions", &self.flOptions)
            .field("szDesc", &self.szDesc)
            .field("cxDefault", &self.cxDefault)
            .field("cyDefault", &self.cyDefault)
            .field("flStyleDefault", &self.flStyleDefault)
            .field("flExtStyleDefault", &self.flExtStyleDefault)
            .field("flCtrlTypeMask", &self.flCtrlTypeMask)
            .field("cStyleFlags", &self.cStyleFlags)
            .field("aStyleFlags", &self.aStyleFlags)
            .field("szTextDefault", &self.szTextDefault)
            .field("lpfnStyle", &self.lpfnStyle.map(|f| f as usize))
            .field("lpfnSizeToText", &self.lpfnSizeToText.map(|f| f as usize))
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for CCINFOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for CCINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CCINFOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for CCINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for CCINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CCM_DPISCALE: u32 = 8204u32;
pub const CCM_FIRST: u32 = 8192u32;
pub const CCM_GETCOLORSCHEME: u32 = 8195u32;
pub const CCM_GETDROPTARGET: u32 = 8196u32;
pub const CCM_GETUNICODEFORMAT: u32 = 8198u32;
pub const CCM_GETVERSION: u32 = 8200u32;
pub const CCM_LAST: u32 = 8704u32;
pub const CCM_SETBKCOLOR: u32 = 8193u32;
pub const CCM_SETCOLORSCHEME: u32 = 8194u32;
pub const CCM_SETNOTIFYWINDOW: u32 = 8201u32;
pub const CCM_SETUNICODEFORMAT: u32 = 8197u32;
pub const CCM_SETVERSION: u32 = 8199u32;
pub const CCM_SETWINDOWTHEME: u32 = 8203u32;
#[repr(C)]
pub struct CCSTYLEA {
    pub flStyle: u32,
    pub flExtStyle: u32,
    pub szText: [::win32_foundation::CHAR; 256],
    pub lgid: u16,
    pub wReserved1: u16,
}
impl ::core::marker::Copy for CCSTYLEA {}
impl ::core::clone::Clone for CCSTYLEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CCSTYLEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCSTYLEA").field("flStyle", &self.flStyle).field("flExtStyle", &self.flExtStyle).field("szText", &self.szText).field("lgid", &self.lgid).field("wReserved1", &self.wReserved1).finish()
    }
}
unsafe impl ::windows_core::Abi for CCSTYLEA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CCSTYLEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CCSTYLEA>()) == 0 }
    }
}
impl ::core::cmp::Eq for CCSTYLEA {}
impl ::core::default::Default for CCSTYLEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CCSTYLEFLAGA {
    pub flStyle: u32,
    pub flStyleMask: u32,
    pub pszStyle: ::windows_core::PSTR,
}
impl ::core::marker::Copy for CCSTYLEFLAGA {}
impl ::core::clone::Clone for CCSTYLEFLAGA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CCSTYLEFLAGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCSTYLEFLAGA").field("flStyle", &self.flStyle).field("flStyleMask", &self.flStyleMask).field("pszStyle", &self.pszStyle).finish()
    }
}
unsafe impl ::windows_core::Abi for CCSTYLEFLAGA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CCSTYLEFLAGA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CCSTYLEFLAGA>()) == 0 }
    }
}
impl ::core::cmp::Eq for CCSTYLEFLAGA {}
impl ::core::default::Default for CCSTYLEFLAGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CCSTYLEFLAGW {
    pub flStyle: u32,
    pub flStyleMask: u32,
    pub pszStyle: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for CCSTYLEFLAGW {}
impl ::core::clone::Clone for CCSTYLEFLAGW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CCSTYLEFLAGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCSTYLEFLAGW").field("flStyle", &self.flStyle).field("flStyleMask", &self.flStyleMask).field("pszStyle", &self.pszStyle).finish()
    }
}
unsafe impl ::windows_core::Abi for CCSTYLEFLAGW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CCSTYLEFLAGW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CCSTYLEFLAGW>()) == 0 }
    }
}
impl ::core::cmp::Eq for CCSTYLEFLAGW {}
impl ::core::default::Default for CCSTYLEFLAGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CCSTYLEW {
    pub flStyle: u32,
    pub flExtStyle: u32,
    pub szText: [u16; 256],
    pub lgid: u16,
    pub wReserved1: u16,
}
impl ::core::marker::Copy for CCSTYLEW {}
impl ::core::clone::Clone for CCSTYLEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CCSTYLEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCSTYLEW").field("flStyle", &self.flStyle).field("flExtStyle", &self.flExtStyle).field("szText", &self.szText).field("lgid", &self.lgid).field("wReserved1", &self.wReserved1).finish()
    }
}
unsafe impl ::windows_core::Abi for CCSTYLEW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CCSTYLEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CCSTYLEW>()) == 0 }
    }
}
impl ::core::cmp::Eq for CCSTYLEW {}
impl ::core::default::Default for CCSTYLEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CCS_ADJUSTABLE: i32 = 32i32;
pub const CCS_BOTTOM: i32 = 3i32;
pub const CCS_NODIVIDER: i32 = 64i32;
pub const CCS_NOMOVEY: i32 = 2i32;
pub const CCS_NOPARENTALIGN: i32 = 8i32;
pub const CCS_NORESIZE: i32 = 4i32;
pub const CCS_TOP: i32 = 1i32;
pub const CCS_VERT: i32 = 128i32;
pub const CDDS_ITEM: u32 = 65536u32;
pub const CDDS_POSTERASE: u32 = 4u32;
pub const CDIS_CHECKED: u32 = 8u32;
pub const CDIS_DEFAULT: u32 = 32u32;
pub const CDIS_DISABLED: u32 = 4u32;
pub const CDIS_DROPHILITED: u32 = 4096u32;
pub const CDIS_FOCUS: u32 = 16u32;
pub const CDIS_GRAYED: u32 = 2u32;
pub const CDIS_HOT: u32 = 64u32;
pub const CDIS_INDETERMINATE: u32 = 256u32;
pub const CDIS_MARKED: u32 = 128u32;
pub const CDIS_NEARHOT: u32 = 1024u32;
pub const CDIS_OTHERSIDEHOT: u32 = 2048u32;
pub const CDIS_SELECTED: u32 = 1u32;
pub const CDIS_SHOWKEYBOARDCUES: u32 = 512u32;
pub const CDRF_DODEFAULT: u32 = 0u32;
pub const CDRF_DOERASE: u32 = 8u32;
pub const CDRF_NEWFONT: u32 = 2u32;
pub const CDRF_NOTIFYITEMDRAW: u32 = 32u32;
pub const CDRF_NOTIFYPOSTERASE: u32 = 64u32;
pub const CDRF_NOTIFYPOSTPAINT: u32 = 16u32;
pub const CDRF_NOTIFYSUBITEMDRAW: u32 = 32u32;
pub const CDRF_SKIPDEFAULT: u32 = 4u32;
pub const CDRF_SKIPPOSTPAINT: u32 = 256u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLOCKPARTS(pub i32);
pub const CLP_TIME: CLOCKPARTS = CLOCKPARTS(1i32);
impl ::core::marker::Copy for CLOCKPARTS {}
impl ::core::clone::Clone for CLOCKPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLOCKPARTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CLOCKPARTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CLOCKPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLOCKPARTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLOCKSTATES(pub i32);
pub const CLS_NORMAL: CLOCKSTATES = CLOCKSTATES(1i32);
pub const CLS_HOT: CLOCKSTATES = CLOCKSTATES(2i32);
pub const CLS_PRESSED: CLOCKSTATES = CLOCKSTATES(3i32);
impl ::core::marker::Copy for CLOCKSTATES {}
impl ::core::clone::Clone for CLOCKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLOCKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CLOCKSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for CLOCKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLOCKSTATES").field(&self.0).finish()
    }
}
pub const CLR_DEFAULT: i32 = -16777216i32;
pub const CLR_HILIGHT: i32 = -16777216i32;
pub const CLR_NONE: i32 = -1i32;
pub const CMB_MASKED: u32 = 2u32;
#[repr(C)]
pub struct COLORMAP {
    pub from: u32,
    pub to: u32,
}
impl ::core::marker::Copy for COLORMAP {}
impl ::core::clone::Clone for COLORMAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COLORMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORMAP").field("from", &self.from).field("to", &self.to).finish()
    }
}
unsafe impl ::windows_core::Abi for COLORMAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COLORMAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COLORMAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for COLORMAP {}
impl ::core::default::Default for COLORMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const COLORMGMTDLGORD: u32 = 1551u32;
#[repr(C)]
pub struct COLORSCHEME {
    pub dwSize: u32,
    pub clrBtnHighlight: u32,
    pub clrBtnShadow: u32,
}
impl ::core::marker::Copy for COLORSCHEME {}
impl ::core::clone::Clone for COLORSCHEME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COLORSCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORSCHEME").field("dwSize", &self.dwSize).field("clrBtnHighlight", &self.clrBtnHighlight).field("clrBtnShadow", &self.clrBtnShadow).finish()
    }
}
unsafe impl ::windows_core::Abi for COLORSCHEME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COLORSCHEME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COLORSCHEME>()) == 0 }
    }
}
impl ::core::cmp::Eq for COLORSCHEME {}
impl ::core::default::Default for COLORSCHEME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct COMBOBOXEXITEMA {
    pub mask: COMBOBOX_EX_ITEM_FLAGS,
    pub iItem: isize,
    pub pszText: ::windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for COMBOBOXEXITEMA {}
impl ::core::clone::Clone for COMBOBOXEXITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMBOBOXEXITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMBOBOXEXITEMA").field("mask", &self.mask).field("iItem", &self.iItem).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("iSelectedImage", &self.iSelectedImage).field("iOverlay", &self.iOverlay).field("iIndent", &self.iIndent).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for COMBOBOXEXITEMA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMBOBOXEXITEMA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMBOBOXEXITEMA>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMBOBOXEXITEMA {}
impl ::core::default::Default for COMBOBOXEXITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct COMBOBOXEXITEMW {
    pub mask: COMBOBOX_EX_ITEM_FLAGS,
    pub iItem: isize,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for COMBOBOXEXITEMW {}
impl ::core::clone::Clone for COMBOBOXEXITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMBOBOXEXITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMBOBOXEXITEMW").field("mask", &self.mask).field("iItem", &self.iItem).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("iSelectedImage", &self.iSelectedImage).field("iOverlay", &self.iOverlay).field("iIndent", &self.iIndent).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for COMBOBOXEXITEMW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMBOBOXEXITEMW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMBOBOXEXITEMW>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMBOBOXEXITEMW {}
impl ::core::default::Default for COMBOBOXEXITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct COMBOBOXINFO {
    pub cbSize: u32,
    pub rcItem: ::win32_foundation::RECT,
    pub rcButton: ::win32_foundation::RECT,
    pub stateButton: COMBOBOXINFO_BUTTON_STATE,
    pub hwndCombo: ::win32_foundation::HWND,
    pub hwndItem: ::win32_foundation::HWND,
    pub hwndList: ::win32_foundation::HWND,
}
impl ::core::marker::Copy for COMBOBOXINFO {}
impl ::core::clone::Clone for COMBOBOXINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMBOBOXINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMBOBOXINFO").field("cbSize", &self.cbSize).field("rcItem", &self.rcItem).field("rcButton", &self.rcButton).field("stateButton", &self.stateButton).field("hwndCombo", &self.hwndCombo).field("hwndItem", &self.hwndItem).field("hwndList", &self.hwndList).finish()
    }
}
unsafe impl ::windows_core::Abi for COMBOBOXINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMBOBOXINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMBOBOXINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMBOBOXINFO {}
impl ::core::default::Default for COMBOBOXINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMBOBOXINFO_BUTTON_STATE(pub u32);
pub const STATE_SYSTEM_INVISIBLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(32768u32);
pub const STATE_SYSTEM_PRESSED: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(8u32);
pub const STATE_SYSTEM_FOCUSABLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(1048576u32);
pub const STATE_SYSTEM_OFFSCREEN: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(65536u32);
pub const STATE_SYSTEM_UNAVAILABLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(1u32);
impl ::core::marker::Copy for COMBOBOXINFO_BUTTON_STATE {}
impl ::core::clone::Clone for COMBOBOXINFO_BUTTON_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMBOBOXINFO_BUTTON_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMBOBOXINFO_BUTTON_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMBOBOXINFO_BUTTON_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMBOBOXINFO_BUTTON_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMBOBOX_EX_ITEM_FLAGS(pub u32);
pub const CBEIF_DI_SETITEM: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(268435456u32);
pub const CBEIF_IMAGE: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(2u32);
pub const CBEIF_INDENT: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(16u32);
pub const CBEIF_LPARAM: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(32u32);
pub const CBEIF_OVERLAY: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(8u32);
pub const CBEIF_SELECTEDIMAGE: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(4u32);
pub const CBEIF_TEXT: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(1u32);
impl ::core::marker::Copy for COMBOBOX_EX_ITEM_FLAGS {}
impl ::core::clone::Clone for COMBOBOX_EX_ITEM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMBOBOX_EX_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for COMBOBOX_EX_ITEM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMBOBOX_EX_ITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMBOBOX_EX_ITEM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for COMBOBOX_EX_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for COMBOBOX_EX_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for COMBOBOX_EX_ITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for COMBOBOX_EX_ITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for COMBOBOX_EX_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const COMCTL32_VERSION: u32 = 6u32;
#[repr(C)]
pub struct COMPAREITEMSTRUCT {
    pub CtlType: u32,
    pub CtlID: u32,
    pub hwndItem: ::win32_foundation::HWND,
    pub itemID1: u32,
    pub itemData1: usize,
    pub itemID2: u32,
    pub itemData2: usize,
    pub dwLocaleId: u32,
}
impl ::core::marker::Copy for COMPAREITEMSTRUCT {}
impl ::core::clone::Clone for COMPAREITEMSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPAREITEMSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPAREITEMSTRUCT").field("CtlType", &self.CtlType).field("CtlID", &self.CtlID).field("hwndItem", &self.hwndItem).field("itemID1", &self.itemID1).field("itemData1", &self.itemData1).field("itemID2", &self.itemID2).field("itemData2", &self.itemData2).field("dwLocaleId", &self.dwLocaleId).finish()
    }
}
unsafe impl ::windows_core::Abi for COMPAREITEMSTRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPAREITEMSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPAREITEMSTRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPAREITEMSTRUCT {}
impl ::core::default::Default for COMPAREITEMSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CONTENTALIGNMENT(pub i32);
pub const CA_LEFT: CONTENTALIGNMENT = CONTENTALIGNMENT(0i32);
pub const CA_CENTER: CONTENTALIGNMENT = CONTENTALIGNMENT(1i32);
pub const CA_RIGHT: CONTENTALIGNMENT = CONTENTALIGNMENT(2i32);
impl ::core::marker::Copy for CONTENTALIGNMENT {}
impl ::core::clone::Clone for CONTENTALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONTENTALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CONTENTALIGNMENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONTENTALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONTENTALIGNMENT").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn CheckDlgButton<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hdlg: Param0, nidbutton: i32, ucheck: DLG_BUTTON_CHECK_STATE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckDlgButton(hdlg: ::win32_foundation::HWND, nidbutton: i32, ucheck: DLG_BUTTON_CHECK_STATE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CheckDlgButton(hdlg.into_param().abi(), ::core::mem::transmute(nidbutton), ::core::mem::transmute(ucheck)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CheckRadioButton<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hdlg: Param0, nidfirstbutton: i32, nidlastbutton: i32, nidcheckbutton: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckRadioButton(hdlg: ::win32_foundation::HWND, nidfirstbutton: i32, nidlastbutton: i32, nidcheckbutton: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(CheckRadioButton(hdlg.into_param().abi(), ::core::mem::transmute(nidfirstbutton), ::core::mem::transmute(nidlastbutton), ::core::mem::transmute(nidcheckbutton)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CloseThemeData(htheme: isize) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThemeData(htheme: isize) -> ::windows_core::HRESULT;
        }
        CloseThemeData(::core::mem::transmute(htheme)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateMappedBitmap<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(hinstance: Param0, idbitmap: isize, wflags: u32, lpcolormap: *const COLORMAP, inummaps: i32) -> ::windows_core::Result<::win32_graphics::Gdi::HBITMAP> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMappedBitmap(hinstance: ::win32_foundation::HINSTANCE, idbitmap: isize, wflags: u32, lpcolormap: *const COLORMAP, inummaps: i32) -> ::win32_graphics::Gdi::HBITMAP;
        }
        let result__ = CreateMappedBitmap(hinstance.into_param().abi(), ::core::mem::transmute(idbitmap), ::core::mem::transmute(wflags), ::core::mem::transmute(lpcolormap), ::core::mem::transmute(inummaps));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn CreatePropertySheetPageA(constpropsheetpagepointer: *mut PROPSHEETPAGEA) -> HPROPSHEETPAGE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePropertySheetPageA(constpropsheetpagepointer: *mut PROPSHEETPAGEA) -> HPROPSHEETPAGE;
        }
        ::core::mem::transmute(CreatePropertySheetPageA(::core::mem::transmute(constpropsheetpagepointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn CreatePropertySheetPageW(constpropsheetpagepointer: *mut PROPSHEETPAGEW) -> HPROPSHEETPAGE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePropertySheetPageW(constpropsheetpagepointer: *mut PROPSHEETPAGEW) -> HPROPSHEETPAGE;
        }
        ::core::mem::transmute(CreatePropertySheetPageW(::core::mem::transmute(constpropsheetpagepointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateStatusWindowA<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(style: i32, lpsztext: Param1, hwndparent: Param2, wid: u32) -> ::win32_foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStatusWindowA(style: i32, lpsztext: ::windows_core::PCSTR, hwndparent: ::win32_foundation::HWND, wid: u32) -> ::win32_foundation::HWND;
        }
        ::core::mem::transmute(CreateStatusWindowA(::core::mem::transmute(style), lpsztext.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(wid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateStatusWindowW<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(style: i32, lpsztext: Param1, hwndparent: Param2, wid: u32) -> ::win32_foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStatusWindowW(style: i32, lpsztext: ::windows_core::PCWSTR, hwndparent: ::win32_foundation::HWND, wid: u32) -> ::win32_foundation::HWND;
        }
        ::core::mem::transmute(CreateStatusWindowW(::core::mem::transmute(style), lpsztext.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(wid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn CreateSyntheticPointerDevice(pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE, maxcount: u32, mode: POINTER_FEEDBACK_MODE) -> ::windows_core::Result<HSYNTHETICPOINTERDEVICE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSyntheticPointerDevice(pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE, maxcount: u32, mode: POINTER_FEEDBACK_MODE) -> HSYNTHETICPOINTERDEVICE;
        }
        let result__ = CreateSyntheticPointerDevice(::core::mem::transmute(pointertype), ::core::mem::transmute(maxcount), ::core::mem::transmute(mode));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateToolbarEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(hwnd: Param0, ws: u32, wid: u32, nbitmaps: i32, hbminst: Param4, wbmid: usize, lpbuttons: *mut TBBUTTON, inumbuttons: i32, dxbutton: i32, dybutton: i32, dxbitmap: i32, dybitmap: i32, ustructsize: u32) -> ::win32_foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateToolbarEx(hwnd: ::win32_foundation::HWND, ws: u32, wid: u32, nbitmaps: i32, hbminst: ::win32_foundation::HINSTANCE, wbmid: usize, lpbuttons: *mut TBBUTTON, inumbuttons: i32, dxbutton: i32, dybutton: i32, dxbitmap: i32, dybitmap: i32, ustructsize: u32) -> ::win32_foundation::HWND;
        }
        ::core::mem::transmute(CreateToolbarEx(hwnd.into_param().abi(), ::core::mem::transmute(ws), ::core::mem::transmute(wid), ::core::mem::transmute(nbitmaps), hbminst.into_param().abi(), ::core::mem::transmute(wbmid), ::core::mem::transmute(lpbuttons), ::core::mem::transmute(inumbuttons), ::core::mem::transmute(dxbutton), ::core::mem::transmute(dybutton), ::core::mem::transmute(dxbitmap), ::core::mem::transmute(dybitmap), ::core::mem::transmute(ustructsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateUpDownControl<'a, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param8: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(dwstyle: u32, x: i32, y: i32, cx: i32, cy: i32, hparent: Param5, nid: i32, hinst: Param7, hbuddy: Param8, nupper: i32, nlower: i32, npos: i32) -> ::win32_foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUpDownControl(dwstyle: u32, x: i32, y: i32, cx: i32, cy: i32, hparent: ::win32_foundation::HWND, nid: i32, hinst: ::win32_foundation::HINSTANCE, hbuddy: ::win32_foundation::HWND, nupper: i32, nlower: i32, npos: i32) -> ::win32_foundation::HWND;
        }
        ::core::mem::transmute(CreateUpDownControl(::core::mem::transmute(dwstyle), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(cx), ::core::mem::transmute(cy), hparent.into_param().abi(), ::core::mem::transmute(nid), hinst.into_param().abi(), hbuddy.into_param().abi(), ::core::mem::transmute(nupper), ::core::mem::transmute(nlower), ::core::mem::transmute(npos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct DATETIMEPICKERINFO {
    pub cbSize: u32,
    pub rcCheck: ::win32_foundation::RECT,
    pub stateCheck: u32,
    pub rcButton: ::win32_foundation::RECT,
    pub stateButton: u32,
    pub hwndEdit: ::win32_foundation::HWND,
    pub hwndUD: ::win32_foundation::HWND,
    pub hwndDropDown: ::win32_foundation::HWND,
}
impl ::core::marker::Copy for DATETIMEPICKERINFO {}
impl ::core::clone::Clone for DATETIMEPICKERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DATETIMEPICKERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATETIMEPICKERINFO").field("cbSize", &self.cbSize).field("rcCheck", &self.rcCheck).field("stateCheck", &self.stateCheck).field("rcButton", &self.rcButton).field("stateButton", &self.stateButton).field("hwndEdit", &self.hwndEdit).field("hwndUD", &self.hwndUD).field("hwndDropDown", &self.hwndDropDown).finish()
    }
}
unsafe impl ::windows_core::Abi for DATETIMEPICKERINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DATETIMEPICKERINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DATETIMEPICKERINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DATETIMEPICKERINFO {}
impl ::core::default::Default for DATETIMEPICKERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DATETIMEPICK_CLASS: &str = "SysDateTimePick32";
pub const DATETIMEPICK_CLASSA: &str = "SysDateTimePick32";
pub const DATETIMEPICK_CLASSW: &str = "SysDateTimePick32";
pub const DA_ERR: i32 = -1i32;
pub const DA_LAST: u32 = 2147483647u32;
#[repr(C)]
pub struct DELETEITEMSTRUCT {
    pub CtlType: DRAWITEMSTRUCT_CTL_TYPE,
    pub CtlID: u32,
    pub itemID: u32,
    pub hwndItem: ::win32_foundation::HWND,
    pub itemData: usize,
}
impl ::core::marker::Copy for DELETEITEMSTRUCT {}
impl ::core::clone::Clone for DELETEITEMSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DELETEITEMSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DELETEITEMSTRUCT").field("CtlType", &self.CtlType).field("CtlID", &self.CtlID).field("itemID", &self.itemID).field("hwndItem", &self.hwndItem).field("itemData", &self.itemData).finish()
    }
}
unsafe impl ::windows_core::Abi for DELETEITEMSTRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DELETEITEMSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DELETEITEMSTRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DELETEITEMSTRUCT {}
impl ::core::default::Default for DELETEITEMSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DLG_BUTTON_CHECK_STATE(pub u32);
pub const BST_CHECKED: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(1u32);
pub const BST_INDETERMINATE: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(2u32);
pub const BST_UNCHECKED: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(0u32);
impl ::core::marker::Copy for DLG_BUTTON_CHECK_STATE {}
impl ::core::clone::Clone for DLG_BUTTON_CHECK_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DLG_BUTTON_CHECK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DLG_BUTTON_CHECK_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DLG_BUTTON_CHECK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DLG_BUTTON_CHECK_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DLG_DIR_LIST_FILE_TYPE(pub u32);
pub const DDL_ARCHIVE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(32u32);
pub const DDL_DIRECTORY: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(16u32);
pub const DDL_DRIVES: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(16384u32);
pub const DDL_EXCLUSIVE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(32768u32);
pub const DDL_HIDDEN: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(2u32);
pub const DDL_READONLY: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(1u32);
pub const DDL_READWRITE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(0u32);
pub const DDL_SYSTEM: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(4u32);
pub const DDL_POSTMSGS: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(8192u32);
impl ::core::marker::Copy for DLG_DIR_LIST_FILE_TYPE {}
impl ::core::clone::Clone for DLG_DIR_LIST_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DLG_DIR_LIST_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DLG_DIR_LIST_FILE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DLG_DIR_LIST_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DLG_DIR_LIST_FILE_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DLG_DIR_LIST_FILE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DLG_DIR_LIST_FILE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DLG_DIR_LIST_FILE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DLG_DIR_LIST_FILE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DLG_DIR_LIST_FILE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DL_COPYCURSOR: u32 = 2u32;
pub const DL_CURSORSET: u32 = 0u32;
pub const DL_MOVECURSOR: u32 = 3u32;
pub const DL_STOPCURSOR: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DPAMM_MESSAGE(pub u32);
pub const DPAMM_MERGE: DPAMM_MESSAGE = DPAMM_MESSAGE(1u32);
pub const DPAMM_DELETE: DPAMM_MESSAGE = DPAMM_MESSAGE(2u32);
pub const DPAMM_INSERT: DPAMM_MESSAGE = DPAMM_MESSAGE(3u32);
impl ::core::marker::Copy for DPAMM_MESSAGE {}
impl ::core::clone::Clone for DPAMM_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DPAMM_MESSAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DPAMM_MESSAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DPAMM_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DPAMM_MESSAGE").field(&self.0).finish()
    }
}
pub const DPAM_INTERSECT: u32 = 8u32;
pub const DPAM_NORMAL: u32 = 2u32;
pub const DPAM_SORTED: u32 = 1u32;
pub const DPAM_UNION: u32 = 4u32;
#[repr(C)]
pub struct DPASTREAMINFO {
    pub iPos: i32,
    pub pvItem: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DPASTREAMINFO {}
impl ::core::clone::Clone for DPASTREAMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DPASTREAMINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DPASTREAMINFO").field("iPos", &self.iPos).field("pvItem", &self.pvItem).finish()
    }
}
unsafe impl ::windows_core::Abi for DPASTREAMINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DPASTREAMINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DPASTREAMINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DPASTREAMINFO {}
impl ::core::default::Default for DPASTREAMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DPAS_INSERTAFTER: u32 = 4u32;
pub const DPAS_INSERTBEFORE: u32 = 2u32;
pub const DPAS_SORTED: u32 = 1u32;
pub const DPA_APPEND: u32 = 2147483647u32;
#[inline]
pub unsafe fn DPA_Clone<'a, Param0: ::windows_core::IntoParam<'a, HDPA>, Param1: ::windows_core::IntoParam<'a, HDPA>>(hdpa: Param0, hdpanew: Param1) -> HDPA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Clone(hdpa: HDPA, hdpanew: HDPA) -> HDPA;
        }
        ::core::mem::transmute(DPA_Clone(hdpa.into_param().abi(), hdpanew.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_Create(citemgrow: i32) -> HDPA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Create(citemgrow: i32) -> HDPA;
        }
        ::core::mem::transmute(DPA_Create(::core::mem::transmute(citemgrow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_CreateEx<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(cpgrow: i32, hheap: Param1) -> HDPA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_CreateEx(cpgrow: i32, hheap: ::win32_foundation::HANDLE) -> HDPA;
        }
        ::core::mem::transmute(DPA_CreateEx(::core::mem::transmute(cpgrow), hheap.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_DeleteAllPtrs<'a, Param0: ::windows_core::IntoParam<'a, HDPA>>(hdpa: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_DeleteAllPtrs(hdpa: HDPA) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DPA_DeleteAllPtrs(hdpa.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_DeletePtr<'a, Param0: ::windows_core::IntoParam<'a, HDPA>>(hdpa: Param0, i: i32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_DeletePtr(hdpa: HDPA, i: i32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(DPA_DeletePtr(hdpa.into_param().abi(), ::core::mem::transmute(i)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_Destroy<'a, Param0: ::windows_core::IntoParam<'a, HDPA>>(hdpa: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Destroy(hdpa: HDPA) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DPA_Destroy(hdpa.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_DestroyCallback<'a, Param0: ::windows_core::IntoParam<'a, HDPA>>(hdpa: Param0, pfncb: PFNDAENUMCALLBACK, pdata: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_DestroyCallback(hdpa: HDPA, pfncb: ::windows_core::RawPtr, pdata: *const ::core::ffi::c_void);
        }
        DPA_DestroyCallback(hdpa.into_param().abi(), ::core::mem::transmute(pfncb), ::core::mem::transmute(pdata))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DPA_ERR: i32 = -1i32;
#[inline]
pub unsafe fn DPA_EnumCallback<'a, Param0: ::windows_core::IntoParam<'a, HDPA>>(hdpa: Param0, pfncb: PFNDAENUMCALLBACK, pdata: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_EnumCallback(hdpa: HDPA, pfncb: ::windows_core::RawPtr, pdata: *const ::core::ffi::c_void);
        }
        DPA_EnumCallback(hdpa.into_param().abi(), ::core::mem::transmute(pfncb), ::core::mem::transmute(pdata))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_GetPtr<'a, Param0: ::windows_core::IntoParam<'a, HDPA>>(hdpa: Param0, i: isize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_GetPtr(hdpa: HDPA, i: isize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(DPA_GetPtr(hdpa.into_param().abi(), ::core::mem::transmute(i)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_GetPtrIndex<'a, Param0: ::windows_core::IntoParam<'a, HDPA>>(hdpa: Param0, p: *const ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_GetPtrIndex(hdpa: HDPA, p: *const ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(DPA_GetPtrIndex(hdpa.into_param().abi(), ::core::mem::transmute(p)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_GetSize<'a, Param0: ::windows_core::IntoParam<'a, HDPA>>(hdpa: Param0) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_GetSize(hdpa: HDPA) -> u64;
        }
        ::core::mem::transmute(DPA_GetSize(hdpa.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_Grow<'a, Param0: ::windows_core::IntoParam<'a, HDPA>>(pdpa: Param0, cp: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Grow(pdpa: HDPA, cp: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DPA_Grow(pdpa.into_param().abi(), ::core::mem::transmute(cp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_InsertPtr<'a, Param0: ::windows_core::IntoParam<'a, HDPA>>(hdpa: Param0, i: i32, p: *const ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_InsertPtr(hdpa: HDPA, i: i32, p: *const ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(DPA_InsertPtr(hdpa.into_param().abi(), ::core::mem::transmute(i), ::core::mem::transmute(p)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DPA_LoadStream<'a, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(phdpa: *mut HDPA, pfn: PFNDPASTREAM, pstream: Param2, pvinstdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_LoadStream(phdpa: *mut HDPA, pfn: ::windows_core::RawPtr, pstream: ::windows_core::RawPtr, pvinstdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        DPA_LoadStream(::core::mem::transmute(phdpa), ::core::mem::transmute(pfn), pstream.into_param().abi(), ::core::mem::transmute(pvinstdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_Merge<'a, Param0: ::windows_core::IntoParam<'a, HDPA>, Param1: ::windows_core::IntoParam<'a, HDPA>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::LPARAM>>(hdpadest: Param0, hdpasrc: Param1, dwflags: u32, pfncompare: PFNDACOMPARE, pfnmerge: PFNDPAMERGE, lparam: Param5) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Merge(hdpadest: HDPA, hdpasrc: HDPA, dwflags: u32, pfncompare: ::windows_core::RawPtr, pfnmerge: ::windows_core::RawPtr, lparam: ::win32_foundation::LPARAM) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DPA_Merge(hdpadest.into_param().abi(), hdpasrc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pfncompare), ::core::mem::transmute(pfnmerge), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DPA_SaveStream<'a, Param0: ::windows_core::IntoParam<'a, HDPA>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(hdpa: Param0, pfn: PFNDPASTREAM, pstream: Param2, pvinstdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_SaveStream(hdpa: HDPA, pfn: ::windows_core::RawPtr, pstream: ::windows_core::RawPtr, pvinstdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        DPA_SaveStream(hdpa.into_param().abi(), ::core::mem::transmute(pfn), pstream.into_param().abi(), ::core::mem::transmute(pvinstdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_Search<'a, Param0: ::windows_core::IntoParam<'a, HDPA>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::LPARAM>>(hdpa: Param0, pfind: *const ::core::ffi::c_void, istart: i32, pfncompare: PFNDACOMPARE, lparam: Param4, options: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Search(hdpa: HDPA, pfind: *const ::core::ffi::c_void, istart: i32, pfncompare: ::windows_core::RawPtr, lparam: ::win32_foundation::LPARAM, options: u32) -> i32;
        }
        ::core::mem::transmute(DPA_Search(hdpa.into_param().abi(), ::core::mem::transmute(pfind), ::core::mem::transmute(istart), ::core::mem::transmute(pfncompare), lparam.into_param().abi(), ::core::mem::transmute(options)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_SetPtr<'a, Param0: ::windows_core::IntoParam<'a, HDPA>>(hdpa: Param0, i: i32, p: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_SetPtr(hdpa: HDPA, i: i32, p: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DPA_SetPtr(hdpa.into_param().abi(), ::core::mem::transmute(i), ::core::mem::transmute(p)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DPA_Sort<'a, Param0: ::windows_core::IntoParam<'a, HDPA>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::LPARAM>>(hdpa: Param0, pfncompare: PFNDACOMPARE, lparam: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Sort(hdpa: HDPA, pfncompare: ::windows_core::RawPtr, lparam: ::win32_foundation::LPARAM) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DPA_Sort(hdpa.into_param().abi(), ::core::mem::transmute(pfncompare), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct DRAGLISTINFO {
    pub uNotification: DRAGLISTINFO_NOTIFICATION_FLAGS,
    pub hWnd: ::win32_foundation::HWND,
    pub ptCursor: ::win32_foundation::POINT,
}
impl ::core::marker::Copy for DRAGLISTINFO {}
impl ::core::clone::Clone for DRAGLISTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRAGLISTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAGLISTINFO").field("uNotification", &self.uNotification).field("hWnd", &self.hWnd).field("ptCursor", &self.ptCursor).finish()
    }
}
unsafe impl ::windows_core::Abi for DRAGLISTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRAGLISTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRAGLISTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRAGLISTINFO {}
impl ::core::default::Default for DRAGLISTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRAGLISTINFO_NOTIFICATION_FLAGS(pub u32);
pub const DL_BEGINDRAG: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1157u32);
pub const DL_CANCELDRAG: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1160u32);
pub const DL_DRAGGING: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1158u32);
pub const DL_DROPPED: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1159u32);
impl ::core::marker::Copy for DRAGLISTINFO_NOTIFICATION_FLAGS {}
impl ::core::clone::Clone for DRAGLISTINFO_NOTIFICATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAGLISTINFO_NOTIFICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DRAGLISTINFO_NOTIFICATION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRAGLISTINFO_NOTIFICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAGLISTINFO_NOTIFICATION_FLAGS").field(&self.0).finish()
    }
}
pub const DRAGLISTMSGSTRING: &str = "commctrl_DragListMsg";
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct DRAWITEMSTRUCT {
    pub CtlType: DRAWITEMSTRUCT_CTL_TYPE,
    pub CtlID: u32,
    pub itemID: u32,
    pub itemAction: u32,
    pub itemState: u32,
    pub hwndItem: ::win32_foundation::HWND,
    pub hDC: ::win32_graphics::Gdi::HDC,
    pub rcItem: ::win32_foundation::RECT,
    pub itemData: usize,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for DRAWITEMSTRUCT {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for DRAWITEMSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for DRAWITEMSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAWITEMSTRUCT").field("CtlType", &self.CtlType).field("CtlID", &self.CtlID).field("itemID", &self.itemID).field("itemAction", &self.itemAction).field("itemState", &self.itemState).field("hwndItem", &self.hwndItem).field("hDC", &self.hDC).field("rcItem", &self.rcItem).field("itemData", &self.itemData).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for DRAWITEMSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for DRAWITEMSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRAWITEMSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for DRAWITEMSTRUCT {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for DRAWITEMSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRAWITEMSTRUCT_CTL_TYPE(pub u32);
pub const ODT_BUTTON: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(4u32);
pub const ODT_COMBOBOX: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(3u32);
pub const ODT_LISTBOX: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(2u32);
pub const ODT_LISTVIEW: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(102u32);
pub const ODT_MENU: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(1u32);
pub const ODT_STATIC: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(5u32);
pub const ODT_TAB: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(101u32);
impl ::core::marker::Copy for DRAWITEMSTRUCT_CTL_TYPE {}
impl ::core::clone::Clone for DRAWITEMSTRUCT_CTL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAWITEMSTRUCT_CTL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DRAWITEMSTRUCT_CTL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRAWITEMSTRUCT_CTL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAWITEMSTRUCT_CTL_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRAW_THEME_PARENT_BACKGROUND_FLAGS(pub u32);
pub const DTPB_WINDOWDC: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(1u32);
pub const DTPB_USECTLCOLORSTATIC: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(2u32);
pub const DTPB_USEERASEBKGND: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(4u32);
impl ::core::marker::Copy for DRAW_THEME_PARENT_BACKGROUND_FLAGS {}
impl ::core::clone::Clone for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAW_THEME_PARENT_BACKGROUND_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DSA_APPEND: u32 = 2147483647u32;
#[inline]
pub unsafe fn DSA_Clone<'a, Param0: ::windows_core::IntoParam<'a, HDSA>>(hdsa: Param0) -> HDSA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_Clone(hdsa: HDSA) -> HDSA;
        }
        ::core::mem::transmute(DSA_Clone(hdsa.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DSA_Create(cbitem: i32, citemgrow: i32) -> HDSA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_Create(cbitem: i32, citemgrow: i32) -> HDSA;
        }
        ::core::mem::transmute(DSA_Create(::core::mem::transmute(cbitem), ::core::mem::transmute(citemgrow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DSA_DeleteAllItems<'a, Param0: ::windows_core::IntoParam<'a, HDSA>>(hdsa: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_DeleteAllItems(hdsa: HDSA) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DSA_DeleteAllItems(hdsa.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DSA_DeleteItem<'a, Param0: ::windows_core::IntoParam<'a, HDSA>>(hdsa: Param0, i: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_DeleteItem(hdsa: HDSA, i: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DSA_DeleteItem(hdsa.into_param().abi(), ::core::mem::transmute(i)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DSA_Destroy<'a, Param0: ::windows_core::IntoParam<'a, HDSA>>(hdsa: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_Destroy(hdsa: HDSA) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DSA_Destroy(hdsa.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DSA_DestroyCallback<'a, Param0: ::windows_core::IntoParam<'a, HDSA>>(hdsa: Param0, pfncb: PFNDAENUMCALLBACK, pdata: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_DestroyCallback(hdsa: HDSA, pfncb: ::windows_core::RawPtr, pdata: *const ::core::ffi::c_void);
        }
        DSA_DestroyCallback(hdsa.into_param().abi(), ::core::mem::transmute(pfncb), ::core::mem::transmute(pdata))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DSA_ERR: i32 = -1i32;
#[inline]
pub unsafe fn DSA_EnumCallback<'a, Param0: ::windows_core::IntoParam<'a, HDSA>>(hdsa: Param0, pfncb: PFNDAENUMCALLBACK, pdata: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_EnumCallback(hdsa: HDSA, pfncb: ::windows_core::RawPtr, pdata: *const ::core::ffi::c_void);
        }
        DSA_EnumCallback(hdsa.into_param().abi(), ::core::mem::transmute(pfncb), ::core::mem::transmute(pdata))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DSA_GetItem<'a, Param0: ::windows_core::IntoParam<'a, HDSA>>(hdsa: Param0, i: i32, pitem: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_GetItem(hdsa: HDSA, i: i32, pitem: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DSA_GetItem(hdsa.into_param().abi(), ::core::mem::transmute(i), ::core::mem::transmute(pitem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DSA_GetItemPtr<'a, Param0: ::windows_core::IntoParam<'a, HDSA>>(hdsa: Param0, i: i32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_GetItemPtr(hdsa: HDSA, i: i32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(DSA_GetItemPtr(hdsa.into_param().abi(), ::core::mem::transmute(i)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DSA_GetSize<'a, Param0: ::windows_core::IntoParam<'a, HDSA>>(hdsa: Param0) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_GetSize(hdsa: HDSA) -> u64;
        }
        ::core::mem::transmute(DSA_GetSize(hdsa.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DSA_InsertItem<'a, Param0: ::windows_core::IntoParam<'a, HDSA>>(hdsa: Param0, i: i32, pitem: *const ::core::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_InsertItem(hdsa: HDSA, i: i32, pitem: *const ::core::ffi::c_void) -> i32;
        }
        ::core::mem::transmute(DSA_InsertItem(hdsa.into_param().abi(), ::core::mem::transmute(i), ::core::mem::transmute(pitem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DSA_SetItem<'a, Param0: ::windows_core::IntoParam<'a, HDSA>>(hdsa: Param0, i: i32, pitem: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_SetItem(hdsa: HDSA, i: i32, pitem: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DSA_SetItem(hdsa.into_param().abi(), ::core::mem::transmute(i), ::core::mem::transmute(pitem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DSA_Sort<'a, Param0: ::windows_core::IntoParam<'a, HDSA>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::LPARAM>>(pdsa: Param0, pfncompare: PFNDACOMPARE, lparam: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_Sort(pdsa: HDSA, pfncompare: ::windows_core::RawPtr, lparam: ::win32_foundation::LPARAM) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DSA_Sort(pdsa.into_param().abi(), ::core::mem::transmute(pfncompare), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct DTBGOPTS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub rcClip: ::win32_foundation::RECT,
}
impl ::core::marker::Copy for DTBGOPTS {}
impl ::core::clone::Clone for DTBGOPTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DTBGOPTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTBGOPTS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("rcClip", &self.rcClip).finish()
    }
}
unsafe impl ::windows_core::Abi for DTBGOPTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DTBGOPTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTBGOPTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DTBGOPTS {}
impl ::core::default::Default for DTBGOPTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DTBG_CLIPRECT: u32 = 1u32;
pub const DTBG_COMPUTINGREGION: u32 = 16u32;
pub const DTBG_DRAWSOLID: u32 = 2u32;
pub const DTBG_MIRRORDC: u32 = 32u32;
pub const DTBG_NOMIRROR: u32 = 64u32;
pub const DTBG_OMITBORDER: u32 = 4u32;
pub const DTBG_OMITCONTENT: u32 = 8u32;
pub const DTM_CLOSEMONTHCAL: u32 = 4109u32;
pub const DTM_FIRST: u32 = 4096u32;
pub const DTM_GETDATETIMEPICKERINFO: u32 = 4110u32;
pub const DTM_GETIDEALSIZE: u32 = 4111u32;
pub const DTM_GETMCCOLOR: u32 = 4103u32;
pub const DTM_GETMCFONT: u32 = 4106u32;
pub const DTM_GETMCSTYLE: u32 = 4108u32;
pub const DTM_GETMONTHCAL: u32 = 4104u32;
pub const DTM_GETRANGE: u32 = 4099u32;
pub const DTM_GETSYSTEMTIME: u32 = 4097u32;
pub const DTM_SETFORMAT: u32 = 4146u32;
pub const DTM_SETFORMATA: u32 = 4101u32;
pub const DTM_SETFORMATW: u32 = 4146u32;
pub const DTM_SETMCCOLOR: u32 = 4102u32;
pub const DTM_SETMCFONT: u32 = 4105u32;
pub const DTM_SETMCSTYLE: u32 = 4107u32;
pub const DTM_SETRANGE: u32 = 4100u32;
pub const DTM_SETSYSTEMTIME: u32 = 4098u32;
pub const DTS_APPCANPARSE: u32 = 16u32;
pub const DTS_LONGDATEFORMAT: u32 = 4u32;
pub const DTS_RIGHTALIGN: u32 = 32u32;
pub const DTS_SHORTDATECENTURYFORMAT: u32 = 12u32;
pub const DTS_SHORTDATEFORMAT: u32 = 0u32;
pub const DTS_SHOWNONE: u32 = 2u32;
pub const DTS_TIMEFORMAT: u32 = 9u32;
pub const DTS_UPDOWN: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct DTTOPTS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub crText: u32,
    pub crBorder: u32,
    pub crShadow: u32,
    pub iTextShadowType: i32,
    pub ptShadowOffset: ::win32_foundation::POINT,
    pub iBorderSize: i32,
    pub iFontPropId: i32,
    pub iColorPropId: i32,
    pub iStateId: i32,
    pub fApplyOverlay: ::win32_foundation::BOOL,
    pub iGlowSize: i32,
    pub pfnDrawTextCallback: DTT_CALLBACK_PROC,
    pub lParam: ::win32_foundation::LPARAM,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for DTTOPTS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for DTTOPTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for DTTOPTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DTTOPTS")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("crText", &self.crText)
            .field("crBorder", &self.crBorder)
            .field("crShadow", &self.crShadow)
            .field("iTextShadowType", &self.iTextShadowType)
            .field("ptShadowOffset", &self.ptShadowOffset)
            .field("iBorderSize", &self.iBorderSize)
            .field("iFontPropId", &self.iFontPropId)
            .field("iColorPropId", &self.iColorPropId)
            .field("iStateId", &self.iStateId)
            .field("fApplyOverlay", &self.fApplyOverlay)
            .field("iGlowSize", &self.iGlowSize)
            .field("pfnDrawTextCallback", &self.pfnDrawTextCallback.map(|f| f as usize))
            .field("lParam", &self.lParam)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for DTTOPTS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for DTTOPTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DTTOPTS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for DTTOPTS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for DTTOPTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type DTT_CALLBACK_PROC = ::core::option::Option<unsafe extern "system" fn(hdc: ::win32_graphics::Gdi::HDC, psztext: ::windows_core::PWSTR, cchtext: i32, prc: *mut ::win32_foundation::RECT, dwflags: u32, lparam: ::win32_foundation::LPARAM) -> i32>;
pub const DTT_FLAGS2VALIDBITS: u32 = 1u32;
pub const DTT_GRAYED: u32 = 1u32;
#[inline]
pub unsafe fn DestroyPropertySheetPage<'a, Param0: ::windows_core::IntoParam<'a, HPROPSHEETPAGE>>(param0: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyPropertySheetPage(param0: HPROPSHEETPAGE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DestroyPropertySheetPage(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DestroySyntheticPointerDevice<'a, Param0: ::windows_core::IntoParam<'a, HSYNTHETICPOINTERDEVICE>>(device: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroySyntheticPointerDevice(device: HSYNTHETICPOINTERDEVICE);
        }
        DestroySyntheticPointerDevice(device.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DlgDirListA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hdlg: Param0, lppathspec: ::windows_core::PSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirListA(hdlg: ::win32_foundation::HWND, lppathspec: ::windows_core::PSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
        }
        ::core::mem::transmute(DlgDirListA(hdlg.into_param().abi(), ::core::mem::transmute(lppathspec), ::core::mem::transmute(nidlistbox), ::core::mem::transmute(nidstaticpath), ::core::mem::transmute(ufiletype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DlgDirListComboBoxA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hdlg: Param0, lppathspec: ::windows_core::PSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirListComboBoxA(hdlg: ::win32_foundation::HWND, lppathspec: ::windows_core::PSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
        }
        ::core::mem::transmute(DlgDirListComboBoxA(hdlg.into_param().abi(), ::core::mem::transmute(lppathspec), ::core::mem::transmute(nidcombobox), ::core::mem::transmute(nidstaticpath), ::core::mem::transmute(ufiletype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DlgDirListComboBoxW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hdlg: Param0, lppathspec: ::windows_core::PWSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirListComboBoxW(hdlg: ::win32_foundation::HWND, lppathspec: ::windows_core::PWSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
        }
        ::core::mem::transmute(DlgDirListComboBoxW(hdlg.into_param().abi(), ::core::mem::transmute(lppathspec), ::core::mem::transmute(nidcombobox), ::core::mem::transmute(nidstaticpath), ::core::mem::transmute(ufiletype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DlgDirListW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hdlg: Param0, lppathspec: ::windows_core::PWSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirListW(hdlg: ::win32_foundation::HWND, lppathspec: ::windows_core::PWSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
        }
        ::core::mem::transmute(DlgDirListW(hdlg.into_param().abi(), ::core::mem::transmute(lppathspec), ::core::mem::transmute(nidlistbox), ::core::mem::transmute(nidstaticpath), ::core::mem::transmute(ufiletype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DlgDirSelectComboBoxExA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnddlg: Param0, lpstring: &mut [u8], idcombobox: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirSelectComboBoxExA(hwnddlg: ::win32_foundation::HWND, lpstring: ::windows_core::PSTR, cchout: i32, idcombobox: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DlgDirSelectComboBoxExA(hwnddlg.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpstring)), lpstring.len() as _, ::core::mem::transmute(idcombobox)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DlgDirSelectComboBoxExW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnddlg: Param0, lpstring: &mut [u16], idcombobox: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirSelectComboBoxExW(hwnddlg: ::win32_foundation::HWND, lpstring: ::windows_core::PWSTR, cchout: i32, idcombobox: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DlgDirSelectComboBoxExW(hwnddlg.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpstring)), lpstring.len() as _, ::core::mem::transmute(idcombobox)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DlgDirSelectExA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnddlg: Param0, lpstring: &mut [u8], idlistbox: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirSelectExA(hwnddlg: ::win32_foundation::HWND, lpstring: ::windows_core::PSTR, chcount: i32, idlistbox: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DlgDirSelectExA(hwnddlg.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpstring)), lpstring.len() as _, ::core::mem::transmute(idlistbox)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DlgDirSelectExW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnddlg: Param0, lpstring: &mut [u16], idlistbox: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirSelectExW(hwnddlg: ::win32_foundation::HWND, lpstring: ::windows_core::PWSTR, chcount: i32, idlistbox: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(DlgDirSelectExW(hwnddlg.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(lpstring)), lpstring.len() as _, ::core::mem::transmute(idlistbox)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DrawInsert<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(handparent: Param0, hlb: Param1, nitem: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawInsert(handparent: ::win32_foundation::HWND, hlb: ::win32_foundation::HWND, nitem: i32);
        }
        DrawInsert(handparent.into_param().abi(), hlb.into_param().abi(), ::core::mem::transmute(nitem))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawShadowText<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hdc: Param0, psztext: &[u16], prc: *const ::win32_foundation::RECT, dwflags: u32, crtext: u32, crshadow: u32, ixoffset: i32, iyoffset: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawShadowText(hdc: ::win32_graphics::Gdi::HDC, psztext: ::windows_core::PCWSTR, cch: u32, prc: *const ::win32_foundation::RECT, dwflags: u32, crtext: u32, crshadow: u32, ixoffset: i32, iyoffset: i32) -> i32;
        }
        ::core::mem::transmute(DrawShadowText(hdc.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(psztext)), psztext.len() as _, ::core::mem::transmute(prc), ::core::mem::transmute(dwflags), ::core::mem::transmute(crtext), ::core::mem::transmute(crshadow), ::core::mem::transmute(ixoffset), ::core::mem::transmute(iyoffset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawStatusTextA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hdc: Param0, lprc: *mut ::win32_foundation::RECT, psztext: Param2, uflags: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawStatusTextA(hdc: ::win32_graphics::Gdi::HDC, lprc: *mut ::win32_foundation::RECT, psztext: ::windows_core::PCSTR, uflags: u32);
        }
        DrawStatusTextA(hdc.into_param().abi(), ::core::mem::transmute(lprc), psztext.into_param().abi(), ::core::mem::transmute(uflags))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawStatusTextW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hdc: Param0, lprc: *mut ::win32_foundation::RECT, psztext: Param2, uflags: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawStatusTextW(hdc: ::win32_graphics::Gdi::HDC, lprc: *mut ::win32_foundation::RECT, psztext: ::windows_core::PCWSTR, uflags: u32);
        }
        DrawStatusTextW(hdc.into_param().abi(), ::core::mem::transmute(lprc), psztext.into_param().abi(), ::core::mem::transmute(uflags))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeBackground<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, prect: *const ::win32_foundation::RECT, pcliprect: *const ::win32_foundation::RECT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeBackground(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const ::win32_foundation::RECT, pcliprect: *const ::win32_foundation::RECT) -> ::windows_core::HRESULT;
        }
        DrawThemeBackground(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(prect), ::core::mem::transmute(pcliprect)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeBackgroundEx<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, prect: *const ::win32_foundation::RECT, poptions: *const DTBGOPTS) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeBackgroundEx(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const ::win32_foundation::RECT, poptions: *const DTBGOPTS) -> ::windows_core::HRESULT;
        }
        DrawThemeBackgroundEx(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(prect), ::core::mem::transmute(poptions)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeEdge<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, pdestrect: *const ::win32_foundation::RECT, uedge: u32, uflags: u32) -> ::windows_core::Result<::win32_foundation::RECT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeEdge(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, pdestrect: *const ::win32_foundation::RECT, uedge: u32, uflags: u32, pcontentrect: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        DrawThemeEdge(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(pdestrect), ::core::mem::transmute(uedge), ::core::mem::transmute(uflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeIcon<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>, Param5: ::windows_core::IntoParam<'a, HIMAGELIST>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, prect: *const ::win32_foundation::RECT, himl: Param5, iimageindex: i32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeIcon(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const ::win32_foundation::RECT, himl: HIMAGELIST, iimageindex: i32) -> ::windows_core::HRESULT;
        }
        DrawThemeIcon(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(prect), himl.into_param().abi(), ::core::mem::transmute(iimageindex)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeParentBackground<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hwnd: Param0, hdc: Param1, prc: *const ::win32_foundation::RECT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeParentBackground(hwnd: ::win32_foundation::HWND, hdc: ::win32_graphics::Gdi::HDC, prc: *const ::win32_foundation::RECT) -> ::windows_core::HRESULT;
        }
        DrawThemeParentBackground(hwnd.into_param().abi(), hdc.into_param().abi(), ::core::mem::transmute(prc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeParentBackgroundEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(hwnd: Param0, hdc: Param1, dwflags: DRAW_THEME_PARENT_BACKGROUND_FLAGS, prc: *const ::win32_foundation::RECT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeParentBackgroundEx(hwnd: ::win32_foundation::HWND, hdc: ::win32_graphics::Gdi::HDC, dwflags: DRAW_THEME_PARENT_BACKGROUND_FLAGS, prc: *const ::win32_foundation::RECT) -> ::windows_core::HRESULT;
        }
        DrawThemeParentBackgroundEx(hwnd.into_param().abi(), hdc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(prc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeText<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, psztext: &[u16], dwtextflags: u32, dwtextflags2: u32, prect: *const ::win32_foundation::RECT) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeText(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: ::windows_core::PCWSTR, cchtext: i32, dwtextflags: u32, dwtextflags2: u32, prect: *const ::win32_foundation::RECT) -> ::windows_core::HRESULT;
        }
        DrawThemeText(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(::windows_core::as_ptr_or_null(psztext)), psztext.len() as _, ::core::mem::transmute(dwtextflags), ::core::mem::transmute(dwtextflags2), ::core::mem::transmute(prect)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawThemeTextEx<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, psztext: &[u16], dwtextflags: u32, prect: *mut ::win32_foundation::RECT, poptions: *const DTTOPTS) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeTextEx(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: ::windows_core::PCWSTR, cchtext: i32, dwtextflags: u32, prect: *mut ::win32_foundation::RECT, poptions: *const DTTOPTS) -> ::windows_core::HRESULT;
        }
        DrawThemeTextEx(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(::windows_core::as_ptr_or_null(psztext)), psztext.len() as _, ::core::mem::transmute(dwtextflags), ::core::mem::transmute(prect), ::core::mem::transmute(poptions)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ECM_FIRST: u32 = 5376u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EC_ENDOFLINE(pub i32);
pub const EC_ENDOFLINE_DETECTFROMCONTENT: EC_ENDOFLINE = EC_ENDOFLINE(0i32);
pub const EC_ENDOFLINE_CRLF: EC_ENDOFLINE = EC_ENDOFLINE(1i32);
pub const EC_ENDOFLINE_CR: EC_ENDOFLINE = EC_ENDOFLINE(2i32);
pub const EC_ENDOFLINE_LF: EC_ENDOFLINE = EC_ENDOFLINE(3i32);
impl ::core::marker::Copy for EC_ENDOFLINE {}
impl ::core::clone::Clone for EC_ENDOFLINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EC_ENDOFLINE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EC_ENDOFLINE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EC_ENDOFLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_ENDOFLINE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EC_SEARCHWEB_ENTRYPOINT(pub i32);
pub const EC_SEARCHWEB_ENTRYPOINT_EXTERNAL: EC_SEARCHWEB_ENTRYPOINT = EC_SEARCHWEB_ENTRYPOINT(0i32);
pub const EC_SEARCHWEB_ENTRYPOINT_CONTEXTMENU: EC_SEARCHWEB_ENTRYPOINT = EC_SEARCHWEB_ENTRYPOINT(1i32);
impl ::core::marker::Copy for EC_SEARCHWEB_ENTRYPOINT {}
impl ::core::clone::Clone for EC_SEARCHWEB_ENTRYPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EC_SEARCHWEB_ENTRYPOINT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EC_SEARCHWEB_ENTRYPOINT {
    type Abi = Self;
}
impl ::core::fmt::Debug for EC_SEARCHWEB_ENTRYPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EC_SEARCHWEB_ENTRYPOINT").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct EDITBALLOONTIP {
    pub cbStruct: u32,
    pub pszTitle: ::windows_core::PCWSTR,
    pub pszText: ::windows_core::PCWSTR,
    pub ttiIcon: EDITBALLOONTIP_ICON,
}
impl ::core::marker::Copy for EDITBALLOONTIP {}
impl ::core::clone::Clone for EDITBALLOONTIP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EDITBALLOONTIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EDITBALLOONTIP").field("cbStruct", &self.cbStruct).field("pszTitle", &self.pszTitle).field("pszText", &self.pszText).field("ttiIcon", &self.ttiIcon).finish()
    }
}
unsafe impl ::windows_core::Abi for EDITBALLOONTIP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EDITBALLOONTIP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EDITBALLOONTIP>()) == 0 }
    }
}
impl ::core::cmp::Eq for EDITBALLOONTIP {}
impl ::core::default::Default for EDITBALLOONTIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EDITBALLOONTIP_ICON(pub u32);
pub const TTI_ERROR: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(3u32);
pub const TTI_INFO: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(1u32);
pub const TTI_NONE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(0u32);
pub const TTI_WARNING: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(2u32);
pub const TTI_INFO_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(4u32);
pub const TTI_WARNING_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(5u32);
pub const TTI_ERROR_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(6u32);
impl ::core::marker::Copy for EDITBALLOONTIP_ICON {}
impl ::core::clone::Clone for EDITBALLOONTIP_ICON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EDITBALLOONTIP_ICON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EDITBALLOONTIP_ICON {
    type Abi = Self;
}
impl ::core::fmt::Debug for EDITBALLOONTIP_ICON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDITBALLOONTIP_ICON").field(&self.0).finish()
    }
}
pub type EDITWORDBREAKPROCA = ::core::option::Option<unsafe extern "system" fn(lpch: ::windows_core::PCSTR, ichcurrent: i32, cch: i32, code: WORD_BREAK_ACTION) -> i32>;
pub type EDITWORDBREAKPROCW = ::core::option::Option<unsafe extern "system" fn(lpch: ::windows_core::PCWSTR, ichcurrent: i32, cch: i32, code: WORD_BREAK_ACTION) -> i32>;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EMPTYMARKUPPARTS(pub i32);
pub const EMP_MARKUPTEXT: EMPTYMARKUPPARTS = EMPTYMARKUPPARTS(1i32);
impl ::core::marker::Copy for EMPTYMARKUPPARTS {}
impl ::core::clone::Clone for EMPTYMARKUPPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EMPTYMARKUPPARTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EMPTYMARKUPPARTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EMPTYMARKUPPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMPTYMARKUPPARTS").field(&self.0).finish()
    }
}
pub const EM_CANUNDO: u32 = 198u32;
pub const EM_CHARFROMPOS: u32 = 215u32;
pub const EM_EMPTYUNDOBUFFER: u32 = 205u32;
pub const EM_ENABLEFEATURE: u32 = 218u32;
pub const EM_ENABLESEARCHWEB: u32 = 5390u32;
pub const EM_FILELINEFROMCHAR: u32 = 5395u32;
pub const EM_FILELINEINDEX: u32 = 5396u32;
pub const EM_FILELINELENGTH: u32 = 5397u32;
pub const EM_FMTLINES: u32 = 200u32;
pub const EM_GETCARETINDEX: u32 = 5394u32;
pub const EM_GETCUEBANNER: u32 = 5378u32;
pub const EM_GETENDOFLINE: u32 = 5389u32;
pub const EM_GETEXTENDEDSTYLE: u32 = 5387u32;
pub const EM_GETFILELINE: u32 = 5398u32;
pub const EM_GETFILELINECOUNT: u32 = 5399u32;
pub const EM_GETFIRSTVISIBLELINE: u32 = 206u32;
pub const EM_GETHANDLE: u32 = 189u32;
pub const EM_GETHILITE: u32 = 5382u32;
pub const EM_GETIMESTATUS: u32 = 217u32;
pub const EM_GETLIMITTEXT: u32 = 213u32;
pub const EM_GETLINE: u32 = 196u32;
pub const EM_GETLINECOUNT: u32 = 186u32;
pub const EM_GETMARGINS: u32 = 212u32;
pub const EM_GETMODIFY: u32 = 184u32;
pub const EM_GETPASSWORDCHAR: u32 = 210u32;
pub const EM_GETRECT: u32 = 178u32;
pub const EM_GETSEL: u32 = 176u32;
pub const EM_GETTHUMB: u32 = 190u32;
pub const EM_GETWORDBREAKPROC: u32 = 209u32;
pub const EM_HIDEBALLOONTIP: u32 = 5380u32;
pub const EM_LIMITTEXT: u32 = 197u32;
pub const EM_LINEFROMCHAR: u32 = 201u32;
pub const EM_LINEINDEX: u32 = 187u32;
pub const EM_LINELENGTH: u32 = 193u32;
pub const EM_LINESCROLL: u32 = 182u32;
pub const EM_NOSETFOCUS: u32 = 5383u32;
pub const EM_POSFROMCHAR: u32 = 214u32;
pub const EM_REPLACESEL: u32 = 194u32;
pub const EM_SCROLL: u32 = 181u32;
pub const EM_SCROLLCARET: u32 = 183u32;
pub const EM_SEARCHWEB: u32 = 5391u32;
pub const EM_SETCARETINDEX: u32 = 5393u32;
pub const EM_SETCUEBANNER: u32 = 5377u32;
pub const EM_SETENDOFLINE: u32 = 5388u32;
pub const EM_SETEXTENDEDSTYLE: u32 = 5386u32;
pub const EM_SETHANDLE: u32 = 188u32;
pub const EM_SETHILITE: u32 = 5381u32;
pub const EM_SETIMESTATUS: u32 = 216u32;
pub const EM_SETLIMITTEXT: u32 = 197u32;
pub const EM_SETMARGINS: u32 = 211u32;
pub const EM_SETMODIFY: u32 = 185u32;
pub const EM_SETPASSWORDCHAR: u32 = 204u32;
pub const EM_SETREADONLY: u32 = 207u32;
pub const EM_SETRECT: u32 = 179u32;
pub const EM_SETRECTNP: u32 = 180u32;
pub const EM_SETSEL: u32 = 177u32;
pub const EM_SETTABSTOPS: u32 = 203u32;
pub const EM_SETWORDBREAKPROC: u32 = 208u32;
pub const EM_SHOWBALLOONTIP: u32 = 5379u32;
pub const EM_TAKEFOCUS: u32 = 5384u32;
pub const EM_UNDO: u32 = 199u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ENABLE_SCROLL_BAR_ARROWS(pub u32);
pub const ESB_DISABLE_BOTH: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(3u32);
pub const ESB_DISABLE_DOWN: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
pub const ESB_DISABLE_LEFT: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
pub const ESB_DISABLE_LTUP: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
pub const ESB_DISABLE_RIGHT: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
pub const ESB_DISABLE_RTDN: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
pub const ESB_DISABLE_UP: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
pub const ESB_ENABLE_BOTH: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(0u32);
impl ::core::marker::Copy for ENABLE_SCROLL_BAR_ARROWS {}
impl ::core::clone::Clone for ENABLE_SCROLL_BAR_ARROWS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENABLE_SCROLL_BAR_ARROWS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ENABLE_SCROLL_BAR_ARROWS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ENABLE_SCROLL_BAR_ARROWS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENABLE_SCROLL_BAR_ARROWS").field(&self.0).finish()
    }
}
pub const ES_EX_ALLOWEOL_CR: i32 = 1i32;
pub const ES_EX_ALLOWEOL_LF: i32 = 2i32;
pub const ES_EX_CONVERT_EOL_ON_PASTE: i32 = 4i32;
pub const ES_EX_ZOOMABLE: i32 = 16i32;
pub const ETDT_DISABLE: u32 = 1u32;
pub const ETDT_ENABLE: u32 = 2u32;
pub const ETDT_USEAEROWIZARDTABTEXTURE: u32 = 8u32;
pub const ETDT_USETABTEXTURE: u32 = 4u32;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn EnableScrollBar<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, wsbflags: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, warrows: ENABLE_SCROLL_BAR_ARROWS) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableScrollBar(hwnd: ::win32_foundation::HWND, wsbflags: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, warrows: ENABLE_SCROLL_BAR_ARROWS) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EnableScrollBar(hwnd.into_param().abi(), ::core::mem::transmute(wsbflags), ::core::mem::transmute(warrows)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnableThemeDialogTexture<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, dwflags: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableThemeDialogTexture(hwnd: ::win32_foundation::HWND, dwflags: u32) -> ::windows_core::HRESULT;
        }
        EnableThemeDialogTexture(hwnd.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnableTheming<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(fenable: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableTheming(fenable: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        EnableTheming(fenable.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EndBufferedAnimation<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hbpanimation: isize, fupdatetarget: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndBufferedAnimation(hbpanimation: isize, fupdatetarget: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        EndBufferedAnimation(::core::mem::transmute(hbpanimation), fupdatetarget.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EndBufferedPaint<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hbufferedpaint: isize, fupdatetarget: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndBufferedPaint(hbufferedpaint: isize, fupdatetarget: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        EndBufferedPaint(::core::mem::transmute(hbufferedpaint), fupdatetarget.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EndPanningFeedback<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hwnd: Param0, fanimateback: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndPanningFeedback(hwnd: ::win32_foundation::HWND, fanimateback: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EndPanningFeedback(hwnd.into_param().abi(), fanimateback.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EvaluateProximityToPolygon(controlpolygon: &[::win32_foundation::POINT], phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvaluateProximityToPolygon(numvertices: u32, controlpolygon: *const ::win32_foundation::POINT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EvaluateProximityToPolygon(controlpolygon.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(controlpolygon)), ::core::mem::transmute(phittestinginput), ::core::mem::transmute(pproximityeval)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EvaluateProximityToRect(controlboundingbox: *const ::win32_foundation::RECT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvaluateProximityToRect(controlboundingbox: *const ::win32_foundation::RECT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(EvaluateProximityToRect(::core::mem::transmute(controlboundingbox), ::core::mem::transmute(phittestinginput), ::core::mem::transmute(pproximityeval)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FEEDBACK_TYPE(pub i32);
pub const FEEDBACK_TOUCH_CONTACTVISUALIZATION: FEEDBACK_TYPE = FEEDBACK_TYPE(1i32);
pub const FEEDBACK_PEN_BARRELVISUALIZATION: FEEDBACK_TYPE = FEEDBACK_TYPE(2i32);
pub const FEEDBACK_PEN_TAP: FEEDBACK_TYPE = FEEDBACK_TYPE(3i32);
pub const FEEDBACK_PEN_DOUBLETAP: FEEDBACK_TYPE = FEEDBACK_TYPE(4i32);
pub const FEEDBACK_PEN_PRESSANDHOLD: FEEDBACK_TYPE = FEEDBACK_TYPE(5i32);
pub const FEEDBACK_PEN_RIGHTTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(6i32);
pub const FEEDBACK_TOUCH_TAP: FEEDBACK_TYPE = FEEDBACK_TYPE(7i32);
pub const FEEDBACK_TOUCH_DOUBLETAP: FEEDBACK_TYPE = FEEDBACK_TYPE(8i32);
pub const FEEDBACK_TOUCH_PRESSANDHOLD: FEEDBACK_TYPE = FEEDBACK_TYPE(9i32);
pub const FEEDBACK_TOUCH_RIGHTTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(10i32);
pub const FEEDBACK_GESTURE_PRESSANDTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(11i32);
pub const FEEDBACK_MAX: FEEDBACK_TYPE = FEEDBACK_TYPE(-1i32);
impl ::core::marker::Copy for FEEDBACK_TYPE {}
impl ::core::clone::Clone for FEEDBACK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FEEDBACK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FEEDBACK_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FEEDBACK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FEEDBACK_TYPE").field(&self.0).finish()
    }
}
pub const FILEOPENORD: u32 = 1536u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILLTYPE(pub i32);
pub const FT_SOLID: FILLTYPE = FILLTYPE(0i32);
pub const FT_VERTGRADIENT: FILLTYPE = FILLTYPE(1i32);
pub const FT_HORZGRADIENT: FILLTYPE = FILLTYPE(2i32);
pub const FT_RADIALGRADIENT: FILLTYPE = FILLTYPE(3i32);
pub const FT_TILEIMAGE: FILLTYPE = FILLTYPE(4i32);
impl ::core::marker::Copy for FILLTYPE {}
impl ::core::clone::Clone for FILLTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FILLTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILLTYPE").field(&self.0).finish()
    }
}
pub const FINDDLGORD: u32 = 1540u32;
pub const FONTDLGORD: u32 = 1542u32;
pub const FORMATDLGORD30: u32 = 1544u32;
pub const FORMATDLGORD31: u32 = 1543u32;
pub const FSB_ENCARTA_MODE: u32 = 1u32;
pub const FSB_FLAT_MODE: u32 = 2u32;
pub const FSB_REGULAR_MODE: u32 = 0u32;
#[inline]
pub unsafe fn FlatSB_EnableScrollBar<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(param0: Param0, param1: i32, param2: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_EnableScrollBar(param0: ::win32_foundation::HWND, param1: i32, param2: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FlatSB_EnableScrollBar(param0.into_param().abi(), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_GetScrollInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut super::WindowsAndMessaging::SCROLLINFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_GetScrollInfo(param0: ::win32_foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut super::WindowsAndMessaging::SCROLLINFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FlatSB_GetScrollInfo(param0.into_param().abi(), ::core::mem::transmute(code), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_GetScrollPos<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_GetScrollPos(param0: ::win32_foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS) -> i32;
        }
        ::core::mem::transmute(FlatSB_GetScrollPos(param0.into_param().abi(), ::core::mem::transmute(code)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FlatSB_GetScrollProp<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(param0: Param0, propindex: WSB_PROP, param2: *mut i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_GetScrollProp(param0: ::win32_foundation::HWND, propindex: WSB_PROP, param2: *mut i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FlatSB_GetScrollProp(param0.into_param().abi(), ::core::mem::transmute(propindex), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_GetScrollRange<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut i32, param3: *mut i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_GetScrollRange(param0: ::win32_foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut i32, param3: *mut i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FlatSB_GetScrollRange(param0.into_param().abi(), ::core::mem::transmute(code), ::core::mem::transmute(param2), ::core::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_SetScrollInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, psi: *mut super::WindowsAndMessaging::SCROLLINFO, fredraw: Param3) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_SetScrollInfo(param0: ::win32_foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, psi: *mut super::WindowsAndMessaging::SCROLLINFO, fredraw: ::win32_foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(FlatSB_SetScrollInfo(param0.into_param().abi(), ::core::mem::transmute(code), ::core::mem::transmute(psi), fredraw.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_SetScrollPos<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, pos: i32, fredraw: Param3) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_SetScrollPos(param0: ::win32_foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, pos: i32, fredraw: ::win32_foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(FlatSB_SetScrollPos(param0.into_param().abi(), ::core::mem::transmute(code), ::core::mem::transmute(pos), fredraw.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FlatSB_SetScrollProp<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(param0: Param0, index: WSB_PROP, newvalue: isize, param3: Param3) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_SetScrollProp(param0: ::win32_foundation::HWND, index: WSB_PROP, newvalue: isize, param3: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FlatSB_SetScrollProp(param0.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(newvalue), param3.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_SetScrollRange<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, min: i32, max: i32, fredraw: Param4) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_SetScrollRange(param0: ::win32_foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, min: i32, max: i32, fredraw: ::win32_foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(FlatSB_SetScrollRange(param0.into_param().abi(), ::core::mem::transmute(code), ::core::mem::transmute(min), ::core::mem::transmute(max), fredraw.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn FlatSB_ShowScrollBar<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_ShowScrollBar(param0: ::win32_foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FlatSB_ShowScrollBar(param0.into_param().abi(), ::core::mem::transmute(code), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const GDTR_MAX: u32 = 2u32;
pub const GDTR_MIN: u32 = 1u32;
pub const GDT_ERROR: i32 = -1i32;
pub const GDT_NONE: u32 = 1u32;
pub const GDT_VALID: u32 = 0u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_THEME_BITMAP_FLAGS(pub u32);
pub const GBF_DIRECT: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(1u32);
pub const GBF_COPY: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(2u32);
pub const GBF_VALIDBITS: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(3u32);
impl ::core::marker::Copy for GET_THEME_BITMAP_FLAGS {}
impl ::core::clone::Clone for GET_THEME_BITMAP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_THEME_BITMAP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GET_THEME_BITMAP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_THEME_BITMAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_THEME_BITMAP_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLYPHFONTSIZINGTYPE(pub i32);
pub const GFST_NONE: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(0i32);
pub const GFST_SIZE: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(1i32);
pub const GFST_DPI: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(2i32);
impl ::core::marker::Copy for GLYPHFONTSIZINGTYPE {}
impl ::core::clone::Clone for GLYPHFONTSIZINGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLYPHFONTSIZINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GLYPHFONTSIZINGTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLYPHFONTSIZINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLYPHFONTSIZINGTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLYPHTYPE(pub i32);
pub const GT_NONE: GLYPHTYPE = GLYPHTYPE(0i32);
pub const GT_IMAGEGLYPH: GLYPHTYPE = GLYPHTYPE(1i32);
pub const GT_FONTGLYPH: GLYPHTYPE = GLYPHTYPE(2i32);
impl ::core::marker::Copy for GLYPHTYPE {}
impl ::core::clone::Clone for GLYPHTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLYPHTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GLYPHTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLYPHTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLYPHTYPE").field(&self.0).finish()
    }
}
pub const GMR_DAYSTATE: u32 = 1u32;
pub const GMR_VISIBLE: u32 = 0u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GRIDCELLBACKGROUNDSTATES(pub i32);
pub const MCGCB_SELECTED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(1i32);
pub const MCGCB_HOT: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(2i32);
pub const MCGCB_SELECTEDHOT: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(3i32);
pub const MCGCB_SELECTEDNOTFOCUSED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(4i32);
pub const MCGCB_TODAY: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(5i32);
pub const MCGCB_TODAYSELECTED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(6i32);
impl ::core::marker::Copy for GRIDCELLBACKGROUNDSTATES {}
impl ::core::clone::Clone for GRIDCELLBACKGROUNDSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GRIDCELLBACKGROUNDSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GRIDCELLBACKGROUNDSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GRIDCELLBACKGROUNDSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRIDCELLBACKGROUNDSTATES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GRIDCELLSTATES(pub i32);
pub const MCGC_HOT: GRIDCELLSTATES = GRIDCELLSTATES(1i32);
pub const MCGC_HASSTATE: GRIDCELLSTATES = GRIDCELLSTATES(2i32);
pub const MCGC_HASSTATEHOT: GRIDCELLSTATES = GRIDCELLSTATES(3i32);
pub const MCGC_TODAY: GRIDCELLSTATES = GRIDCELLSTATES(4i32);
pub const MCGC_TODAYSELECTED: GRIDCELLSTATES = GRIDCELLSTATES(5i32);
pub const MCGC_SELECTED: GRIDCELLSTATES = GRIDCELLSTATES(6i32);
pub const MCGC_SELECTEDHOT: GRIDCELLSTATES = GRIDCELLSTATES(7i32);
impl ::core::marker::Copy for GRIDCELLSTATES {}
impl ::core::clone::Clone for GRIDCELLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GRIDCELLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GRIDCELLSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GRIDCELLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRIDCELLSTATES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GRIDCELLUPPERSTATES(pub i32);
pub const MCGCU_HOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(1i32);
pub const MCGCU_HASSTATE: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(2i32);
pub const MCGCU_HASSTATEHOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(3i32);
pub const MCGCU_SELECTED: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(4i32);
pub const MCGCU_SELECTEDHOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(5i32);
impl ::core::marker::Copy for GRIDCELLUPPERSTATES {}
impl ::core::clone::Clone for GRIDCELLUPPERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GRIDCELLUPPERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GRIDCELLUPPERSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GRIDCELLUPPERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRIDCELLUPPERSTATES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetBufferedPaintBits(hbufferedpaint: isize, ppbbuffer: *mut *mut ::win32_graphics::Gdi::RGBQUAD, pcxrow: *mut i32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBufferedPaintBits(hbufferedpaint: isize, ppbbuffer: *mut *mut ::win32_graphics::Gdi::RGBQUAD, pcxrow: *mut i32) -> ::windows_core::HRESULT;
        }
        GetBufferedPaintBits(::core::mem::transmute(hbufferedpaint), ::core::mem::transmute(ppbbuffer), ::core::mem::transmute(pcxrow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetBufferedPaintDC(hbufferedpaint: isize) -> ::win32_graphics::Gdi::HDC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBufferedPaintDC(hbufferedpaint: isize) -> ::win32_graphics::Gdi::HDC;
        }
        ::core::mem::transmute(GetBufferedPaintDC(::core::mem::transmute(hbufferedpaint)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetBufferedPaintTargetDC(hbufferedpaint: isize) -> ::win32_graphics::Gdi::HDC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBufferedPaintTargetDC(hbufferedpaint: isize) -> ::win32_graphics::Gdi::HDC;
        }
        ::core::mem::transmute(GetBufferedPaintTargetDC(::core::mem::transmute(hbufferedpaint)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetBufferedPaintTargetRect(hbufferedpaint: isize) -> ::windows_core::Result<::win32_foundation::RECT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBufferedPaintTargetRect(hbufferedpaint: isize, prc: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        GetBufferedPaintTargetRect(::core::mem::transmute(hbufferedpaint), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetComboBoxInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwndcombo: Param0, pcbi: *mut COMBOBOXINFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetComboBoxInfo(hwndcombo: ::win32_foundation::HWND, pcbi: *mut COMBOBOXINFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetComboBoxInfo(hwndcombo.into_param().abi(), ::core::mem::transmute(pcbi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetCurrentThemeName(pszthemefilename: &mut [u16], pszcolorbuff: &mut [u16], pszsizebuff: &mut [u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentThemeName(pszthemefilename: ::windows_core::PWSTR, cchmaxnamechars: i32, pszcolorbuff: ::windows_core::PWSTR, cchmaxcolorchars: i32, pszsizebuff: ::windows_core::PWSTR, cchmaxsizechars: i32) -> ::windows_core::HRESULT;
        }
        GetCurrentThemeName(::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszthemefilename)), pszthemefilename.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszcolorbuff)), pszcolorbuff.len() as _, ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszsizebuff)), pszsizebuff.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetEffectiveClientRect<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, lprc: *mut ::win32_foundation::RECT, lpinfo: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEffectiveClientRect(hwnd: ::win32_foundation::HWND, lprc: *mut ::win32_foundation::RECT, lpinfo: *const i32);
        }
        GetEffectiveClientRect(hwnd.into_param().abi(), ::core::mem::transmute(lprc), ::core::mem::transmute(lpinfo))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetListBoxInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetListBoxInfo(hwnd: ::win32_foundation::HWND) -> u32;
        }
        ::core::mem::transmute(GetListBoxInfo(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetMUILanguage() -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMUILanguage() -> u16;
        }
        ::core::mem::transmute(GetMUILanguage())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeAnimationProperty(htheme: isize, istoryboardid: i32, itargetid: i32, eproperty: TA_PROPERTY, pvproperty: *mut ::core::ffi::c_void, cbsize: u32, pcbsizeout: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeAnimationProperty(htheme: isize, istoryboardid: i32, itargetid: i32, eproperty: TA_PROPERTY, pvproperty: *mut ::core::ffi::c_void, cbsize: u32, pcbsizeout: *mut u32) -> ::windows_core::HRESULT;
        }
        GetThemeAnimationProperty(::core::mem::transmute(htheme), ::core::mem::transmute(istoryboardid), ::core::mem::transmute(itargetid), ::core::mem::transmute(eproperty), ::core::mem::transmute(pvproperty), ::core::mem::transmute(cbsize), ::core::mem::transmute(pcbsizeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeAnimationTransform(htheme: isize, istoryboardid: i32, itargetid: i32, dwtransformindex: u32, ptransform: *mut TA_TRANSFORM, cbsize: u32, pcbsizeout: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeAnimationTransform(htheme: isize, istoryboardid: i32, itargetid: i32, dwtransformindex: u32, ptransform: *mut TA_TRANSFORM, cbsize: u32, pcbsizeout: *mut u32) -> ::windows_core::HRESULT;
        }
        GetThemeAnimationTransform(::core::mem::transmute(htheme), ::core::mem::transmute(istoryboardid), ::core::mem::transmute(itargetid), ::core::mem::transmute(dwtransformindex), ::core::mem::transmute(ptransform), ::core::mem::transmute(cbsize), ::core::mem::transmute(pcbsizeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeAppProperties() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeAppProperties() -> u32;
        }
        ::core::mem::transmute(GetThemeAppProperties())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeBackgroundContentRect<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, pboundingrect: *const ::win32_foundation::RECT) -> ::windows_core::Result<::win32_foundation::RECT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeBackgroundContentRect(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, pboundingrect: *const ::win32_foundation::RECT, pcontentrect: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        GetThemeBackgroundContentRect(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(pboundingrect), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeBackgroundExtent<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, pcontentrect: *const ::win32_foundation::RECT) -> ::windows_core::Result<::win32_foundation::RECT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeBackgroundExtent(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, pcontentrect: *const ::win32_foundation::RECT, pextentrect: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        GetThemeBackgroundExtent(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(pcontentrect), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeBackgroundRegion<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, prect: *const ::win32_foundation::RECT) -> ::windows_core::Result<::win32_graphics::Gdi::HRGN> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeBackgroundRegion(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const ::win32_foundation::RECT, pregion: *mut ::win32_graphics::Gdi::HRGN) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_graphics::Gdi::HRGN>::zeroed();
        GetThemeBackgroundRegion(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(prect), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Gdi::HRGN>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeBitmap(htheme: isize, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, dwflags: GET_THEME_BITMAP_FLAGS) -> ::windows_core::Result<::win32_graphics::Gdi::HBITMAP> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeBitmap(htheme: isize, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, dwflags: GET_THEME_BITMAP_FLAGS, phbitmap: *mut ::win32_graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_graphics::Gdi::HBITMAP>::zeroed();
        GetThemeBitmap(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(dwflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Gdi::HBITMAP>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeBool(htheme: isize, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> ::windows_core::Result<::win32_foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeBool(htheme: isize, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, pfval: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        GetThemeBool(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeColor(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeColor(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pcolor: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        GetThemeColor(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeDocumentationProperty<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszthemename: Param0, pszpropertyname: Param1, pszvaluebuff: &mut [u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeDocumentationProperty(pszthemename: ::windows_core::PCWSTR, pszpropertyname: ::windows_core::PCWSTR, pszvaluebuff: ::windows_core::PWSTR, cchmaxvalchars: i32) -> ::windows_core::HRESULT;
        }
        GetThemeDocumentationProperty(pszthemename.into_param().abi(), pszpropertyname.into_param().abi(), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszvaluebuff)), pszvaluebuff.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeEnumValue(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows_core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeEnumValue(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pival: *mut i32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        GetThemeEnumValue(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeFilename(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pszthemefilename: &mut [u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeFilename(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pszthemefilename: ::windows_core::PWSTR, cchmaxbuffchars: i32) -> ::windows_core::HRESULT;
        }
        GetThemeFilename(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszthemefilename)), pszthemefilename.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeFont<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows_core::Result<::win32_graphics::Gdi::LOGFONTW> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeFont(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, ipropid: i32, pfont: *mut ::win32_graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_graphics::Gdi::LOGFONTW>::zeroed();
        GetThemeFont(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Gdi::LOGFONTW>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeInt(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows_core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeInt(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pival: *mut i32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        GetThemeInt(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeIntList(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows_core::Result<INTLIST> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeIntList(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pintlist: *mut INTLIST) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<INTLIST>::zeroed();
        GetThemeIntList(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INTLIST>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeMargins<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, ipropid: i32, prc: *const ::win32_foundation::RECT) -> ::windows_core::Result<MARGINS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeMargins(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, ipropid: i32, prc: *const ::win32_foundation::RECT, pmargins: *mut MARGINS) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<MARGINS>::zeroed();
        GetThemeMargins(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(prc), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<MARGINS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeMetric<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> ::windows_core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeMetric(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, pival: *mut i32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        GetThemeMetric(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemePartSize<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, prc: *const ::win32_foundation::RECT, esize: THEMESIZE) -> ::windows_core::Result<::win32_foundation::SIZE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemePartSize(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, prc: *const ::win32_foundation::RECT, esize: THEMESIZE, psz: *mut ::win32_foundation::SIZE) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::SIZE>::zeroed();
        GetThemePartSize(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(prc), ::core::mem::transmute(esize), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::SIZE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemePosition(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows_core::Result<::win32_foundation::POINT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemePosition(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, ppoint: *mut ::win32_foundation::POINT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::POINT>::zeroed();
        GetThemePosition(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::POINT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemePropertyOrigin(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows_core::Result<PROPERTYORIGIN> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemePropertyOrigin(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, porigin: *mut PROPERTYORIGIN) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<PROPERTYORIGIN>::zeroed();
        GetThemePropertyOrigin(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROPERTYORIGIN>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeRect(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows_core::Result<::win32_foundation::RECT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeRect(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, prect: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        GetThemeRect(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeStream<'a, Param6: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>>(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, ppvstream: *mut *mut ::core::ffi::c_void, pcbstream: *mut u32, hinst: Param6) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeStream(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, ppvstream: *mut *mut ::core::ffi::c_void, pcbstream: *mut u32, hinst: ::win32_foundation::HINSTANCE) -> ::windows_core::HRESULT;
        }
        GetThemeStream(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(ppvstream), ::core::mem::transmute(pcbstream), hinst.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeString(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pszbuff: &mut [u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeString(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pszbuff: ::windows_core::PWSTR, cchmaxbuffchars: i32) -> ::windows_core::HRESULT;
        }
        GetThemeString(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(ipropid), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszbuff)), pszbuff.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeSysBool(htheme: isize, iboolid: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysBool(htheme: isize, iboolid: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetThemeSysBool(::core::mem::transmute(htheme), ::core::mem::transmute(iboolid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeSysColor(htheme: isize, icolorid: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysColor(htheme: isize, icolorid: i32) -> u32;
        }
        ::core::mem::transmute(GetThemeSysColor(::core::mem::transmute(htheme), ::core::mem::transmute(icolorid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeSysColorBrush(htheme: isize, icolorid: THEME_PROPERTY_SYMBOL_ID) -> ::win32_graphics::Gdi::HBRUSH {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysColorBrush(htheme: isize, icolorid: THEME_PROPERTY_SYMBOL_ID) -> ::win32_graphics::Gdi::HBRUSH;
        }
        ::core::mem::transmute(GetThemeSysColorBrush(::core::mem::transmute(htheme), ::core::mem::transmute(icolorid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeSysFont(htheme: isize, ifontid: THEME_PROPERTY_SYMBOL_ID) -> ::windows_core::Result<::win32_graphics::Gdi::LOGFONTW> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysFont(htheme: isize, ifontid: THEME_PROPERTY_SYMBOL_ID, plf: *mut ::win32_graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_graphics::Gdi::LOGFONTW>::zeroed();
        GetThemeSysFont(::core::mem::transmute(htheme), ::core::mem::transmute(ifontid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Gdi::LOGFONTW>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeSysInt(htheme: isize, iintid: i32) -> ::windows_core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysInt(htheme: isize, iintid: i32, pivalue: *mut i32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        GetThemeSysInt(::core::mem::transmute(htheme), ::core::mem::transmute(iintid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeSysSize(htheme: isize, isizeid: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysSize(htheme: isize, isizeid: i32) -> i32;
        }
        ::core::mem::transmute(GetThemeSysSize(::core::mem::transmute(htheme), ::core::mem::transmute(isizeid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeSysString(htheme: isize, istringid: THEME_PROPERTY_SYMBOL_ID, pszstringbuff: &mut [u16]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysString(htheme: isize, istringid: THEME_PROPERTY_SYMBOL_ID, pszstringbuff: ::windows_core::PWSTR, cchmaxstringchars: i32) -> ::windows_core::HRESULT;
        }
        GetThemeSysString(::core::mem::transmute(htheme), ::core::mem::transmute(istringid), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszstringbuff)), pszstringbuff.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeTextExtent<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, psztext: &[u16], dwtextflags: u32, pboundingrect: *const ::win32_foundation::RECT) -> ::windows_core::Result<::win32_foundation::RECT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeTextExtent(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: ::windows_core::PCWSTR, cchcharcount: i32, dwtextflags: u32, pboundingrect: *const ::win32_foundation::RECT, pextentrect: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        GetThemeTextExtent(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(::windows_core::as_ptr_or_null(psztext)), psztext.len() as _, ::core::mem::transmute(dwtextflags), ::core::mem::transmute(pboundingrect), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeTextMetrics<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32) -> ::windows_core::Result<::win32_graphics::Gdi::TEXTMETRICW> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeTextMetrics(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, ptm: *mut ::win32_graphics::Gdi::TEXTMETRICW) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_graphics::Gdi::TEXTMETRICW>::zeroed();
        GetThemeTextMetrics(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_graphics::Gdi::TEXTMETRICW>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeTimingFunction(htheme: isize, itimingfunctionid: i32, ptimingfunction: *mut TA_TIMINGFUNCTION, cbsize: u32, pcbsizeout: *mut u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeTimingFunction(htheme: isize, itimingfunctionid: i32, ptimingfunction: *mut TA_TIMINGFUNCTION, cbsize: u32, pcbsizeout: *mut u32) -> ::windows_core::HRESULT;
        }
        GetThemeTimingFunction(::core::mem::transmute(htheme), ::core::mem::transmute(itimingfunctionid), ::core::mem::transmute(ptimingfunction), ::core::mem::transmute(cbsize), ::core::mem::transmute(pcbsizeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThemeTransitionDuration(htheme: isize, ipartid: i32, istateidfrom: i32, istateidto: i32, ipropid: i32) -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeTransitionDuration(htheme: isize, ipartid: i32, istateidfrom: i32, istateidto: i32, ipropid: i32, pdwduration: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        GetThemeTransitionDuration(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateidfrom), ::core::mem::transmute(istateidto), ::core::mem::transmute(ipropid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetWindowFeedbackSetting<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, feedback: FEEDBACK_TYPE, dwflags: u32, psize: *mut u32, config: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowFeedbackSetting(hwnd: ::win32_foundation::HWND, feedback: FEEDBACK_TYPE, dwflags: u32, psize: *mut u32, config: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetWindowFeedbackSetting(hwnd.into_param().abi(), ::core::mem::transmute(feedback), ::core::mem::transmute(dwflags), ::core::mem::transmute(psize), ::core::mem::transmute(config)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetWindowTheme<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowTheme(hwnd: ::win32_foundation::HWND) -> isize;
        }
        ::core::mem::transmute(GetWindowTheme(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HALIGN(pub i32);
pub const HA_LEFT: HALIGN = HALIGN(0i32);
pub const HA_CENTER: HALIGN = HALIGN(1i32);
pub const HA_RIGHT: HALIGN = HALIGN(2i32);
impl ::core::marker::Copy for HALIGN {}
impl ::core::clone::Clone for HALIGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HALIGN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HALIGN {
    type Abi = Self;
}
impl ::core::fmt::Debug for HALIGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HALIGN").field(&self.0).finish()
    }
}
pub const HDFT_HASNOVALUE: u32 = 32768u32;
pub const HDFT_ISDATE: u32 = 2u32;
pub const HDFT_ISNUMBER: u32 = 1u32;
pub const HDFT_ISSTRING: u32 = 0u32;
pub const HDF_BITMAP: u32 = 8192u32;
pub const HDF_BITMAP_ON_RIGHT: u32 = 4096u32;
pub const HDF_CENTER: u32 = 2u32;
pub const HDF_CHECKBOX: u32 = 64u32;
pub const HDF_CHECKED: u32 = 128u32;
pub const HDF_FIXEDWIDTH: u32 = 256u32;
pub const HDF_IMAGE: u32 = 2048u32;
pub const HDF_JUSTIFYMASK: u32 = 3u32;
pub const HDF_LEFT: u32 = 0u32;
pub const HDF_OWNERDRAW: u32 = 32768u32;
pub const HDF_RIGHT: u32 = 1u32;
pub const HDF_RTLREADING: u32 = 4u32;
pub const HDF_SORTDOWN: u32 = 512u32;
pub const HDF_SORTUP: u32 = 1024u32;
pub const HDF_SPLITBUTTON: u32 = 16777216u32;
pub const HDF_STRING: u32 = 16384u32;
#[repr(C)]
pub struct HDHITTESTINFO {
    pub pt: ::win32_foundation::POINT,
    pub flags: u32,
    pub iItem: i32,
}
impl ::core::marker::Copy for HDHITTESTINFO {}
impl ::core::clone::Clone for HDHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HDHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).field("iItem", &self.iItem).finish()
    }
}
unsafe impl ::windows_core::Abi for HDHITTESTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HDHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HDHITTESTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HDHITTESTINFO {}
impl ::core::default::Default for HDHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const HDIS_FOCUSED: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct HDITEMA {
    pub mask: HDI_MASK,
    pub cxy: i32,
    pub pszText: ::windows_core::PSTR,
    pub hbm: ::win32_graphics::Gdi::HBITMAP,
    pub cchTextMax: i32,
    pub fmt: i32,
    pub lParam: ::win32_foundation::LPARAM,
    pub iImage: i32,
    pub iOrder: i32,
    pub r#type: u32,
    pub pvFilter: *mut ::core::ffi::c_void,
    pub state: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for HDITEMA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for HDITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for HDITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDITEMA").field("mask", &self.mask).field("cxy", &self.cxy).field("pszText", &self.pszText).field("hbm", &self.hbm).field("cchTextMax", &self.cchTextMax).field("fmt", &self.fmt).field("lParam", &self.lParam).field("iImage", &self.iImage).field("iOrder", &self.iOrder).field("type", &self.r#type).field("pvFilter", &self.pvFilter).field("state", &self.state).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for HDITEMA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for HDITEMA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HDITEMA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for HDITEMA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for HDITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct HDITEMW {
    pub mask: HDI_MASK,
    pub cxy: i32,
    pub pszText: ::windows_core::PWSTR,
    pub hbm: ::win32_graphics::Gdi::HBITMAP,
    pub cchTextMax: i32,
    pub fmt: i32,
    pub lParam: ::win32_foundation::LPARAM,
    pub iImage: i32,
    pub iOrder: i32,
    pub r#type: u32,
    pub pvFilter: *mut ::core::ffi::c_void,
    pub state: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for HDITEMW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for HDITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for HDITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDITEMW").field("mask", &self.mask).field("cxy", &self.cxy).field("pszText", &self.pszText).field("hbm", &self.hbm).field("cchTextMax", &self.cchTextMax).field("fmt", &self.fmt).field("lParam", &self.lParam).field("iImage", &self.iImage).field("iOrder", &self.iOrder).field("type", &self.r#type).field("pvFilter", &self.pvFilter).field("state", &self.state).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for HDITEMW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for HDITEMW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HDITEMW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for HDITEMW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for HDITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HDI_MASK(pub u32);
pub const HDI_WIDTH: HDI_MASK = HDI_MASK(1u32);
pub const HDI_HEIGHT: HDI_MASK = HDI_MASK(1u32);
pub const HDI_TEXT: HDI_MASK = HDI_MASK(2u32);
pub const HDI_FORMAT: HDI_MASK = HDI_MASK(4u32);
pub const HDI_LPARAM: HDI_MASK = HDI_MASK(8u32);
pub const HDI_BITMAP: HDI_MASK = HDI_MASK(16u32);
pub const HDI_IMAGE: HDI_MASK = HDI_MASK(32u32);
pub const HDI_DI_SETITEM: HDI_MASK = HDI_MASK(64u32);
pub const HDI_ORDER: HDI_MASK = HDI_MASK(128u32);
pub const HDI_FILTER: HDI_MASK = HDI_MASK(256u32);
pub const HDI_STATE: HDI_MASK = HDI_MASK(512u32);
impl ::core::marker::Copy for HDI_MASK {}
impl ::core::clone::Clone for HDI_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HDI_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HDI_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for HDI_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDI_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HDI_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HDI_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HDI_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HDI_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HDI_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct HDLAYOUT {
    pub prc: *mut ::win32_foundation::RECT,
    pub pwpos: *mut super::WindowsAndMessaging::WINDOWPOS,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for HDLAYOUT {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for HDLAYOUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::fmt::Debug for HDLAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDLAYOUT").field("prc", &self.prc).field("pwpos", &self.pwpos).finish()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows_core::Abi for HDLAYOUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for HDLAYOUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HDLAYOUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for HDLAYOUT {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for HDLAYOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const HDM_CLEARFILTER: u32 = 4632u32;
pub const HDM_CREATEDRAGIMAGE: u32 = 4624u32;
pub const HDM_DELETEITEM: u32 = 4610u32;
pub const HDM_EDITFILTER: u32 = 4631u32;
pub const HDM_FIRST: u32 = 4608u32;
pub const HDM_GETBITMAPMARGIN: u32 = 4629u32;
pub const HDM_GETFOCUSEDITEM: u32 = 4635u32;
pub const HDM_GETIMAGELIST: u32 = 4617u32;
pub const HDM_GETITEM: u32 = 4619u32;
pub const HDM_GETITEMA: u32 = 4611u32;
pub const HDM_GETITEMCOUNT: u32 = 4608u32;
pub const HDM_GETITEMDROPDOWNRECT: u32 = 4633u32;
pub const HDM_GETITEMRECT: u32 = 4615u32;
pub const HDM_GETITEMW: u32 = 4619u32;
pub const HDM_GETORDERARRAY: u32 = 4625u32;
pub const HDM_GETOVERFLOWRECT: u32 = 4634u32;
pub const HDM_GETUNICODEFORMAT: u32 = 8198u32;
pub const HDM_HITTEST: u32 = 4614u32;
pub const HDM_INSERTITEM: u32 = 4618u32;
pub const HDM_INSERTITEMA: u32 = 4609u32;
pub const HDM_INSERTITEMW: u32 = 4618u32;
pub const HDM_LAYOUT: u32 = 4613u32;
pub const HDM_ORDERTOINDEX: u32 = 4623u32;
pub const HDM_SETBITMAPMARGIN: u32 = 4628u32;
pub const HDM_SETFILTERCHANGETIMEOUT: u32 = 4630u32;
pub const HDM_SETFOCUSEDITEM: u32 = 4636u32;
pub const HDM_SETHOTDIVIDER: u32 = 4627u32;
pub const HDM_SETIMAGELIST: u32 = 4616u32;
pub const HDM_SETITEM: u32 = 4620u32;
pub const HDM_SETITEMA: u32 = 4612u32;
pub const HDM_SETITEMW: u32 = 4620u32;
pub const HDM_SETORDERARRAY: u32 = 4626u32;
pub const HDM_SETUNICODEFORMAT: u32 = 8197u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HDPA(pub isize);
impl HDPA {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HDPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDPA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDPA {}
impl ::core::fmt::Debug for HDPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDPA").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HDPA {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HDSA(pub isize);
impl HDSA {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HDSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HDSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HDSA {}
impl ::core::fmt::Debug for HDSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDSA").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HDSA {
    type Abi = Self;
}
pub const HDSIL_NORMAL: u32 = 0u32;
pub const HDSIL_STATE: u32 = 1u32;
pub const HDS_BUTTONS: u32 = 2u32;
pub const HDS_CHECKBOXES: u32 = 1024u32;
pub const HDS_DRAGDROP: u32 = 64u32;
pub const HDS_FILTERBAR: u32 = 256u32;
pub const HDS_FLAT: u32 = 512u32;
pub const HDS_FULLDRAG: u32 = 128u32;
pub const HDS_HIDDEN: u32 = 8u32;
pub const HDS_HORZ: u32 = 0u32;
pub const HDS_HOTTRACK: u32 = 4u32;
pub const HDS_NOSIZING: u32 = 2048u32;
pub const HDS_OVERFLOW: u32 = 4096u32;
#[repr(C)]
pub struct HD_TEXTFILTERA {
    pub pszText: ::windows_core::PSTR,
    pub cchTextMax: i32,
}
impl ::core::marker::Copy for HD_TEXTFILTERA {}
impl ::core::clone::Clone for HD_TEXTFILTERA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HD_TEXTFILTERA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HD_TEXTFILTERA").field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).finish()
    }
}
unsafe impl ::windows_core::Abi for HD_TEXTFILTERA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HD_TEXTFILTERA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HD_TEXTFILTERA>()) == 0 }
    }
}
impl ::core::cmp::Eq for HD_TEXTFILTERA {}
impl ::core::default::Default for HD_TEXTFILTERA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct HD_TEXTFILTERW {
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
}
impl ::core::marker::Copy for HD_TEXTFILTERW {}
impl ::core::clone::Clone for HD_TEXTFILTERW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HD_TEXTFILTERW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HD_TEXTFILTERW").field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).finish()
    }
}
unsafe impl ::windows_core::Abi for HD_TEXTFILTERW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HD_TEXTFILTERW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HD_TEXTFILTERW>()) == 0 }
    }
}
impl ::core::cmp::Eq for HD_TEXTFILTERW {}
impl ::core::default::Default for HD_TEXTFILTERW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HEADER_CONTROL_NOTIFICATION_BUTTON(pub u32);
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_LEFT: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(0u32);
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_RIGHT: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(1u32);
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_MIDDLE: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(2u32);
impl ::core::marker::Copy for HEADER_CONTROL_NOTIFICATION_BUTTON {}
impl ::core::clone::Clone for HEADER_CONTROL_NOTIFICATION_BUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HEADER_CONTROL_NOTIFICATION_BUTTON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HEADER_CONTROL_NOTIFICATION_BUTTON {
    type Abi = Self;
}
impl ::core::fmt::Debug for HEADER_CONTROL_NOTIFICATION_BUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEADER_CONTROL_NOTIFICATION_BUTTON").field(&self.0).finish()
    }
}
pub const HHT_ABOVE: u32 = 256u32;
pub const HHT_BELOW: u32 = 512u32;
pub const HHT_NOWHERE: u32 = 1u32;
pub const HHT_ONDIVIDER: u32 = 4u32;
pub const HHT_ONDIVOPEN: u32 = 8u32;
pub const HHT_ONDROPDOWN: u32 = 8192u32;
pub const HHT_ONFILTER: u32 = 16u32;
pub const HHT_ONFILTERBUTTON: u32 = 32u32;
pub const HHT_ONHEADER: u32 = 2u32;
pub const HHT_ONITEMSTATEICON: u32 = 4096u32;
pub const HHT_ONOVERFLOW: u32 = 16384u32;
pub const HHT_TOLEFT: u32 = 2048u32;
pub const HHT_TORIGHT: u32 = 1024u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HIMAGELIST(pub isize);
impl HIMAGELIST {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HIMAGELIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HIMAGELIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HIMAGELIST {}
impl ::core::fmt::Debug for HIMAGELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIMAGELIST").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HIMAGELIST {
    type Abi = Self;
}
#[inline]
pub unsafe fn HIMAGELIST_QueryInterface<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HIMAGELIST_QueryInterface(himl: HIMAGELIST, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        HIMAGELIST_QueryInterface(himl.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const HIST_ADDTOFAVORITES: u32 = 3u32;
pub const HIST_BACK: u32 = 0u32;
pub const HIST_FAVORITES: u32 = 2u32;
pub const HIST_FORWARD: u32 = 1u32;
pub const HIST_VIEWTREE: u32 = 4u32;
pub const HKCOMB_A: u32 = 8u32;
pub const HKCOMB_C: u32 = 4u32;
pub const HKCOMB_CA: u32 = 64u32;
pub const HKCOMB_NONE: u32 = 1u32;
pub const HKCOMB_S: u32 = 2u32;
pub const HKCOMB_SA: u32 = 32u32;
pub const HKCOMB_SC: u32 = 16u32;
pub const HKCOMB_SCA: u32 = 128u32;
pub const HKM_GETHOTKEY: u32 = 1026u32;
pub const HKM_SETHOTKEY: u32 = 1025u32;
pub const HKM_SETRULES: u32 = 1027u32;
pub const HOTKEYF_ALT: u32 = 4u32;
pub const HOTKEYF_CONTROL: u32 = 2u32;
pub const HOTKEYF_EXT: u32 = 128u32;
pub const HOTKEYF_SHIFT: u32 = 1u32;
pub const HOTKEY_CLASS: &str = "msctls_hotkey32";
pub const HOTKEY_CLASSA: &str = "msctls_hotkey32";
pub const HOTKEY_CLASSW: &str = "msctls_hotkey32";
pub const HOVER_DEFAULT: u32 = 4294967295u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HPROPSHEETPAGE(pub isize);
impl HPROPSHEETPAGE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HPROPSHEETPAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HPROPSHEETPAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HPROPSHEETPAGE {}
impl ::core::fmt::Debug for HPROPSHEETPAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HPROPSHEETPAGE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HPROPSHEETPAGE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HSYNTHETICPOINTERDEVICE(pub isize);
impl HSYNTHETICPOINTERDEVICE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HSYNTHETICPOINTERDEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HSYNTHETICPOINTERDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HSYNTHETICPOINTERDEVICE {}
impl ::core::fmt::Debug for HSYNTHETICPOINTERDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HSYNTHETICPOINTERDEVICE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HSYNTHETICPOINTERDEVICE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTREEITEM(pub isize);
impl ::core::default::Default for HTREEITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HTREEITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HTREEITEM {}
impl ::core::fmt::Debug for HTREEITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTREEITEM").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for HTREEITEM {
    type Abi = Self;
}
pub const HTTB_BACKGROUNDSEG: u32 = 0u32;
pub const HTTB_CAPTION: u32 = 4u32;
pub const HTTB_FIXEDBORDER: u32 = 2u32;
pub const HTTB_RESIZINGBORDER_BOTTOM: u32 = 128u32;
pub const HTTB_RESIZINGBORDER_LEFT: u32 = 16u32;
pub const HTTB_RESIZINGBORDER_RIGHT: u32 = 64u32;
pub const HTTB_RESIZINGBORDER_TOP: u32 = 32u32;
pub const HTTB_SIZINGTEMPLATE: u32 = 256u32;
pub const HTTB_SYSTEMSIZINGMARGINS: u32 = 512u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HYPERLINKSTATES(pub i32);
pub const HLS_NORMALTEXT: HYPERLINKSTATES = HYPERLINKSTATES(1i32);
pub const HLS_LINKTEXT: HYPERLINKSTATES = HYPERLINKSTATES(2i32);
impl ::core::marker::Copy for HYPERLINKSTATES {}
impl ::core::clone::Clone for HYPERLINKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HYPERLINKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for HYPERLINKSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for HYPERLINKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HYPERLINKSTATES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HitTestThemeBackground<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>, Param6: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HRGN>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::POINT>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, dwoptions: u32, prect: *const ::win32_foundation::RECT, hrgn: Param6, pttest: Param7) -> ::windows_core::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HitTestThemeBackground(htheme: isize, hdc: ::win32_graphics::Gdi::HDC, ipartid: i32, istateid: i32, dwoptions: u32, prect: *const ::win32_foundation::RECT, hrgn: ::win32_graphics::Gdi::HRGN, pttest: ::win32_foundation::POINT, pwhittestcode: *mut u16) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        HitTestThemeBackground(::core::mem::transmute(htheme), hdc.into_param().abi(), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid), ::core::mem::transmute(dwoptions), ::core::mem::transmute(prect), hrgn.into_param().abi(), pttest.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ICONEFFECT(pub i32);
pub const ICE_NONE: ICONEFFECT = ICONEFFECT(0i32);
pub const ICE_GLOW: ICONEFFECT = ICONEFFECT(1i32);
pub const ICE_SHADOW: ICONEFFECT = ICONEFFECT(2i32);
pub const ICE_PULSE: ICONEFFECT = ICONEFFECT(3i32);
pub const ICE_ALPHA: ICONEFFECT = ICONEFFECT(4i32);
impl ::core::marker::Copy for ICONEFFECT {}
impl ::core::clone::Clone for ICONEFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ICONEFFECT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ICONEFFECT {
    type Abi = Self;
}
impl ::core::fmt::Debug for ICONEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICONEFFECT").field(&self.0).finish()
    }
}
pub const IDB_HIST_DISABLED: u32 = 14u32;
pub const IDB_HIST_HOT: u32 = 13u32;
pub const IDB_HIST_LARGE_COLOR: u32 = 9u32;
pub const IDB_HIST_NORMAL: u32 = 12u32;
pub const IDB_HIST_PRESSED: u32 = 15u32;
pub const IDB_HIST_SMALL_COLOR: u32 = 8u32;
pub const IDB_STD_LARGE_COLOR: u32 = 1u32;
pub const IDB_STD_SMALL_COLOR: u32 = 0u32;
pub const IDB_VIEW_LARGE_COLOR: u32 = 5u32;
pub const IDB_VIEW_SMALL_COLOR: u32 = 4u32;
pub const IDC_MANAGE_LINK: u32 = 1592u32;
pub const ID_PSRESTARTWINDOWS: u32 = 2u32;
#[repr(transparent)]
pub struct IImageList(::windows_core::IUnknown);
impl IImageList {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>>(&self, hbmimage: Param0, hbmmask: Param1) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), hbmimage.into_param().abi(), hbmmask.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn ReplaceIcon<'a, Param1: ::windows_core::IntoParam<'a, super::WindowsAndMessaging::HICON>>(&self, i: i32, hicon: Param1) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ReplaceIcon)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), hicon.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetOverlayImage(&self, iimage: i32, ioverlay: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOverlayImage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iimage), ::core::mem::transmute(ioverlay)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Replace<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>, Param2: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>>(&self, i: i32, hbmimage: Param1, hbmmask: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Replace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), hbmimage.into_param().abi(), hbmmask.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AddMasked<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>>(&self, hbmimage: Param0, crmask: u32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).AddMasked)(::windows_core::Interface::as_raw(self), hbmimage.into_param().abi(), ::core::mem::transmute(crmask), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Draw(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Draw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimldp)).ok()
    }
    pub unsafe fn Remove(&self, i: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon(&self, i: i32, flags: u32) -> ::windows_core::Result<super::WindowsAndMessaging::HICON> {
        let mut result__ = ::core::mem::MaybeUninit::<super::WindowsAndMessaging::HICON>::zeroed();
        (::windows_core::Interface::vtable(self).GetIcon)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetImageInfo(&self, i: i32) -> ::windows_core::Result<IMAGEINFO> {
        let mut result__ = ::core::mem::MaybeUninit::<IMAGEINFO>::zeroed();
        (::windows_core::Interface::vtable(self).GetImageInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAGEINFO>(result__)
    }
    pub unsafe fn Copy<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, idst: i32, punksrc: Param1, isrc: i32, uflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Copy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(idst), punksrc.into_param().abi(), ::core::mem::transmute(isrc), ::core::mem::transmute(uflags)).ok()
    }
    pub unsafe fn Merge<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, i1: i32, punk2: Param1, i2: i32, dx: i32, dy: i32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Merge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i1), punk2.into_param().abi(), ::core::mem::transmute(i2), ::core::mem::transmute(dx), ::core::mem::transmute(dy), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn Clone(&self, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn GetImageRect(&self, i: i32) -> ::windows_core::Result<::win32_foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        (::windows_core::Interface::vtable(self).GetImageRect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    pub unsafe fn GetIconSize(&self, cx: *mut i32, cy: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetIconSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cx), ::core::mem::transmute(cy)).ok()
    }
    pub unsafe fn SetIconSize(&self, cx: i32, cy: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIconSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cx), ::core::mem::transmute(cy)).ok()
    }
    pub unsafe fn GetImageCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetImageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetImageCount(&self, unewcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetImageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(unewcount)).ok()
    }
    pub unsafe fn SetBkColor(&self, clrbk: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).SetBkColor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(clrbk), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBkColor(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetBkColor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn BeginDrag(&self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginDrag)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(itrack), ::core::mem::transmute(dxhotspot), ::core::mem::transmute(dyhotspot)).ok()
    }
    pub unsafe fn EndDrag(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndDrag)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DragEnter<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndlock: Param0, x: i32, y: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DragEnter)(::windows_core::Interface::as_raw(self), hwndlock.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    pub unsafe fn DragLeave<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndlock: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DragLeave)(::windows_core::Interface::as_raw(self), hwndlock.into_param().abi()).ok()
    }
    pub unsafe fn DragMove(&self, x: i32, y: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DragMove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    pub unsafe fn SetDragCursorImage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punk: Param0, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDragCursorImage)(::windows_core::Interface::as_raw(self), punk.into_param().abi(), ::core::mem::transmute(idrag), ::core::mem::transmute(dxhotspot), ::core::mem::transmute(dyhotspot)).ok()
    }
    pub unsafe fn DragShowNolock<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fshow: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DragShowNolock)(::windows_core::Interface::as_raw(self), fshow.into_param().abi()).ok()
    }
    pub unsafe fn GetDragImage(&self, ppt: *mut ::win32_foundation::POINT, ppthotspot: *mut ::win32_foundation::POINT, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDragImage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppt), ::core::mem::transmute(ppthotspot), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn GetItemFlags(&self, i: i32) -> ::windows_core::Result<IMAGE_LIST_ITEM_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<IMAGE_LIST_ITEM_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).GetItemFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAGE_LIST_ITEM_FLAGS>(result__)
    }
    pub unsafe fn GetOverlayImage(&self, ioverlay: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetOverlayImage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ioverlay), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IImageList> for ::windows_core::IUnknown {
    fn from(value: IImageList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImageList> for ::windows_core::IUnknown {
    fn from(value: &IImageList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IImageList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IImageList {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IImageList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImageList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageList {}
impl ::core::fmt::Debug for IImageList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageList").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IImageList {
    type Vtable = IImageList_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46eb5926_582e_4017_9fdf_e8998daa0950);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageList_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hbmimage: ::win32_graphics::Gdi::HBITMAP, hbmmask: ::win32_graphics::Gdi::HBITMAP, pi: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Add: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub ReplaceIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, hicon: super::WindowsAndMessaging::HICON, pi: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    ReplaceIcon: usize,
    pub SetOverlayImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimage: i32, ioverlay: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Replace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, hbmimage: ::win32_graphics::Gdi::HBITMAP, hbmmask: ::win32_graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Replace: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AddMasked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hbmimage: ::win32_graphics::Gdi::HBITMAP, crmask: u32, pi: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AddMasked: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Draw: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, flags: u32, picon: *mut super::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetImageInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, pimageinfo: *mut IMAGEINFO) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetImageInfo: usize,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idst: i32, punksrc: *mut ::core::ffi::c_void, isrc: i32, uflags: u32) -> ::windows_core::HRESULT,
    pub Merge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i1: i32, punk2: *mut ::core::ffi::c_void, i2: i32, dx: i32, dy: i32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetImageRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, prc: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    pub GetIconSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cx: *mut i32, cy: *mut i32) -> ::windows_core::HRESULT,
    pub SetIconSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cx: i32, cy: i32) -> ::windows_core::HRESULT,
    pub GetImageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pi: *mut i32) -> ::windows_core::HRESULT,
    pub SetImageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unewcount: u32) -> ::windows_core::HRESULT,
    pub SetBkColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clrbk: u32, pclr: *mut u32) -> ::windows_core::HRESULT,
    pub GetBkColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclr: *mut u32) -> ::windows_core::HRESULT,
    pub BeginDrag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows_core::HRESULT,
    pub EndDrag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DragEnter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndlock: ::win32_foundation::HWND, x: i32, y: i32) -> ::windows_core::HRESULT,
    pub DragLeave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndlock: ::win32_foundation::HWND) -> ::windows_core::HRESULT,
    pub DragMove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows_core::HRESULT,
    pub SetDragCursorImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows_core::HRESULT,
    pub DragShowNolock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetDragImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppt: *mut ::win32_foundation::POINT, ppthotspot: *mut ::win32_foundation::POINT, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetItemFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, dwflags: *mut IMAGE_LIST_ITEM_FLAGS) -> ::windows_core::HRESULT,
    pub GetOverlayImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ioverlay: i32, piindex: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IImageList2(::windows_core::IUnknown);
impl IImageList2 {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>>(&self, hbmimage: Param0, hbmmask: Param1) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Add)(::windows_core::Interface::as_raw(self), hbmimage.into_param().abi(), hbmmask.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn ReplaceIcon<'a, Param1: ::windows_core::IntoParam<'a, super::WindowsAndMessaging::HICON>>(&self, i: i32, hicon: Param1) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReplaceIcon)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), hicon.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetOverlayImage(&self, iimage: i32, ioverlay: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOverlayImage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iimage), ::core::mem::transmute(ioverlay)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Replace<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>, Param2: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>>(&self, i: i32, hbmimage: Param1, hbmmask: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Replace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), hbmimage.into_param().abi(), hbmmask.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AddMasked<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>>(&self, hbmimage: Param0, crmask: u32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddMasked)(::windows_core::Interface::as_raw(self), hbmimage.into_param().abi(), ::core::mem::transmute(crmask), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Draw(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Draw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimldp)).ok()
    }
    pub unsafe fn Remove(&self, i: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Remove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon(&self, i: i32, flags: u32) -> ::windows_core::Result<super::WindowsAndMessaging::HICON> {
        let mut result__ = ::core::mem::MaybeUninit::<super::WindowsAndMessaging::HICON>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetIcon)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetImageInfo(&self, i: i32) -> ::windows_core::Result<IMAGEINFO> {
        let mut result__ = ::core::mem::MaybeUninit::<IMAGEINFO>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetImageInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAGEINFO>(result__)
    }
    pub unsafe fn Copy<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, idst: i32, punksrc: Param1, isrc: i32, uflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Copy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(idst), punksrc.into_param().abi(), ::core::mem::transmute(isrc), ::core::mem::transmute(uflags)).ok()
    }
    pub unsafe fn Merge<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, i1: i32, punk2: Param1, i2: i32, dx: i32, dy: i32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Merge)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i1), punk2.into_param().abi(), ::core::mem::transmute(i2), ::core::mem::transmute(dx), ::core::mem::transmute(dy), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn Clone(&self, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn GetImageRect(&self, i: i32) -> ::windows_core::Result<::win32_foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetImageRect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    pub unsafe fn GetIconSize(&self, cx: *mut i32, cy: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetIconSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cx), ::core::mem::transmute(cy)).ok()
    }
    pub unsafe fn SetIconSize(&self, cx: i32, cy: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIconSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cx), ::core::mem::transmute(cy)).ok()
    }
    pub unsafe fn GetImageCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetImageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetImageCount(&self, unewcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetImageCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(unewcount)).ok()
    }
    pub unsafe fn SetBkColor(&self, clrbk: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SetBkColor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(clrbk), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetBkColor(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBkColor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn BeginDrag(&self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginDrag)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(itrack), ::core::mem::transmute(dxhotspot), ::core::mem::transmute(dyhotspot)).ok()
    }
    pub unsafe fn EndDrag(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndDrag)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DragEnter<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndlock: Param0, x: i32, y: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DragEnter)(::windows_core::Interface::as_raw(self), hwndlock.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    pub unsafe fn DragLeave<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, hwndlock: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DragLeave)(::windows_core::Interface::as_raw(self), hwndlock.into_param().abi()).ok()
    }
    pub unsafe fn DragMove(&self, x: i32, y: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DragMove)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    pub unsafe fn SetDragCursorImage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punk: Param0, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDragCursorImage)(::windows_core::Interface::as_raw(self), punk.into_param().abi(), ::core::mem::transmute(idrag), ::core::mem::transmute(dxhotspot), ::core::mem::transmute(dyhotspot)).ok()
    }
    pub unsafe fn DragShowNolock<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fshow: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DragShowNolock)(::windows_core::Interface::as_raw(self), fshow.into_param().abi()).ok()
    }
    pub unsafe fn GetDragImage(&self, ppt: *mut ::win32_foundation::POINT, ppthotspot: *mut ::win32_foundation::POINT, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDragImage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppt), ::core::mem::transmute(ppthotspot), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn GetItemFlags(&self, i: i32) -> ::windows_core::Result<IMAGE_LIST_ITEM_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::<IMAGE_LIST_ITEM_FLAGS>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetItemFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAGE_LIST_ITEM_FLAGS>(result__)
    }
    pub unsafe fn GetOverlayImage(&self, ioverlay: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOverlayImage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ioverlay), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Resize(&self, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cxnewiconsize), ::core::mem::transmute(cynewiconsize)).ok()
    }
    pub unsafe fn GetOriginalSize(&self, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOriginalSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iimage), ::core::mem::transmute(dwflags), ::core::mem::transmute(pcx), ::core::mem::transmute(pcy)).ok()
    }
    pub unsafe fn SetOriginalSize(&self, iimage: i32, cx: i32, cy: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOriginalSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iimage), ::core::mem::transmute(cx), ::core::mem::transmute(cy)).ok()
    }
    pub unsafe fn SetCallback<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, punk: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCallback)(::windows_core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn GetCallback(&self, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCallback)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn ForceImagePresent(&self, iimage: i32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ForceImagePresent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(iimage), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn DiscardImages(&self, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DiscardImages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ifirstimage), ::core::mem::transmute(ilastimage), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn PreloadImages(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PreloadImages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pimldp)).ok()
    }
    pub unsafe fn GetStatistics(&self, pils: *mut IMAGELISTSTATS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStatistics)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pils)).ok()
    }
    pub unsafe fn Initialize(&self, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(cx), ::core::mem::transmute(cy), ::core::mem::transmute(flags), ::core::mem::transmute(cinitial), ::core::mem::transmute(cgrow)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Replace2<'a, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>, Param2: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, i: i32, hbmimage: Param1, hbmmask: Param2, punk: Param3, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Replace2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), hbmimage.into_param().abi(), hbmmask.into_param().abi(), punk.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn ReplaceFromImageList<'a, Param1: ::windows_core::IntoParam<'a, IImageList>, Param3: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, i: i32, pil: Param1, isrc: i32, punk: Param3, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReplaceFromImageList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(i), pil.into_param().abi(), ::core::mem::transmute(isrc), punk.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<IImageList2> for ::windows_core::IUnknown {
    fn from(value: IImageList2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImageList2> for ::windows_core::IUnknown {
    fn from(value: &IImageList2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IImageList2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IImageList2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IImageList2> for IImageList {
    fn from(value: IImageList2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IImageList2> for IImageList {
    fn from(value: &IImageList2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageList> for IImageList2 {
    fn into_param(self) -> ::windows_core::Param<'a, IImageList> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IImageList> for &'a IImageList2 {
    fn into_param(self) -> ::windows_core::Param<'a, IImageList> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IImageList2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IImageList2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageList2 {}
impl ::core::fmt::Debug for IImageList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageList2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IImageList2 {
    type Vtable = IImageList2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x192b9d83_50fc_457b_90a0_2b82a8b5dae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageList2_Vtbl {
    pub base__: IImageList_Vtbl,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows_core::HRESULT,
    pub GetOriginalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows_core::HRESULT,
    pub SetOriginalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimage: i32, cx: i32, cy: i32) -> ::windows_core::HRESULT,
    pub SetCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ForceImagePresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iimage: i32, dwflags: u32) -> ::windows_core::HRESULT,
    pub DiscardImages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub PreloadImages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    PreloadImages: usize,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pils: *mut IMAGELISTSTATS) -> ::windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Replace2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, hbmimage: ::win32_graphics::Gdi::HBITMAP, hbmmask: ::win32_graphics::Gdi::HBITMAP, punk: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Replace2: usize,
    pub ReplaceFromImageList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, i: i32, pil: ::windows_core::RawPtr, isrc: i32, punk: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
}
pub const ILDI_PURGE: u32 = 1u32;
pub const ILDI_QUERYACCESS: u32 = 8u32;
pub const ILDI_RESETACCESS: u32 = 4u32;
pub const ILDI_STANDBY: u32 = 2u32;
pub const ILDRF_IMAGELOWQUALITY: u32 = 1u32;
pub const ILDRF_OVERLAYLOWQUALITY: u32 = 16u32;
pub const ILD_ASYNC: u32 = 32768u32;
pub const ILD_BLEND25: u32 = 2u32;
pub const ILD_DPISCALE: u32 = 16384u32;
pub const ILD_IMAGE: u32 = 32u32;
pub const ILD_OVERLAYMASK: u32 = 3840u32;
pub const ILD_PRESERVEALPHA: u32 = 4096u32;
pub const ILD_ROP: u32 = 64u32;
pub const ILD_SCALE: u32 = 8192u32;
pub const ILD_TRANSPARENT: u32 = 1u32;
pub const ILFIP_ALWAYS: u32 = 0u32;
pub const ILFIP_FROMSTANDBY: u32 = 1u32;
pub const ILGOS_ALWAYS: u32 = 0u32;
pub const ILGOS_FROMSTANDBY: u32 = 1u32;
pub const ILGT_ASYNC: u32 = 1u32;
pub const ILGT_NORMAL: u32 = 0u32;
pub const ILP_DOWNLEVEL: u32 = 1u32;
pub const ILP_NORMAL: u32 = 0u32;
pub const ILR_DEFAULT: u32 = 0u32;
pub const ILR_HORIZONTAL_CENTER: u32 = 1u32;
pub const ILR_HORIZONTAL_LEFT: u32 = 0u32;
pub const ILR_HORIZONTAL_RIGHT: u32 = 2u32;
pub const ILR_SCALE_ASPECTRATIO: u32 = 256u32;
pub const ILR_SCALE_CLIP: u32 = 0u32;
pub const ILR_VERTICAL_BOTTOM: u32 = 32u32;
pub const ILR_VERTICAL_CENTER: u32 = 16u32;
pub const ILR_VERTICAL_TOP: u32 = 0u32;
pub const ILS_ALPHA: u32 = 8u32;
pub const ILS_GLOW: u32 = 1u32;
pub const ILS_NORMAL: u32 = 0u32;
pub const ILS_SATURATE: u32 = 4u32;
pub const ILS_SHADOW: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct IMAGEINFO {
    pub hbmImage: ::win32_graphics::Gdi::HBITMAP,
    pub hbmMask: ::win32_graphics::Gdi::HBITMAP,
    pub Unused1: i32,
    pub Unused2: i32,
    pub rcImage: ::win32_foundation::RECT,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for IMAGEINFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for IMAGEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for IMAGEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGEINFO").field("hbmImage", &self.hbmImage).field("hbmMask", &self.hbmMask).field("Unused1", &self.Unused1).field("Unused2", &self.Unused2).field("rcImage", &self.rcImage).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for IMAGEINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for IMAGEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMAGEINFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for IMAGEINFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for IMAGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IMAGELAYOUT(pub i32);
pub const IL_VERTICAL: IMAGELAYOUT = IMAGELAYOUT(0i32);
pub const IL_HORIZONTAL: IMAGELAYOUT = IMAGELAYOUT(1i32);
impl ::core::marker::Copy for IMAGELAYOUT {}
impl ::core::clone::Clone for IMAGELAYOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGELAYOUT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IMAGELAYOUT {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAGELAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGELAYOUT").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct IMAGELISTDRAWPARAMS {
    pub cbSize: u32,
    pub himl: HIMAGELIST,
    pub i: i32,
    pub hdcDst: ::win32_graphics::Gdi::HDC,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub xBitmap: i32,
    pub yBitmap: i32,
    pub rgbBk: u32,
    pub rgbFg: u32,
    pub fStyle: u32,
    pub dwRop: u32,
    pub fState: u32,
    pub Frame: u32,
    pub crEffect: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for IMAGELISTDRAWPARAMS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for IMAGELISTDRAWPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for IMAGELISTDRAWPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGELISTDRAWPARAMS")
            .field("cbSize", &self.cbSize)
            .field("himl", &self.himl)
            .field("i", &self.i)
            .field("hdcDst", &self.hdcDst)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("cx", &self.cx)
            .field("cy", &self.cy)
            .field("xBitmap", &self.xBitmap)
            .field("yBitmap", &self.yBitmap)
            .field("rgbBk", &self.rgbBk)
            .field("rgbFg", &self.rgbFg)
            .field("fStyle", &self.fStyle)
            .field("dwRop", &self.dwRop)
            .field("fState", &self.fState)
            .field("Frame", &self.Frame)
            .field("crEffect", &self.crEffect)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for IMAGELISTDRAWPARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for IMAGELISTDRAWPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMAGELISTDRAWPARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for IMAGELISTDRAWPARAMS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for IMAGELISTDRAWPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IMAGELISTSTATS {
    pub cbSize: u32,
    pub cAlloc: i32,
    pub cUsed: i32,
    pub cStandby: i32,
}
impl ::core::marker::Copy for IMAGELISTSTATS {}
impl ::core::clone::Clone for IMAGELISTSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMAGELISTSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGELISTSTATS").field("cbSize", &self.cbSize).field("cAlloc", &self.cAlloc).field("cUsed", &self.cUsed).field("cStandby", &self.cStandby).finish()
    }
}
unsafe impl ::windows_core::Abi for IMAGELISTSTATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMAGELISTSTATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMAGELISTSTATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMAGELISTSTATS {}
impl ::core::default::Default for IMAGELISTSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IMAGELIST_CREATION_FLAGS(pub u32);
pub const ILC_MASK: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(1u32);
pub const ILC_COLOR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(0u32);
pub const ILC_COLORDDB: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(254u32);
pub const ILC_COLOR4: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(4u32);
pub const ILC_COLOR8: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(8u32);
pub const ILC_COLOR16: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(16u32);
pub const ILC_COLOR24: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(24u32);
pub const ILC_COLOR32: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(32u32);
pub const ILC_PALETTE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(2048u32);
pub const ILC_MIRROR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(8192u32);
pub const ILC_PERITEMMIRROR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(32768u32);
pub const ILC_ORIGINALSIZE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(65536u32);
pub const ILC_HIGHQUALITYSCALE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(131072u32);
impl ::core::marker::Copy for IMAGELIST_CREATION_FLAGS {}
impl ::core::clone::Clone for IMAGELIST_CREATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGELIST_CREATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IMAGELIST_CREATION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAGELIST_CREATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGELIST_CREATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IMAGELIST_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGELIST_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGELIST_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGELIST_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGELIST_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IMAGESELECTTYPE(pub i32);
pub const IST_NONE: IMAGESELECTTYPE = IMAGESELECTTYPE(0i32);
pub const IST_SIZE: IMAGESELECTTYPE = IMAGESELECTTYPE(1i32);
pub const IST_DPI: IMAGESELECTTYPE = IMAGESELECTTYPE(2i32);
impl ::core::marker::Copy for IMAGESELECTTYPE {}
impl ::core::clone::Clone for IMAGESELECTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGESELECTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IMAGESELECTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAGESELECTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGESELECTTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IMAGE_LIST_COPY_FLAGS(pub u32);
pub const ILCF_MOVE: IMAGE_LIST_COPY_FLAGS = IMAGE_LIST_COPY_FLAGS(0u32);
pub const ILCF_SWAP: IMAGE_LIST_COPY_FLAGS = IMAGE_LIST_COPY_FLAGS(1u32);
impl ::core::marker::Copy for IMAGE_LIST_COPY_FLAGS {}
impl ::core::clone::Clone for IMAGE_LIST_COPY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_LIST_COPY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IMAGE_LIST_COPY_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAGE_LIST_COPY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_LIST_COPY_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IMAGE_LIST_DRAW_STYLE(pub u32);
pub const ILD_BLEND: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
pub const ILD_BLEND50: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
pub const ILD_FOCUS: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(2u32);
pub const ILD_MASK: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(16u32);
pub const ILD_NORMAL: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(0u32);
pub const ILD_SELECTED: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
impl ::core::marker::Copy for IMAGE_LIST_DRAW_STYLE {}
impl ::core::clone::Clone for IMAGE_LIST_DRAW_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_LIST_DRAW_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IMAGE_LIST_DRAW_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAGE_LIST_DRAW_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_LIST_DRAW_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IMAGE_LIST_DRAW_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IMAGE_LIST_DRAW_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IMAGE_LIST_DRAW_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IMAGE_LIST_DRAW_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IMAGE_LIST_DRAW_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IMAGE_LIST_ITEM_FLAGS(pub u32);
pub const ILIF_ALPHA: IMAGE_LIST_ITEM_FLAGS = IMAGE_LIST_ITEM_FLAGS(1u32);
pub const ILIF_LOWQUALITY: IMAGE_LIST_ITEM_FLAGS = IMAGE_LIST_ITEM_FLAGS(2u32);
impl ::core::marker::Copy for IMAGE_LIST_ITEM_FLAGS {}
impl ::core::clone::Clone for IMAGE_LIST_ITEM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_LIST_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IMAGE_LIST_ITEM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAGE_LIST_ITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_LIST_ITEM_FLAGS").field(&self.0).finish()
    }
}
pub const INFOTIPSIZE: u32 = 1024u32;
#[repr(C)]
pub struct INITCOMMONCONTROLSEX {
    pub dwSize: u32,
    pub dwICC: INITCOMMONCONTROLSEX_ICC,
}
impl ::core::marker::Copy for INITCOMMONCONTROLSEX {}
impl ::core::clone::Clone for INITCOMMONCONTROLSEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INITCOMMONCONTROLSEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INITCOMMONCONTROLSEX").field("dwSize", &self.dwSize).field("dwICC", &self.dwICC).finish()
    }
}
unsafe impl ::windows_core::Abi for INITCOMMONCONTROLSEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INITCOMMONCONTROLSEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INITCOMMONCONTROLSEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for INITCOMMONCONTROLSEX {}
impl ::core::default::Default for INITCOMMONCONTROLSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INITCOMMONCONTROLSEX_ICC(pub u32);
pub const ICC_ANIMATE_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(128u32);
pub const ICC_BAR_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(4u32);
pub const ICC_COOL_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(1024u32);
pub const ICC_DATE_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(256u32);
pub const ICC_HOTKEY_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(64u32);
pub const ICC_INTERNET_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(2048u32);
pub const ICC_LINK_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(32768u32);
pub const ICC_LISTVIEW_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(1u32);
pub const ICC_NATIVEFNTCTL_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(8192u32);
pub const ICC_PAGESCROLLER_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(4096u32);
pub const ICC_PROGRESS_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(32u32);
pub const ICC_STANDARD_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(16384u32);
pub const ICC_TAB_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(8u32);
pub const ICC_TREEVIEW_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(2u32);
pub const ICC_UPDOWN_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(16u32);
pub const ICC_USEREX_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(512u32);
pub const ICC_WIN95_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(255u32);
impl ::core::marker::Copy for INITCOMMONCONTROLSEX_ICC {}
impl ::core::clone::Clone for INITCOMMONCONTROLSEX_ICC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INITCOMMONCONTROLSEX_ICC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for INITCOMMONCONTROLSEX_ICC {
    type Abi = Self;
}
impl ::core::fmt::Debug for INITCOMMONCONTROLSEX_ICC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INITCOMMONCONTROLSEX_ICC").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for INITCOMMONCONTROLSEX_ICC {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for INITCOMMONCONTROLSEX_ICC {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for INITCOMMONCONTROLSEX_ICC {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for INITCOMMONCONTROLSEX_ICC {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for INITCOMMONCONTROLSEX_ICC {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
pub struct INTLIST {
    pub iValueCount: i32,
    pub iValues: [i32; 402],
}
impl ::core::marker::Copy for INTLIST {}
impl ::core::clone::Clone for INTLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INTLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTLIST").field("iValueCount", &self.iValueCount).field("iValues", &self.iValues).finish()
    }
}
unsafe impl ::windows_core::Abi for INTLIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INTLIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INTLIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for INTLIST {}
impl ::core::default::Default for INTLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const INVALID_LINK_INDEX: i32 = -1i32;
pub const IPM_CLEARADDRESS: u32 = 1124u32;
pub const IPM_GETADDRESS: u32 = 1126u32;
pub const IPM_ISBLANK: u32 = 1129u32;
pub const IPM_SETADDRESS: u32 = 1125u32;
pub const IPM_SETFOCUS: u32 = 1128u32;
pub const IPM_SETRANGE: u32 = 1127u32;
pub const I_IMAGECALLBACK: i32 = -1i32;
pub const I_IMAGENONE: i32 = -2i32;
pub const I_INDENTCALLBACK: i32 = -1i32;
pub const ImageList: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c476ba2_02b1_48f4_8048_b24619ddc058);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_Add<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>, Param2: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>>(himl: Param0, hbmimage: Param1, hbmmask: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Add(himl: HIMAGELIST, hbmimage: ::win32_graphics::Gdi::HBITMAP, hbmmask: ::win32_graphics::Gdi::HBITMAP) -> i32;
        }
        ::core::mem::transmute(ImageList_Add(himl.into_param().abi(), hbmimage.into_param().abi(), hbmmask.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_AddMasked<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>, Param1: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>>(himl: Param0, hbmimage: Param1, crmask: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_AddMasked(himl: HIMAGELIST, hbmimage: ::win32_graphics::Gdi::HBITMAP, crmask: u32) -> i32;
        }
        ::core::mem::transmute(ImageList_AddMasked(himl.into_param().abi(), hbmimage.into_param().abi(), ::core::mem::transmute(crmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_BeginDrag<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himltrack: Param0, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_BeginDrag(himltrack: HIMAGELIST, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_BeginDrag(himltrack.into_param().abi(), ::core::mem::transmute(itrack), ::core::mem::transmute(dxhotspot), ::core::mem::transmute(dyhotspot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_CoCreateInstance<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, T: ::windows_core::Interface>(rclsid: *const ::windows_core::GUID, punkouter: Param1) -> ::windows_core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_CoCreateInstance(rclsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        ImageList_CoCreateInstance(::core::mem::transmute(rclsid), punkouter.into_param().abi(), &<T as ::windows_core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_Copy<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>, Param2: ::windows_core::IntoParam<'a, HIMAGELIST>>(himldst: Param0, idst: i32, himlsrc: Param2, isrc: i32, uflags: IMAGE_LIST_COPY_FLAGS) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Copy(himldst: HIMAGELIST, idst: i32, himlsrc: HIMAGELIST, isrc: i32, uflags: IMAGE_LIST_COPY_FLAGS) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_Copy(himldst.into_param().abi(), ::core::mem::transmute(idst), himlsrc.into_param().abi(), ::core::mem::transmute(isrc), ::core::mem::transmute(uflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_Create(cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Create(cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> HIMAGELIST;
        }
        ::core::mem::transmute(ImageList_Create(::core::mem::transmute(cx), ::core::mem::transmute(cy), ::core::mem::transmute(flags), ::core::mem::transmute(cinitial), ::core::mem::transmute(cgrow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_Destroy<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Destroy(himl: HIMAGELIST) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_Destroy(himl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_DragEnter<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwndlock: Param0, x: i32, y: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_DragEnter(hwndlock: ::win32_foundation::HWND, x: i32, y: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_DragEnter(hwndlock.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_DragLeave<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwndlock: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_DragLeave(hwndlock: ::win32_foundation::HWND) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_DragLeave(hwndlock.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_DragMove(x: i32, y: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_DragMove(x: i32, y: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_DragMove(::core::mem::transmute(x), ::core::mem::transmute(y)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_DragShowNolock<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(fshow: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_DragShowNolock(fshow: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_DragShowNolock(fshow.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_Draw<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>, Param2: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(himl: Param0, i: i32, hdcdst: Param2, x: i32, y: i32, fstyle: IMAGE_LIST_DRAW_STYLE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Draw(himl: HIMAGELIST, i: i32, hdcdst: ::win32_graphics::Gdi::HDC, x: i32, y: i32, fstyle: IMAGE_LIST_DRAW_STYLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_Draw(himl.into_param().abi(), ::core::mem::transmute(i), hdcdst.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(fstyle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_DrawEx<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>, Param2: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HDC>>(himl: Param0, i: i32, hdcdst: Param2, x: i32, y: i32, dx: i32, dy: i32, rgbbk: u32, rgbfg: u32, fstyle: IMAGE_LIST_DRAW_STYLE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_DrawEx(himl: HIMAGELIST, i: i32, hdcdst: ::win32_graphics::Gdi::HDC, x: i32, y: i32, dx: i32, dy: i32, rgbbk: u32, rgbfg: u32, fstyle: IMAGE_LIST_DRAW_STYLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_DrawEx(himl.into_param().abi(), ::core::mem::transmute(i), hdcdst.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(dx), ::core::mem::transmute(dy), ::core::mem::transmute(rgbbk), ::core::mem::transmute(rgbfg), ::core::mem::transmute(fstyle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_DrawIndirect(pimldp: *const IMAGELISTDRAWPARAMS) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_DrawIndirect(pimldp: *const IMAGELISTDRAWPARAMS) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_DrawIndirect(::core::mem::transmute(pimldp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_Duplicate<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Duplicate(himl: HIMAGELIST) -> HIMAGELIST;
        }
        ::core::mem::transmute(ImageList_Duplicate(himl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_EndDrag() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_EndDrag();
        }
        ImageList_EndDrag()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_GetBkColor<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_GetBkColor(himl: HIMAGELIST) -> u32;
        }
        ::core::mem::transmute(ImageList_GetBkColor(himl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_GetDragImage(ppt: *mut ::win32_foundation::POINT, ppthotspot: *mut ::win32_foundation::POINT) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_GetDragImage(ppt: *mut ::win32_foundation::POINT, ppthotspot: *mut ::win32_foundation::POINT) -> HIMAGELIST;
        }
        ::core::mem::transmute(ImageList_GetDragImage(::core::mem::transmute(ppt), ::core::mem::transmute(ppthotspot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ImageList_GetIcon<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0, i: i32, flags: u32) -> super::WindowsAndMessaging::HICON {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_GetIcon(himl: HIMAGELIST, i: i32, flags: u32) -> super::WindowsAndMessaging::HICON;
        }
        ::core::mem::transmute(ImageList_GetIcon(himl.into_param().abi(), ::core::mem::transmute(i), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_GetIconSize<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0, cx: *mut i32, cy: *mut i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_GetIconSize(himl: HIMAGELIST, cx: *mut i32, cy: *mut i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_GetIconSize(himl.into_param().abi(), ::core::mem::transmute(cx), ::core::mem::transmute(cy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_GetImageCount<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_GetImageCount(himl: HIMAGELIST) -> i32;
        }
        ::core::mem::transmute(ImageList_GetImageCount(himl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_GetImageInfo<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0, i: i32, pimageinfo: *mut IMAGEINFO) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_GetImageInfo(himl: HIMAGELIST, i: i32, pimageinfo: *mut IMAGEINFO) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_GetImageInfo(himl.into_param().abi(), ::core::mem::transmute(i), ::core::mem::transmute(pimageinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ImageList_LoadImageA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(hi: Param0, lpbmp: Param1, cx: i32, cgrow: i32, crmask: u32, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_LoadImageA(hi: ::win32_foundation::HINSTANCE, lpbmp: ::windows_core::PCSTR, cx: i32, cgrow: i32, crmask: u32, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST;
        }
        ::core::mem::transmute(ImageList_LoadImageA(hi.into_param().abi(), lpbmp.into_param().abi(), ::core::mem::transmute(cx), ::core::mem::transmute(cgrow), ::core::mem::transmute(crmask), ::core::mem::transmute(utype), ::core::mem::transmute(uflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ImageList_LoadImageW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hi: Param0, lpbmp: Param1, cx: i32, cgrow: i32, crmask: u32, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_LoadImageW(hi: ::win32_foundation::HINSTANCE, lpbmp: ::windows_core::PCWSTR, cx: i32, cgrow: i32, crmask: u32, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST;
        }
        ::core::mem::transmute(ImageList_LoadImageW(hi.into_param().abi(), lpbmp.into_param().abi(), ::core::mem::transmute(cx), ::core::mem::transmute(cgrow), ::core::mem::transmute(crmask), ::core::mem::transmute(utype), ::core::mem::transmute(uflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_Merge<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>, Param2: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl1: Param0, i1: i32, himl2: Param2, i2: i32, dx: i32, dy: i32) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Merge(himl1: HIMAGELIST, i1: i32, himl2: HIMAGELIST, i2: i32, dx: i32, dy: i32) -> HIMAGELIST;
        }
        ::core::mem::transmute(ImageList_Merge(himl1.into_param().abi(), ::core::mem::transmute(i1), himl2.into_param().abi(), ::core::mem::transmute(i2), ::core::mem::transmute(dx), ::core::mem::transmute(dy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_Read<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(pstm: Param0) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Read(pstm: ::windows_core::RawPtr) -> HIMAGELIST;
        }
        ::core::mem::transmute(ImageList_Read(pstm.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_ReadEx<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(dwflags: u32, pstm: Param1, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_ReadEx(dwflags: u32, pstm: ::windows_core::RawPtr, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ImageList_ReadEx(::core::mem::transmute(dwflags), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_Remove<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0, i: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Remove(himl: HIMAGELIST, i: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_Remove(himl.into_param().abi(), ::core::mem::transmute(i)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_Replace<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>, Param2: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>, Param3: ::windows_core::IntoParam<'a, ::win32_graphics::Gdi::HBITMAP>>(himl: Param0, i: i32, hbmimage: Param2, hbmmask: Param3) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Replace(himl: HIMAGELIST, i: i32, hbmimage: ::win32_graphics::Gdi::HBITMAP, hbmmask: ::win32_graphics::Gdi::HBITMAP) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_Replace(himl.into_param().abi(), ::core::mem::transmute(i), hbmimage.into_param().abi(), hbmmask.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ImageList_ReplaceIcon<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>, Param2: ::windows_core::IntoParam<'a, super::WindowsAndMessaging::HICON>>(himl: Param0, i: i32, hicon: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_ReplaceIcon(himl: HIMAGELIST, i: i32, hicon: super::WindowsAndMessaging::HICON) -> i32;
        }
        ::core::mem::transmute(ImageList_ReplaceIcon(himl.into_param().abi(), ::core::mem::transmute(i), hicon.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_SetBkColor<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0, clrbk: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_SetBkColor(himl: HIMAGELIST, clrbk: u32) -> u32;
        }
        ::core::mem::transmute(ImageList_SetBkColor(himl.into_param().abi(), ::core::mem::transmute(clrbk)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_SetDragCursorImage<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himldrag: Param0, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_SetDragCursorImage(himldrag: HIMAGELIST, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_SetDragCursorImage(himldrag.into_param().abi(), ::core::mem::transmute(idrag), ::core::mem::transmute(dxhotspot), ::core::mem::transmute(dyhotspot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_SetIconSize<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0, cx: i32, cy: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_SetIconSize(himl: HIMAGELIST, cx: i32, cy: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_SetIconSize(himl.into_param().abi(), ::core::mem::transmute(cx), ::core::mem::transmute(cy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_SetImageCount<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0, unewcount: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_SetImageCount(himl: HIMAGELIST, unewcount: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_SetImageCount(himl.into_param().abi(), ::core::mem::transmute(unewcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ImageList_SetOverlayImage<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>>(himl: Param0, iimage: i32, ioverlay: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_SetOverlayImage(himl: HIMAGELIST, iimage: i32, ioverlay: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_SetOverlayImage(himl.into_param().abi(), ::core::mem::transmute(iimage), ::core::mem::transmute(ioverlay)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_Write<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(himl: Param0, pstm: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Write(himl: HIMAGELIST, pstm: ::windows_core::RawPtr) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ImageList_Write(himl.into_param().abi(), pstm.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_WriteEx<'a, Param0: ::windows_core::IntoParam<'a, HIMAGELIST>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::IStream>>(himl: Param0, dwflags: u32, pstm: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_WriteEx(himl: HIMAGELIST, dwflags: u32, pstm: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        ImageList_WriteEx(himl.into_param().abi(), ::core::mem::transmute(dwflags), pstm.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn InitCommonControls() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitCommonControls();
        }
        InitCommonControls()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn InitCommonControlsEx(picce: *const INITCOMMONCONTROLSEX) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitCommonControlsEx(picce: *const INITCOMMONCONTROLSEX) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(InitCommonControlsEx(::core::mem::transmute(picce)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn InitMUILanguage(uilang: u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitMUILanguage(uilang: u16);
        }
        InitMUILanguage(::core::mem::transmute(uilang))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn InitializeFlatSB<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(param0: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeFlatSB(param0: ::win32_foundation::HWND) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(InitializeFlatSB(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsAppThemed() -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsAppThemed() -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(IsAppThemed())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsCharLowerW(ch: u16) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsCharLowerW(ch: u16) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(IsCharLowerW(::core::mem::transmute(ch)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsCompositionActive() -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsCompositionActive() -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(IsCompositionActive())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsDlgButtonChecked<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hdlg: Param0, nidbutton: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsDlgButtonChecked(hdlg: ::win32_foundation::HWND, nidbutton: i32) -> u32;
        }
        ::core::mem::transmute(IsDlgButtonChecked(hdlg.into_param().abi(), ::core::mem::transmute(nidbutton)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsThemeActive() -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsThemeActive() -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(IsThemeActive())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsThemeBackgroundPartiallyTransparent(htheme: isize, ipartid: i32, istateid: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsThemeBackgroundPartiallyTransparent(htheme: isize, ipartid: i32, istateid: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(IsThemeBackgroundPartiallyTransparent(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsThemeDialogTextureEnabled<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsThemeDialogTextureEnabled(hwnd: ::win32_foundation::HWND) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(IsThemeDialogTextureEnabled(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn IsThemePartDefined(htheme: isize, ipartid: i32, istateid: i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsThemePartDefined(htheme: isize, ipartid: i32, istateid: i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(IsThemePartDefined(::core::mem::transmute(htheme), ::core::mem::transmute(ipartid), ::core::mem::transmute(istateid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LBItemFromPt<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::POINT>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hlb: Param0, pt: Param1, bautoscroll: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LBItemFromPt(hlb: ::win32_foundation::HWND, pt: ::win32_foundation::POINT, bautoscroll: ::win32_foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(LBItemFromPt(hlb.into_param().abi(), pt.into_param().abi(), bautoscroll.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct LHITTESTINFO {
    pub pt: ::win32_foundation::POINT,
    pub item: LITEM,
}
impl ::core::marker::Copy for LHITTESTINFO {}
impl ::core::clone::Clone for LHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LHITTESTINFO").field("pt", &self.pt).field("item", &self.item).finish()
    }
}
unsafe impl ::windows_core::Abi for LHITTESTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LHITTESTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for LHITTESTINFO {}
impl ::core::default::Default for LHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const LIF_ITEMID: u32 = 4u32;
pub const LIF_ITEMINDEX: u32 = 1u32;
pub const LIF_STATE: u32 = 2u32;
pub const LIF_URL: u32 = 8u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LINKPARTS(pub i32);
pub const LP_HYPERLINK: LINKPARTS = LINKPARTS(1i32);
impl ::core::marker::Copy for LINKPARTS {}
impl ::core::clone::Clone for LINKPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LINKPARTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LINKPARTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for LINKPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LINKPARTS").field(&self.0).finish()
    }
}
pub const LIS_DEFAULTCOLORS: u32 = 16u32;
pub const LIS_ENABLED: u32 = 2u32;
pub const LIS_FOCUSED: u32 = 1u32;
pub const LIS_HOTTRACK: u32 = 8u32;
pub const LIS_VISITED: u32 = 4u32;
#[repr(C)]
pub struct LITEM {
    pub mask: u32,
    pub iLink: i32,
    pub state: u32,
    pub stateMask: u32,
    pub szID: [u16; 48],
    pub szUrl: [u16; 2084],
}
impl ::core::marker::Copy for LITEM {}
impl ::core::clone::Clone for LITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LITEM").field("mask", &self.mask).field("iLink", &self.iLink).field("state", &self.state).field("stateMask", &self.stateMask).field("szID", &self.szID).field("szUrl", &self.szUrl).finish()
    }
}
unsafe impl ::windows_core::Abi for LITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for LITEM {}
impl ::core::default::Default for LITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const LM_GETIDEALHEIGHT: u32 = 1793u32;
pub const LM_GETIDEALSIZE: u32 = 1793u32;
pub const LM_GETITEM: u32 = 1795u32;
pub const LM_HITTEST: u32 = 1792u32;
pub const LM_SETITEM: u32 = 1794u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LOGOFFBUTTONSSTATES(pub i32);
pub const SPLS_NORMAL: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(1i32);
pub const SPLS_HOT: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(2i32);
pub const SPLS_PRESSED: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(3i32);
impl ::core::marker::Copy for LOGOFFBUTTONSSTATES {}
impl ::core::clone::Clone for LOGOFFBUTTONSSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOGOFFBUTTONSSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LOGOFFBUTTONSSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for LOGOFFBUTTONSSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOGOFFBUTTONSSTATES").field(&self.0).finish()
    }
}
pub type LPFNADDPROPSHEETPAGES = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: LPFNSVADDPROPSHEETPAGE, param2: ::win32_foundation::LPARAM) -> ::win32_foundation::BOOL>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type LPFNCCINFOA = ::core::option::Option<unsafe extern "system" fn(acci: *mut CCINFOA) -> u32>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type LPFNCCINFOW = ::core::option::Option<unsafe extern "system" fn(acci: *mut CCINFOW) -> u32>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type LPFNCCSIZETOTEXTA = ::core::option::Option<unsafe extern "system" fn(flstyle: u32, flextstyle: u32, hfont: ::win32_graphics::Gdi::HFONT, psztext: ::windows_core::PCSTR) -> i32>;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type LPFNCCSIZETOTEXTW = ::core::option::Option<unsafe extern "system" fn(flstyle: u32, flextstyle: u32, hfont: ::win32_graphics::Gdi::HFONT, psztext: ::windows_core::PCWSTR) -> i32>;
pub type LPFNCCSTYLEA = ::core::option::Option<unsafe extern "system" fn(hwndparent: ::win32_foundation::HWND, pccs: *mut CCSTYLEA) -> ::win32_foundation::BOOL>;
pub type LPFNCCSTYLEW = ::core::option::Option<unsafe extern "system" fn(hwndparent: ::win32_foundation::HWND, pccs: *mut CCSTYLEW) -> ::win32_foundation::BOOL>;
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPFNPSPCALLBACKA = ::core::option::Option<unsafe extern "system" fn(hwnd: ::win32_foundation::HWND, umsg: PSPCB_MESSAGE, ppsp: *mut PROPSHEETPAGEA) -> u32>;
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPFNPSPCALLBACKW = ::core::option::Option<unsafe extern "system" fn(hwnd: ::win32_foundation::HWND, umsg: PSPCB_MESSAGE, ppsp: *mut PROPSHEETPAGEW) -> u32>;
pub type LPFNSVADDPROPSHEETPAGE = ::core::option::Option<unsafe extern "system" fn(param0: HPROPSHEETPAGE, param1: ::win32_foundation::LPARAM) -> ::win32_foundation::BOOL>;
pub const LVA_ALIGNLEFT: u32 = 1u32;
pub const LVA_ALIGNTOP: u32 = 2u32;
pub const LVA_DEFAULT: u32 = 0u32;
pub const LVA_SNAPTOGRID: u32 = 5u32;
pub const LVBKIF_FLAG_ALPHABLEND: u32 = 536870912u32;
pub const LVBKIF_FLAG_TILEOFFSET: u32 = 256u32;
pub const LVBKIF_SOURCE_HBITMAP: u32 = 1u32;
pub const LVBKIF_SOURCE_MASK: u32 = 3u32;
pub const LVBKIF_SOURCE_NONE: u32 = 0u32;
pub const LVBKIF_SOURCE_URL: u32 = 2u32;
pub const LVBKIF_STYLE_MASK: u32 = 16u32;
pub const LVBKIF_STYLE_NORMAL: u32 = 0u32;
pub const LVBKIF_STYLE_TILE: u32 = 16u32;
pub const LVBKIF_TYPE_WATERMARK: u32 = 268435456u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct LVBKIMAGEA {
    pub ulFlags: u32,
    pub hbm: ::win32_graphics::Gdi::HBITMAP,
    pub pszImage: ::windows_core::PSTR,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for LVBKIMAGEA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for LVBKIMAGEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for LVBKIMAGEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVBKIMAGEA").field("ulFlags", &self.ulFlags).field("hbm", &self.hbm).field("pszImage", &self.pszImage).field("cchImageMax", &self.cchImageMax).field("xOffsetPercent", &self.xOffsetPercent).field("yOffsetPercent", &self.yOffsetPercent).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for LVBKIMAGEA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for LVBKIMAGEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVBKIMAGEA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for LVBKIMAGEA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for LVBKIMAGEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct LVBKIMAGEW {
    pub ulFlags: u32,
    pub hbm: ::win32_graphics::Gdi::HBITMAP,
    pub pszImage: ::windows_core::PWSTR,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for LVBKIMAGEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for LVBKIMAGEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for LVBKIMAGEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVBKIMAGEW").field("ulFlags", &self.ulFlags).field("hbm", &self.hbm).field("pszImage", &self.pszImage).field("cchImageMax", &self.cchImageMax).field("xOffsetPercent", &self.xOffsetPercent).field("yOffsetPercent", &self.yOffsetPercent).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for LVBKIMAGEW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for LVBKIMAGEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVBKIMAGEW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for LVBKIMAGEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for LVBKIMAGEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const LVCDRF_NOGROUPFRAME: u32 = 131072u32;
pub const LVCDRF_NOSELECT: u32 = 65536u32;
pub const LVCFMT_FILL: u32 = 2097152u32;
pub const LVCFMT_LINE_BREAK: u32 = 1048576u32;
pub const LVCFMT_NO_TITLE: u32 = 8388608u32;
pub const LVCFMT_WRAP: u32 = 4194304u32;
#[repr(C)]
pub struct LVCOLUMNA {
    pub mask: LVCOLUMNW_MASK,
    pub fmt: LVCOLUMNW_FORMAT,
    pub cx: i32,
    pub pszText: ::windows_core::PSTR,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}
impl ::core::marker::Copy for LVCOLUMNA {}
impl ::core::clone::Clone for LVCOLUMNA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVCOLUMNA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVCOLUMNA").field("mask", &self.mask).field("fmt", &self.fmt).field("cx", &self.cx).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iSubItem", &self.iSubItem).field("iImage", &self.iImage).field("iOrder", &self.iOrder).field("cxMin", &self.cxMin).field("cxDefault", &self.cxDefault).field("cxIdeal", &self.cxIdeal).finish()
    }
}
unsafe impl ::windows_core::Abi for LVCOLUMNA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVCOLUMNA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVCOLUMNA>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVCOLUMNA {}
impl ::core::default::Default for LVCOLUMNA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct LVCOLUMNW {
    pub mask: LVCOLUMNW_MASK,
    pub fmt: LVCOLUMNW_FORMAT,
    pub cx: i32,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}
impl ::core::marker::Copy for LVCOLUMNW {}
impl ::core::clone::Clone for LVCOLUMNW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVCOLUMNW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVCOLUMNW").field("mask", &self.mask).field("fmt", &self.fmt).field("cx", &self.cx).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iSubItem", &self.iSubItem).field("iImage", &self.iImage).field("iOrder", &self.iOrder).field("cxMin", &self.cxMin).field("cxDefault", &self.cxDefault).field("cxIdeal", &self.cxIdeal).finish()
    }
}
unsafe impl ::windows_core::Abi for LVCOLUMNW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVCOLUMNW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVCOLUMNW>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVCOLUMNW {}
impl ::core::default::Default for LVCOLUMNW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LVCOLUMNW_FORMAT(pub u32);
pub const LVCFMT_LEFT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(0u32);
pub const LVCFMT_RIGHT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(1u32);
pub const LVCFMT_CENTER: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(2u32);
pub const LVCFMT_JUSTIFYMASK: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(3u32);
pub const LVCFMT_IMAGE: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(2048u32);
pub const LVCFMT_BITMAP_ON_RIGHT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(4096u32);
pub const LVCFMT_COL_HAS_IMAGES: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(32768u32);
pub const LVCFMT_FIXED_WIDTH: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(256u32);
pub const LVCFMT_NO_DPI_SCALE: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(262144u32);
pub const LVCFMT_FIXED_RATIO: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(524288u32);
pub const LVCFMT_SPLITBUTTON: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(16777216u32);
impl ::core::marker::Copy for LVCOLUMNW_FORMAT {}
impl ::core::clone::Clone for LVCOLUMNW_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LVCOLUMNW_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LVCOLUMNW_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for LVCOLUMNW_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVCOLUMNW_FORMAT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LVCOLUMNW_FORMAT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LVCOLUMNW_FORMAT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LVCOLUMNW_FORMAT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LVCOLUMNW_FORMAT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LVCOLUMNW_FORMAT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LVCOLUMNW_MASK(pub u32);
pub const LVCF_FMT: LVCOLUMNW_MASK = LVCOLUMNW_MASK(1u32);
pub const LVCF_WIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(2u32);
pub const LVCF_TEXT: LVCOLUMNW_MASK = LVCOLUMNW_MASK(4u32);
pub const LVCF_SUBITEM: LVCOLUMNW_MASK = LVCOLUMNW_MASK(8u32);
pub const LVCF_IMAGE: LVCOLUMNW_MASK = LVCOLUMNW_MASK(16u32);
pub const LVCF_ORDER: LVCOLUMNW_MASK = LVCOLUMNW_MASK(32u32);
pub const LVCF_MINWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(64u32);
pub const LVCF_DEFAULTWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(128u32);
pub const LVCF_IDEALWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(256u32);
impl ::core::marker::Copy for LVCOLUMNW_MASK {}
impl ::core::clone::Clone for LVCOLUMNW_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LVCOLUMNW_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LVCOLUMNW_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for LVCOLUMNW_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVCOLUMNW_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LVCOLUMNW_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LVCOLUMNW_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LVCOLUMNW_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LVCOLUMNW_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LVCOLUMNW_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LVFF_ITEMCOUNT: u32 = 1u32;
#[repr(C)]
pub struct LVFINDINFOA {
    pub flags: LVFINDINFOW_FLAGS,
    pub psz: ::windows_core::PCSTR,
    pub lParam: ::win32_foundation::LPARAM,
    pub pt: ::win32_foundation::POINT,
    pub vkDirection: u32,
}
impl ::core::marker::Copy for LVFINDINFOA {}
impl ::core::clone::Clone for LVFINDINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVFINDINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVFINDINFOA").field("flags", &self.flags).field("psz", &self.psz).field("lParam", &self.lParam).field("pt", &self.pt).field("vkDirection", &self.vkDirection).finish()
    }
}
unsafe impl ::windows_core::Abi for LVFINDINFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVFINDINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVFINDINFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVFINDINFOA {}
impl ::core::default::Default for LVFINDINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct LVFINDINFOW {
    pub flags: LVFINDINFOW_FLAGS,
    pub psz: ::windows_core::PCWSTR,
    pub lParam: ::win32_foundation::LPARAM,
    pub pt: ::win32_foundation::POINT,
    pub vkDirection: u32,
}
impl ::core::marker::Copy for LVFINDINFOW {}
impl ::core::clone::Clone for LVFINDINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVFINDINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVFINDINFOW").field("flags", &self.flags).field("psz", &self.psz).field("lParam", &self.lParam).field("pt", &self.pt).field("vkDirection", &self.vkDirection).finish()
    }
}
unsafe impl ::windows_core::Abi for LVFINDINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVFINDINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVFINDINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVFINDINFOW {}
impl ::core::default::Default for LVFINDINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LVFINDINFOW_FLAGS(pub u32);
pub const LVFI_PARAM: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(1u32);
pub const LVFI_PARTIAL: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(8u32);
pub const LVFI_STRING: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(2u32);
pub const LVFI_SUBSTRING: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(4u32);
pub const LVFI_WRAP: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(32u32);
pub const LVFI_NEARESTXY: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(64u32);
impl ::core::marker::Copy for LVFINDINFOW_FLAGS {}
impl ::core::clone::Clone for LVFINDINFOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LVFINDINFOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LVFINDINFOW_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for LVFINDINFOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVFINDINFOW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LVFINDINFOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LVFINDINFOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LVFINDINFOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LVFINDINFOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LVFINDINFOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LVFIS_FOCUSED: u32 = 1u32;
#[repr(C)]
pub struct LVFOOTERINFO {
    pub mask: u32,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub cItems: u32,
}
impl ::core::marker::Copy for LVFOOTERINFO {}
impl ::core::clone::Clone for LVFOOTERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVFOOTERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVFOOTERINFO").field("mask", &self.mask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("cItems", &self.cItems).finish()
    }
}
unsafe impl ::windows_core::Abi for LVFOOTERINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVFOOTERINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVFOOTERINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVFOOTERINFO {}
impl ::core::default::Default for LVFOOTERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct LVFOOTERITEM {
    pub mask: LVFOOTERITEM_MASK,
    pub iItem: i32,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub state: u32,
    pub stateMask: u32,
}
impl ::core::marker::Copy for LVFOOTERITEM {}
impl ::core::clone::Clone for LVFOOTERITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVFOOTERITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVFOOTERITEM").field("mask", &self.mask).field("iItem", &self.iItem).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("state", &self.state).field("stateMask", &self.stateMask).finish()
    }
}
unsafe impl ::windows_core::Abi for LVFOOTERITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVFOOTERITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVFOOTERITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVFOOTERITEM {}
impl ::core::default::Default for LVFOOTERITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LVFOOTERITEM_MASK(pub u32);
pub const LVFIF_TEXT: LVFOOTERITEM_MASK = LVFOOTERITEM_MASK(1u32);
pub const LVFIF_STATE: LVFOOTERITEM_MASK = LVFOOTERITEM_MASK(2u32);
impl ::core::marker::Copy for LVFOOTERITEM_MASK {}
impl ::core::clone::Clone for LVFOOTERITEM_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LVFOOTERITEM_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LVFOOTERITEM_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for LVFOOTERITEM_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVFOOTERITEM_MASK").field(&self.0).finish()
    }
}
pub const LVGA_FOOTER_CENTER: u32 = 16u32;
pub const LVGA_FOOTER_LEFT: u32 = 8u32;
pub const LVGA_FOOTER_RIGHT: u32 = 32u32;
pub const LVGF_ALIGN: u32 = 8u32;
pub const LVGF_DESCRIPTIONBOTTOM: u32 = 2048u32;
pub const LVGF_DESCRIPTIONTOP: u32 = 1024u32;
pub const LVGF_EXTENDEDIMAGE: u32 = 8192u32;
pub const LVGF_GROUPID: u32 = 16u32;
pub const LVGF_ITEMS: u32 = 16384u32;
pub const LVGF_SUBSET: u32 = 32768u32;
pub const LVGF_SUBSETITEMS: u32 = 65536u32;
pub const LVGF_SUBTITLE: u32 = 256u32;
pub const LVGF_TASK: u32 = 512u32;
pub const LVGF_TITLEIMAGE: u32 = 4096u32;
pub const LVGGR_GROUP: u32 = 0u32;
pub const LVGGR_HEADER: u32 = 1u32;
pub const LVGGR_LABEL: u32 = 2u32;
pub const LVGGR_SUBSETLINK: u32 = 3u32;
pub const LVGIT_UNFOLDED: u32 = 1u32;
pub const LVGMF_BORDERCOLOR: u32 = 2u32;
pub const LVGMF_BORDERSIZE: u32 = 1u32;
pub const LVGMF_NONE: u32 = 0u32;
pub const LVGMF_TEXTCOLOR: u32 = 4u32;
#[repr(C)]
pub struct LVGROUP {
    pub cbSize: u32,
    pub mask: LVGROUP_MASK,
    pub pszHeader: ::windows_core::PWSTR,
    pub cchHeader: i32,
    pub pszFooter: ::windows_core::PWSTR,
    pub cchFooter: i32,
    pub iGroupId: i32,
    pub stateMask: u32,
    pub state: u32,
    pub uAlign: u32,
    pub pszSubtitle: ::windows_core::PWSTR,
    pub cchSubtitle: u32,
    pub pszTask: ::windows_core::PWSTR,
    pub cchTask: u32,
    pub pszDescriptionTop: ::windows_core::PWSTR,
    pub cchDescriptionTop: u32,
    pub pszDescriptionBottom: ::windows_core::PWSTR,
    pub cchDescriptionBottom: u32,
    pub iTitleImage: i32,
    pub iExtendedImage: i32,
    pub iFirstItem: i32,
    pub cItems: u32,
    pub pszSubsetTitle: ::windows_core::PWSTR,
    pub cchSubsetTitle: u32,
}
impl ::core::marker::Copy for LVGROUP {}
impl ::core::clone::Clone for LVGROUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVGROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVGROUP")
            .field("cbSize", &self.cbSize)
            .field("mask", &self.mask)
            .field("pszHeader", &self.pszHeader)
            .field("cchHeader", &self.cchHeader)
            .field("pszFooter", &self.pszFooter)
            .field("cchFooter", &self.cchFooter)
            .field("iGroupId", &self.iGroupId)
            .field("stateMask", &self.stateMask)
            .field("state", &self.state)
            .field("uAlign", &self.uAlign)
            .field("pszSubtitle", &self.pszSubtitle)
            .field("cchSubtitle", &self.cchSubtitle)
            .field("pszTask", &self.pszTask)
            .field("cchTask", &self.cchTask)
            .field("pszDescriptionTop", &self.pszDescriptionTop)
            .field("cchDescriptionTop", &self.cchDescriptionTop)
            .field("pszDescriptionBottom", &self.pszDescriptionBottom)
            .field("cchDescriptionBottom", &self.cchDescriptionBottom)
            .field("iTitleImage", &self.iTitleImage)
            .field("iExtendedImage", &self.iExtendedImage)
            .field("iFirstItem", &self.iFirstItem)
            .field("cItems", &self.cItems)
            .field("pszSubsetTitle", &self.pszSubsetTitle)
            .field("cchSubsetTitle", &self.cchSubsetTitle)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for LVGROUP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVGROUP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVGROUP>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVGROUP {}
impl ::core::default::Default for LVGROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct LVGROUPMETRICS {
    pub cbSize: u32,
    pub mask: u32,
    pub Left: u32,
    pub Top: u32,
    pub Right: u32,
    pub Bottom: u32,
    pub crLeft: u32,
    pub crTop: u32,
    pub crRight: u32,
    pub crBottom: u32,
    pub crHeader: u32,
    pub crFooter: u32,
}
impl ::core::marker::Copy for LVGROUPMETRICS {}
impl ::core::clone::Clone for LVGROUPMETRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVGROUPMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVGROUPMETRICS").field("cbSize", &self.cbSize).field("mask", &self.mask).field("Left", &self.Left).field("Top", &self.Top).field("Right", &self.Right).field("Bottom", &self.Bottom).field("crLeft", &self.crLeft).field("crTop", &self.crTop).field("crRight", &self.crRight).field("crBottom", &self.crBottom).field("crHeader", &self.crHeader).field("crFooter", &self.crFooter).finish()
    }
}
unsafe impl ::windows_core::Abi for LVGROUPMETRICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVGROUPMETRICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVGROUPMETRICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVGROUPMETRICS {}
impl ::core::default::Default for LVGROUPMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LVGROUP_MASK(pub u32);
pub const LVGF_NONE: LVGROUP_MASK = LVGROUP_MASK(0u32);
pub const LVGF_HEADER: LVGROUP_MASK = LVGROUP_MASK(1u32);
pub const LVGF_FOOTER: LVGROUP_MASK = LVGROUP_MASK(2u32);
pub const LVGF_STATE: LVGROUP_MASK = LVGROUP_MASK(4u32);
impl ::core::marker::Copy for LVGROUP_MASK {}
impl ::core::clone::Clone for LVGROUP_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LVGROUP_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LVGROUP_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for LVGROUP_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVGROUP_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LVGROUP_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LVGROUP_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LVGROUP_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LVGROUP_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LVGROUP_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LVGS_COLLAPSED: u32 = 1u32;
pub const LVGS_COLLAPSIBLE: u32 = 8u32;
pub const LVGS_FOCUSED: u32 = 16u32;
pub const LVGS_HIDDEN: u32 = 2u32;
pub const LVGS_NOHEADER: u32 = 4u32;
pub const LVGS_NORMAL: u32 = 0u32;
pub const LVGS_SELECTED: u32 = 32u32;
pub const LVGS_SUBSETED: u32 = 64u32;
pub const LVGS_SUBSETLINKFOCUSED: u32 = 128u32;
#[repr(C)]
pub struct LVHITTESTINFO {
    pub pt: ::win32_foundation::POINT,
    pub flags: LVHITTESTINFO_FLAGS,
    pub iItem: i32,
    pub iSubItem: i32,
    pub iGroup: i32,
}
impl ::core::marker::Copy for LVHITTESTINFO {}
impl ::core::clone::Clone for LVHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("iGroup", &self.iGroup).finish()
    }
}
unsafe impl ::windows_core::Abi for LVHITTESTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVHITTESTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVHITTESTINFO {}
impl ::core::default::Default for LVHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LVHITTESTINFO_FLAGS(pub u32);
pub const LVHT_ABOVE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(8u32);
pub const LVHT_BELOW: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(16u32);
pub const LVHT_NOWHERE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(1u32);
pub const LVHT_ONITEMICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(2u32);
pub const LVHT_ONITEMLABEL: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(4u32);
pub const LVHT_ONITEMSTATEICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(8u32);
pub const LVHT_TOLEFT: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(64u32);
pub const LVHT_TORIGHT: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(32u32);
pub const LVHT_EX_GROUP_HEADER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(268435456u32);
pub const LVHT_EX_GROUP_FOOTER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(536870912u32);
pub const LVHT_EX_GROUP_COLLAPSE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(1073741824u32);
pub const LVHT_EX_GROUP_BACKGROUND: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(2147483648u32);
pub const LVHT_EX_GROUP_STATEICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(16777216u32);
pub const LVHT_EX_GROUP_SUBSETLINK: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(33554432u32);
pub const LVHT_EX_GROUP: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(4076863488u32);
pub const LVHT_EX_ONCONTENTS: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(67108864u32);
pub const LVHT_EX_FOOTER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(134217728u32);
impl ::core::marker::Copy for LVHITTESTINFO_FLAGS {}
impl ::core::clone::Clone for LVHITTESTINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LVHITTESTINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LVHITTESTINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for LVHITTESTINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVHITTESTINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LVHITTESTINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LVHITTESTINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LVHITTESTINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LVIF_COLFMT: u32 = 65536u32;
pub const LVIF_COLUMNS: u32 = 512u32;
pub const LVIF_DI_SETITEM: u32 = 4096u32;
pub const LVIF_GROUPID: u32 = 256u32;
pub const LVIF_IMAGE: u32 = 2u32;
pub const LVIF_INDENT: u32 = 16u32;
pub const LVIF_NORECOMPUTE: u32 = 2048u32;
pub const LVIF_PARAM: u32 = 4u32;
pub const LVIF_STATE: u32 = 8u32;
pub const LVIF_TEXT: u32 = 1u32;
pub const LVIM_AFTER: u32 = 1u32;
#[repr(C)]
pub struct LVINSERTGROUPSORTED {
    pub pfnGroupCompare: PFNLVGROUPCOMPARE,
    pub pvData: *mut ::core::ffi::c_void,
    pub lvGroup: LVGROUP,
}
impl ::core::marker::Copy for LVINSERTGROUPSORTED {}
impl ::core::clone::Clone for LVINSERTGROUPSORTED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVINSERTGROUPSORTED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVINSERTGROUPSORTED").field("pfnGroupCompare", &self.pfnGroupCompare.map(|f| f as usize)).field("pvData", &self.pvData).field("lvGroup", &self.lvGroup).finish()
    }
}
unsafe impl ::windows_core::Abi for LVINSERTGROUPSORTED {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVINSERTGROUPSORTED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVINSERTGROUPSORTED>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVINSERTGROUPSORTED {}
impl ::core::default::Default for LVINSERTGROUPSORTED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct LVINSERTMARK {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iItem: i32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for LVINSERTMARK {}
impl ::core::clone::Clone for LVINSERTMARK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVINSERTMARK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVINSERTMARK").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("iItem", &self.iItem).field("dwReserved", &self.dwReserved).finish()
    }
}
unsafe impl ::windows_core::Abi for LVINSERTMARK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVINSERTMARK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVINSERTMARK>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVINSERTMARK {}
impl ::core::default::Default for LVINSERTMARK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const LVIR_BOUNDS: u32 = 0u32;
pub const LVIR_ICON: u32 = 1u32;
pub const LVIR_LABEL: u32 = 2u32;
pub const LVIR_SELECTBOUNDS: u32 = 3u32;
pub const LVIS_ACTIVATING: u32 = 32u32;
pub const LVIS_CUT: u32 = 4u32;
pub const LVIS_DROPHILITED: u32 = 8u32;
pub const LVIS_FOCUSED: u32 = 1u32;
pub const LVIS_GLOW: u32 = 16u32;
pub const LVIS_OVERLAYMASK: u32 = 3840u32;
pub const LVIS_SELECTED: u32 = 2u32;
pub const LVIS_STATEIMAGEMASK: u32 = 61440u32;
#[repr(C)]
pub struct LVITEMA {
    pub mask: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: ::windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: ::win32_foundation::LPARAM,
    pub iIndent: i32,
    pub iGroupId: LVITEMA_GROUP_ID,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut i32,
    pub iGroup: i32,
}
impl ::core::marker::Copy for LVITEMA {}
impl ::core::clone::Clone for LVITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVITEMA")
            .field("mask", &self.mask)
            .field("iItem", &self.iItem)
            .field("iSubItem", &self.iSubItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("lParam", &self.lParam)
            .field("iIndent", &self.iIndent)
            .field("iGroupId", &self.iGroupId)
            .field("cColumns", &self.cColumns)
            .field("puColumns", &self.puColumns)
            .field("piColFmt", &self.piColFmt)
            .field("iGroup", &self.iGroup)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for LVITEMA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVITEMA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVITEMA>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVITEMA {}
impl ::core::default::Default for LVITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LVITEMA_GROUP_ID(pub i32);
pub const I_GROUPIDCALLBACK: LVITEMA_GROUP_ID = LVITEMA_GROUP_ID(-1i32);
pub const I_GROUPIDNONE: LVITEMA_GROUP_ID = LVITEMA_GROUP_ID(-2i32);
impl ::core::marker::Copy for LVITEMA_GROUP_ID {}
impl ::core::clone::Clone for LVITEMA_GROUP_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LVITEMA_GROUP_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LVITEMA_GROUP_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for LVITEMA_GROUP_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVITEMA_GROUP_ID").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct LVITEMINDEX {
    pub iItem: i32,
    pub iGroup: i32,
}
impl ::core::marker::Copy for LVITEMINDEX {}
impl ::core::clone::Clone for LVITEMINDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVITEMINDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVITEMINDEX").field("iItem", &self.iItem).field("iGroup", &self.iGroup).finish()
    }
}
unsafe impl ::windows_core::Abi for LVITEMINDEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVITEMINDEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVITEMINDEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVITEMINDEX {}
impl ::core::default::Default for LVITEMINDEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct LVITEMW {
    pub mask: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: ::win32_foundation::LPARAM,
    pub iIndent: i32,
    pub iGroupId: LVITEMA_GROUP_ID,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut i32,
    pub iGroup: i32,
}
impl ::core::marker::Copy for LVITEMW {}
impl ::core::clone::Clone for LVITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVITEMW")
            .field("mask", &self.mask)
            .field("iItem", &self.iItem)
            .field("iSubItem", &self.iSubItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("lParam", &self.lParam)
            .field("iIndent", &self.iIndent)
            .field("iGroupId", &self.iGroupId)
            .field("cColumns", &self.cColumns)
            .field("puColumns", &self.puColumns)
            .field("piColFmt", &self.piColFmt)
            .field("iGroup", &self.iGroup)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for LVITEMW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVITEMW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVITEMW>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVITEMW {}
impl ::core::default::Default for LVITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const LVKF_ALT: u32 = 1u32;
pub const LVKF_CONTROL: u32 = 2u32;
pub const LVKF_SHIFT: u32 = 4u32;
pub const LVM_APPROXIMATEVIEWRECT: u32 = 4160u32;
pub const LVM_ARRANGE: u32 = 4118u32;
pub const LVM_CANCELEDITLABEL: u32 = 4275u32;
pub const LVM_CREATEDRAGIMAGE: u32 = 4129u32;
pub const LVM_DELETEALLITEMS: u32 = 4105u32;
pub const LVM_DELETECOLUMN: u32 = 4124u32;
pub const LVM_DELETEITEM: u32 = 4104u32;
pub const LVM_EDITLABEL: u32 = 4214u32;
pub const LVM_EDITLABELA: u32 = 4119u32;
pub const LVM_EDITLABELW: u32 = 4214u32;
pub const LVM_ENABLEGROUPVIEW: u32 = 4253u32;
pub const LVM_ENSUREVISIBLE: u32 = 4115u32;
pub const LVM_FINDITEM: u32 = 4179u32;
pub const LVM_FINDITEMA: u32 = 4109u32;
pub const LVM_FINDITEMW: u32 = 4179u32;
pub const LVM_FIRST: u32 = 4096u32;
pub const LVM_GETBKCOLOR: u32 = 4096u32;
pub const LVM_GETBKIMAGE: u32 = 4235u32;
pub const LVM_GETBKIMAGEA: u32 = 4165u32;
pub const LVM_GETBKIMAGEW: u32 = 4235u32;
pub const LVM_GETCALLBACKMASK: u32 = 4106u32;
pub const LVM_GETCOLUMN: u32 = 4191u32;
pub const LVM_GETCOLUMNA: u32 = 4121u32;
pub const LVM_GETCOLUMNORDERARRAY: u32 = 4155u32;
pub const LVM_GETCOLUMNW: u32 = 4191u32;
pub const LVM_GETCOLUMNWIDTH: u32 = 4125u32;
pub const LVM_GETCOUNTPERPAGE: u32 = 4136u32;
pub const LVM_GETEDITCONTROL: u32 = 4120u32;
pub const LVM_GETEMPTYTEXT: u32 = 4300u32;
pub const LVM_GETEXTENDEDLISTVIEWSTYLE: u32 = 4151u32;
pub const LVM_GETFOCUSEDGROUP: u32 = 4189u32;
pub const LVM_GETFOOTERINFO: u32 = 4302u32;
pub const LVM_GETFOOTERITEM: u32 = 4304u32;
pub const LVM_GETFOOTERITEMRECT: u32 = 4303u32;
pub const LVM_GETFOOTERRECT: u32 = 4301u32;
pub const LVM_GETGROUPCOUNT: u32 = 4248u32;
pub const LVM_GETGROUPINFO: u32 = 4245u32;
pub const LVM_GETGROUPINFOBYINDEX: u32 = 4249u32;
pub const LVM_GETGROUPMETRICS: u32 = 4252u32;
pub const LVM_GETGROUPRECT: u32 = 4194u32;
pub const LVM_GETGROUPSTATE: u32 = 4188u32;
pub const LVM_GETHEADER: u32 = 4127u32;
pub const LVM_GETHOTCURSOR: u32 = 4159u32;
pub const LVM_GETHOTITEM: u32 = 4157u32;
pub const LVM_GETHOVERTIME: u32 = 4168u32;
pub const LVM_GETIMAGELIST: u32 = 4098u32;
pub const LVM_GETINSERTMARK: u32 = 4263u32;
pub const LVM_GETINSERTMARKCOLOR: u32 = 4267u32;
pub const LVM_GETINSERTMARKRECT: u32 = 4265u32;
pub const LVM_GETISEARCHSTRING: u32 = 4213u32;
pub const LVM_GETISEARCHSTRINGA: u32 = 4148u32;
pub const LVM_GETISEARCHSTRINGW: u32 = 4213u32;
pub const LVM_GETITEM: u32 = 4171u32;
pub const LVM_GETITEMA: u32 = 4101u32;
pub const LVM_GETITEMCOUNT: u32 = 4100u32;
pub const LVM_GETITEMINDEXRECT: u32 = 4305u32;
pub const LVM_GETITEMPOSITION: u32 = 4112u32;
pub const LVM_GETITEMRECT: u32 = 4110u32;
pub const LVM_GETITEMSPACING: u32 = 4147u32;
pub const LVM_GETITEMSTATE: u32 = 4140u32;
pub const LVM_GETITEMTEXT: u32 = 4211u32;
pub const LVM_GETITEMTEXTA: u32 = 4141u32;
pub const LVM_GETITEMTEXTW: u32 = 4211u32;
pub const LVM_GETITEMW: u32 = 4171u32;
pub const LVM_GETNEXTITEM: u32 = 4108u32;
pub const LVM_GETNEXTITEMINDEX: u32 = 4307u32;
pub const LVM_GETNUMBEROFWORKAREAS: u32 = 4169u32;
pub const LVM_GETORIGIN: u32 = 4137u32;
pub const LVM_GETOUTLINECOLOR: u32 = 4272u32;
pub const LVM_GETSELECTEDCOLUMN: u32 = 4270u32;
pub const LVM_GETSELECTEDCOUNT: u32 = 4146u32;
pub const LVM_GETSELECTIONMARK: u32 = 4162u32;
pub const LVM_GETSTRINGWIDTH: u32 = 4183u32;
pub const LVM_GETSTRINGWIDTHA: u32 = 4113u32;
pub const LVM_GETSTRINGWIDTHW: u32 = 4183u32;
pub const LVM_GETSUBITEMRECT: u32 = 4152u32;
pub const LVM_GETTEXTBKCOLOR: u32 = 4133u32;
pub const LVM_GETTEXTCOLOR: u32 = 4131u32;
pub const LVM_GETTILEINFO: u32 = 4261u32;
pub const LVM_GETTILEVIEWINFO: u32 = 4259u32;
pub const LVM_GETTOOLTIPS: u32 = 4174u32;
pub const LVM_GETTOPINDEX: u32 = 4135u32;
pub const LVM_GETUNICODEFORMAT: u32 = 8198u32;
pub const LVM_GETVIEW: u32 = 4239u32;
pub const LVM_GETVIEWRECT: u32 = 4130u32;
pub const LVM_GETWORKAREAS: u32 = 4166u32;
pub const LVM_HASGROUP: u32 = 4257u32;
pub const LVM_HITTEST: u32 = 4114u32;
pub const LVM_INSERTCOLUMN: u32 = 4193u32;
pub const LVM_INSERTCOLUMNA: u32 = 4123u32;
pub const LVM_INSERTCOLUMNW: u32 = 4193u32;
pub const LVM_INSERTGROUP: u32 = 4241u32;
pub const LVM_INSERTGROUPSORTED: u32 = 4255u32;
pub const LVM_INSERTITEM: u32 = 4173u32;
pub const LVM_INSERTITEMA: u32 = 4103u32;
pub const LVM_INSERTITEMW: u32 = 4173u32;
pub const LVM_INSERTMARKHITTEST: u32 = 4264u32;
pub const LVM_ISGROUPVIEWENABLED: u32 = 4271u32;
pub const LVM_ISITEMVISIBLE: u32 = 4278u32;
pub const LVM_MAPIDTOINDEX: u32 = 4277u32;
pub const LVM_MAPINDEXTOID: u32 = 4276u32;
pub const LVM_MOVEGROUP: u32 = 4247u32;
pub const LVM_MOVEITEMTOGROUP: u32 = 4250u32;
pub const LVM_REDRAWITEMS: u32 = 4117u32;
pub const LVM_REMOVEALLGROUPS: u32 = 4256u32;
pub const LVM_REMOVEGROUP: u32 = 4246u32;
pub const LVM_SCROLL: u32 = 4116u32;
pub const LVM_SETBKCOLOR: u32 = 4097u32;
pub const LVM_SETBKIMAGE: u32 = 4234u32;
pub const LVM_SETBKIMAGEA: u32 = 4164u32;
pub const LVM_SETBKIMAGEW: u32 = 4234u32;
pub const LVM_SETCALLBACKMASK: u32 = 4107u32;
pub const LVM_SETCOLUMN: u32 = 4192u32;
pub const LVM_SETCOLUMNA: u32 = 4122u32;
pub const LVM_SETCOLUMNORDERARRAY: u32 = 4154u32;
pub const LVM_SETCOLUMNW: u32 = 4192u32;
pub const LVM_SETCOLUMNWIDTH: u32 = 4126u32;
pub const LVM_SETEXTENDEDLISTVIEWSTYLE: u32 = 4150u32;
pub const LVM_SETGROUPINFO: u32 = 4243u32;
pub const LVM_SETGROUPMETRICS: u32 = 4251u32;
pub const LVM_SETHOTCURSOR: u32 = 4158u32;
pub const LVM_SETHOTITEM: u32 = 4156u32;
pub const LVM_SETHOVERTIME: u32 = 4167u32;
pub const LVM_SETICONSPACING: u32 = 4149u32;
pub const LVM_SETIMAGELIST: u32 = 4099u32;
pub const LVM_SETINFOTIP: u32 = 4269u32;
pub const LVM_SETINSERTMARK: u32 = 4262u32;
pub const LVM_SETINSERTMARKCOLOR: u32 = 4266u32;
pub const LVM_SETITEM: u32 = 4172u32;
pub const LVM_SETITEMA: u32 = 4102u32;
pub const LVM_SETITEMCOUNT: u32 = 4143u32;
pub const LVM_SETITEMINDEXSTATE: u32 = 4306u32;
pub const LVM_SETITEMPOSITION: u32 = 4111u32;
pub const LVM_SETITEMPOSITION32: u32 = 4145u32;
pub const LVM_SETITEMSTATE: u32 = 4139u32;
pub const LVM_SETITEMTEXT: u32 = 4212u32;
pub const LVM_SETITEMTEXTA: u32 = 4142u32;
pub const LVM_SETITEMTEXTW: u32 = 4212u32;
pub const LVM_SETITEMW: u32 = 4172u32;
pub const LVM_SETOUTLINECOLOR: u32 = 4273u32;
pub const LVM_SETSELECTEDCOLUMN: u32 = 4236u32;
pub const LVM_SETSELECTIONMARK: u32 = 4163u32;
pub const LVM_SETTEXTBKCOLOR: u32 = 4134u32;
pub const LVM_SETTEXTCOLOR: u32 = 4132u32;
pub const LVM_SETTILEINFO: u32 = 4260u32;
pub const LVM_SETTILEVIEWINFO: u32 = 4258u32;
pub const LVM_SETTOOLTIPS: u32 = 4170u32;
pub const LVM_SETUNICODEFORMAT: u32 = 8197u32;
pub const LVM_SETVIEW: u32 = 4238u32;
pub const LVM_SETWORKAREAS: u32 = 4161u32;
pub const LVM_SORTGROUPS: u32 = 4254u32;
pub const LVM_SORTITEMS: u32 = 4144u32;
pub const LVM_SORTITEMSEX: u32 = 4177u32;
pub const LVM_SUBITEMHITTEST: u32 = 4153u32;
pub const LVM_UPDATE: u32 = 4138u32;
pub const LVNI_ABOVE: u32 = 256u32;
pub const LVNI_ALL: u32 = 0u32;
pub const LVNI_BELOW: u32 = 512u32;
pub const LVNI_CUT: u32 = 4u32;
pub const LVNI_DROPHILITED: u32 = 8u32;
pub const LVNI_FOCUSED: u32 = 1u32;
pub const LVNI_PREVIOUS: u32 = 32u32;
pub const LVNI_SAMEGROUPONLY: u32 = 128u32;
pub const LVNI_SELECTED: u32 = 2u32;
pub const LVNI_TOLEFT: u32 = 1024u32;
pub const LVNI_TORIGHT: u32 = 2048u32;
pub const LVNI_VISIBLEONLY: u32 = 64u32;
pub const LVNI_VISIBLEORDER: u32 = 16u32;
pub const LVNSCH_DEFAULT: i32 = -1i32;
pub const LVNSCH_ERROR: i32 = -2i32;
pub const LVNSCH_IGNORE: i32 = -3i32;
pub const LVSCW_AUTOSIZE: i32 = -1i32;
pub const LVSCW_AUTOSIZE_USEHEADER: i32 = -2i32;
#[repr(C)]
pub struct LVSETINFOTIP {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pszText: ::windows_core::PWSTR,
    pub iItem: i32,
    pub iSubItem: i32,
}
impl ::core::marker::Copy for LVSETINFOTIP {}
impl ::core::clone::Clone for LVSETINFOTIP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVSETINFOTIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVSETINFOTIP").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("pszText", &self.pszText).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).finish()
    }
}
unsafe impl ::windows_core::Abi for LVSETINFOTIP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVSETINFOTIP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVSETINFOTIP>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVSETINFOTIP {}
impl ::core::default::Default for LVSETINFOTIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const LVSICF_NOINVALIDATEALL: u32 = 1u32;
pub const LVSICF_NOSCROLL: u32 = 2u32;
pub const LVSIL_GROUPHEADER: u32 = 3u32;
pub const LVSIL_NORMAL: u32 = 0u32;
pub const LVSIL_SMALL: u32 = 1u32;
pub const LVSIL_STATE: u32 = 2u32;
pub const LVS_ALIGNLEFT: u32 = 2048u32;
pub const LVS_ALIGNMASK: u32 = 3072u32;
pub const LVS_ALIGNTOP: u32 = 0u32;
pub const LVS_AUTOARRANGE: u32 = 256u32;
pub const LVS_EDITLABELS: u32 = 512u32;
pub const LVS_EX_AUTOAUTOARRANGE: u32 = 16777216u32;
pub const LVS_EX_AUTOCHECKSELECT: u32 = 134217728u32;
pub const LVS_EX_AUTOSIZECOLUMNS: u32 = 268435456u32;
pub const LVS_EX_BORDERSELECT: u32 = 32768u32;
pub const LVS_EX_CHECKBOXES: u32 = 4u32;
pub const LVS_EX_COLUMNOVERFLOW: u32 = 2147483648u32;
pub const LVS_EX_COLUMNSNAPPOINTS: u32 = 1073741824u32;
pub const LVS_EX_DOUBLEBUFFER: u32 = 65536u32;
pub const LVS_EX_FLATSB: u32 = 256u32;
pub const LVS_EX_FULLROWSELECT: u32 = 32u32;
pub const LVS_EX_GRIDLINES: u32 = 1u32;
pub const LVS_EX_HEADERDRAGDROP: u32 = 16u32;
pub const LVS_EX_HEADERINALLVIEWS: u32 = 33554432u32;
pub const LVS_EX_HIDELABELS: u32 = 131072u32;
pub const LVS_EX_INFOTIP: u32 = 1024u32;
pub const LVS_EX_JUSTIFYCOLUMNS: u32 = 2097152u32;
pub const LVS_EX_LABELTIP: u32 = 16384u32;
pub const LVS_EX_MULTIWORKAREAS: u32 = 8192u32;
pub const LVS_EX_ONECLICKACTIVATE: u32 = 64u32;
pub const LVS_EX_REGIONAL: u32 = 512u32;
pub const LVS_EX_SIMPLESELECT: u32 = 1048576u32;
pub const LVS_EX_SINGLEROW: u32 = 262144u32;
pub const LVS_EX_SNAPTOGRID: u32 = 524288u32;
pub const LVS_EX_SUBITEMIMAGES: u32 = 2u32;
pub const LVS_EX_TRACKSELECT: u32 = 8u32;
pub const LVS_EX_TRANSPARENTBKGND: u32 = 4194304u32;
pub const LVS_EX_TRANSPARENTSHADOWTEXT: u32 = 8388608u32;
pub const LVS_EX_TWOCLICKACTIVATE: u32 = 128u32;
pub const LVS_EX_UNDERLINECOLD: u32 = 4096u32;
pub const LVS_EX_UNDERLINEHOT: u32 = 2048u32;
pub const LVS_ICON: u32 = 0u32;
pub const LVS_LIST: u32 = 3u32;
pub const LVS_NOCOLUMNHEADER: u32 = 16384u32;
pub const LVS_NOLABELWRAP: u32 = 128u32;
pub const LVS_NOSCROLL: u32 = 8192u32;
pub const LVS_NOSORTHEADER: u32 = 32768u32;
pub const LVS_OWNERDATA: u32 = 4096u32;
pub const LVS_OWNERDRAWFIXED: u32 = 1024u32;
pub const LVS_REPORT: u32 = 1u32;
pub const LVS_SHAREIMAGELISTS: u32 = 64u32;
pub const LVS_SHOWSELALWAYS: u32 = 8u32;
pub const LVS_SINGLESEL: u32 = 4u32;
pub const LVS_SMALLICON: u32 = 2u32;
pub const LVS_SORTASCENDING: u32 = 16u32;
pub const LVS_SORTDESCENDING: u32 = 32u32;
pub const LVS_TYPEMASK: u32 = 3u32;
pub const LVS_TYPESTYLEMASK: u32 = 64512u32;
#[repr(C)]
pub struct LVTILEINFO {
    pub cbSize: u32,
    pub iItem: i32,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut i32,
}
impl ::core::marker::Copy for LVTILEINFO {}
impl ::core::clone::Clone for LVTILEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVTILEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVTILEINFO").field("cbSize", &self.cbSize).field("iItem", &self.iItem).field("cColumns", &self.cColumns).field("puColumns", &self.puColumns).field("piColFmt", &self.piColFmt).finish()
    }
}
unsafe impl ::windows_core::Abi for LVTILEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVTILEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVTILEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVTILEINFO {}
impl ::core::default::Default for LVTILEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct LVTILEVIEWINFO {
    pub cbSize: u32,
    pub dwMask: u32,
    pub dwFlags: LVTILEVIEWINFO_FLAGS,
    pub sizeTile: ::win32_foundation::SIZE,
    pub cLines: i32,
    pub rcLabelMargin: ::win32_foundation::RECT,
}
impl ::core::marker::Copy for LVTILEVIEWINFO {}
impl ::core::clone::Clone for LVTILEVIEWINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LVTILEVIEWINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LVTILEVIEWINFO").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("dwFlags", &self.dwFlags).field("sizeTile", &self.sizeTile).field("cLines", &self.cLines).field("rcLabelMargin", &self.rcLabelMargin).finish()
    }
}
unsafe impl ::windows_core::Abi for LVTILEVIEWINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LVTILEVIEWINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LVTILEVIEWINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for LVTILEVIEWINFO {}
impl ::core::default::Default for LVTILEVIEWINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LVTILEVIEWINFO_FLAGS(pub u32);
pub const LVTVIF_EXTENDED: LVTILEVIEWINFO_FLAGS = LVTILEVIEWINFO_FLAGS(4u32);
impl ::core::marker::Copy for LVTILEVIEWINFO_FLAGS {}
impl ::core::clone::Clone for LVTILEVIEWINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LVTILEVIEWINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for LVTILEVIEWINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for LVTILEVIEWINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LVTILEVIEWINFO_FLAGS").field(&self.0).finish()
    }
}
pub const LVTVIF_AUTOSIZE: u32 = 0u32;
pub const LVTVIF_FIXEDHEIGHT: u32 = 2u32;
pub const LVTVIF_FIXEDSIZE: u32 = 3u32;
pub const LVTVIF_FIXEDWIDTH: u32 = 1u32;
pub const LVTVIM_COLUMNS: u32 = 2u32;
pub const LVTVIM_LABELMARGIN: u32 = 4u32;
pub const LVTVIM_TILESIZE: u32 = 1u32;
pub const LV_MAX_WORKAREAS: u32 = 16u32;
pub const LV_VIEW_DETAILS: u32 = 1u32;
pub const LV_VIEW_ICON: u32 = 0u32;
pub const LV_VIEW_LIST: u32 = 3u32;
pub const LV_VIEW_MAX: u32 = 4u32;
pub const LV_VIEW_SMALLICON: u32 = 2u32;
pub const LV_VIEW_TILE: u32 = 4u32;
pub const LWS_IGNORERETURN: u32 = 2u32;
pub const LWS_NOPREFIX: u32 = 4u32;
pub const LWS_RIGHT: u32 = 32u32;
pub const LWS_TRANSPARENT: u32 = 1u32;
pub const LWS_USECUSTOMTEXT: u32 = 16u32;
pub const LWS_USEVISUALSTYLE: u32 = 8u32;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn LoadIconMetric<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hinst: Param0, pszname: Param1, lims: _LI_METRIC) -> ::windows_core::Result<super::WindowsAndMessaging::HICON> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadIconMetric(hinst: ::win32_foundation::HINSTANCE, pszname: ::windows_core::PCWSTR, lims: _LI_METRIC, phico: *mut super::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::WindowsAndMessaging::HICON>::zeroed();
        LoadIconMetric(hinst.into_param().abi(), pszname.into_param().abi(), ::core::mem::transmute(lims), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn LoadIconWithScaleDown<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hinst: Param0, pszname: Param1, cx: i32, cy: i32) -> ::windows_core::Result<super::WindowsAndMessaging::HICON> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadIconWithScaleDown(hinst: ::win32_foundation::HINSTANCE, pszname: ::windows_core::PCWSTR, cx: i32, cy: i32, phico: *mut super::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::WindowsAndMessaging::HICON>::zeroed();
        LoadIconWithScaleDown(hinst.into_param().abi(), pszname.into_param().abi(), ::core::mem::transmute(cx), ::core::mem::transmute(cy), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct MARGINS {
    pub cxLeftWidth: i32,
    pub cxRightWidth: i32,
    pub cyTopHeight: i32,
    pub cyBottomHeight: i32,
}
impl ::core::marker::Copy for MARGINS {}
impl ::core::clone::Clone for MARGINS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MARGINS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MARGINS").field("cxLeftWidth", &self.cxLeftWidth).field("cxRightWidth", &self.cxRightWidth).field("cyTopHeight", &self.cyTopHeight).field("cyBottomHeight", &self.cyBottomHeight).finish()
    }
}
unsafe impl ::windows_core::Abi for MARGINS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MARGINS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MARGINS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MARGINS {}
impl ::core::default::Default for MARGINS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MARKUPTEXTSTATES(pub i32);
pub const EMT_NORMALTEXT: MARKUPTEXTSTATES = MARKUPTEXTSTATES(1i32);
pub const EMT_LINKTEXT: MARKUPTEXTSTATES = MARKUPTEXTSTATES(2i32);
impl ::core::marker::Copy for MARKUPTEXTSTATES {}
impl ::core::clone::Clone for MARKUPTEXTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MARKUPTEXTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MARKUPTEXTSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for MARKUPTEXTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MARKUPTEXTSTATES").field(&self.0).finish()
    }
}
pub const MAXPROPPAGES: u32 = 100u32;
pub const MAX_INTLIST_COUNT: u32 = 402u32;
pub const MAX_LINKID_TEXT: u32 = 48u32;
pub const MAX_THEMECOLOR: u32 = 64u32;
pub const MAX_THEMESIZE: u32 = 64u32;
#[repr(C)]
pub struct MCGRIDINFO {
    pub cbSize: u32,
    pub dwPart: MCGRIDINFO_PART,
    pub dwFlags: MCGRIDINFO_FLAGS,
    pub iCalendar: i32,
    pub iRow: i32,
    pub iCol: i32,
    pub bSelected: ::win32_foundation::BOOL,
    pub stStart: ::win32_foundation::SYSTEMTIME,
    pub stEnd: ::win32_foundation::SYSTEMTIME,
    pub rc: ::win32_foundation::RECT,
    pub pszName: ::windows_core::PWSTR,
    pub cchName: usize,
}
impl ::core::marker::Copy for MCGRIDINFO {}
impl ::core::clone::Clone for MCGRIDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCGRIDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCGRIDINFO").field("cbSize", &self.cbSize).field("dwPart", &self.dwPart).field("dwFlags", &self.dwFlags).field("iCalendar", &self.iCalendar).field("iRow", &self.iRow).field("iCol", &self.iCol).field("bSelected", &self.bSelected).field("stStart", &self.stStart).field("stEnd", &self.stEnd).field("rc", &self.rc).field("pszName", &self.pszName).field("cchName", &self.cchName).finish()
    }
}
unsafe impl ::windows_core::Abi for MCGRIDINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCGRIDINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCGRIDINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCGRIDINFO {}
impl ::core::default::Default for MCGRIDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MCGRIDINFO_FLAGS(pub u32);
pub const MCGIF_DATE: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(1u32);
pub const MCGIF_RECT: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(2u32);
pub const MCGIF_NAME: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(4u32);
impl ::core::marker::Copy for MCGRIDINFO_FLAGS {}
impl ::core::clone::Clone for MCGRIDINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MCGRIDINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MCGRIDINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MCGRIDINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MCGRIDINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MCGRIDINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MCGRIDINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MCGRIDINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MCGRIDINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MCGRIDINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MCGRIDINFO_PART(pub u32);
pub const MCGIP_CALENDARCONTROL: MCGRIDINFO_PART = MCGRIDINFO_PART(0u32);
pub const MCGIP_NEXT: MCGRIDINFO_PART = MCGRIDINFO_PART(1u32);
pub const MCGIP_PREV: MCGRIDINFO_PART = MCGRIDINFO_PART(2u32);
pub const MCGIP_FOOTER: MCGRIDINFO_PART = MCGRIDINFO_PART(3u32);
pub const MCGIP_CALENDAR: MCGRIDINFO_PART = MCGRIDINFO_PART(4u32);
pub const MCGIP_CALENDARHEADER: MCGRIDINFO_PART = MCGRIDINFO_PART(5u32);
pub const MCGIP_CALENDARBODY: MCGRIDINFO_PART = MCGRIDINFO_PART(6u32);
pub const MCGIP_CALENDARROW: MCGRIDINFO_PART = MCGRIDINFO_PART(7u32);
pub const MCGIP_CALENDARCELL: MCGRIDINFO_PART = MCGRIDINFO_PART(8u32);
impl ::core::marker::Copy for MCGRIDINFO_PART {}
impl ::core::clone::Clone for MCGRIDINFO_PART {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MCGRIDINFO_PART {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MCGRIDINFO_PART {
    type Abi = Self;
}
impl ::core::fmt::Debug for MCGRIDINFO_PART {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MCGRIDINFO_PART").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct MCHITTESTINFO {
    pub cbSize: u32,
    pub pt: ::win32_foundation::POINT,
    pub uHit: u32,
    pub st: ::win32_foundation::SYSTEMTIME,
    pub rc: ::win32_foundation::RECT,
    pub iOffset: i32,
    pub iRow: i32,
    pub iCol: i32,
}
impl ::core::marker::Copy for MCHITTESTINFO {}
impl ::core::clone::Clone for MCHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MCHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MCHITTESTINFO").field("cbSize", &self.cbSize).field("pt", &self.pt).field("uHit", &self.uHit).field("st", &self.st).field("rc", &self.rc).field("iOffset", &self.iOffset).field("iRow", &self.iRow).field("iCol", &self.iCol).finish()
    }
}
unsafe impl ::windows_core::Abi for MCHITTESTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MCHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MCHITTESTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MCHITTESTINFO {}
impl ::core::default::Default for MCHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const MCHT_CALENDAR: u32 = 131072u32;
pub const MCHT_CALENDARBK: u32 = 131072u32;
pub const MCHT_CALENDARCONTROL: u32 = 1048576u32;
pub const MCHT_NEXT: u32 = 16777216u32;
pub const MCHT_NOWHERE: u32 = 0u32;
pub const MCHT_PREV: u32 = 33554432u32;
pub const MCHT_TITLE: u32 = 65536u32;
pub const MCHT_TITLEBK: u32 = 65536u32;
pub const MCHT_TODAYLINK: u32 = 196608u32;
pub const MCMV_CENTURY: u32 = 3u32;
pub const MCMV_DECADE: u32 = 2u32;
pub const MCMV_MAX: u32 = 3u32;
pub const MCMV_MONTH: u32 = 0u32;
pub const MCMV_YEAR: u32 = 1u32;
pub const MCM_FIRST: u32 = 4096u32;
pub const MCM_GETCALENDARBORDER: u32 = 4127u32;
pub const MCM_GETCALENDARCOUNT: u32 = 4119u32;
pub const MCM_GETCALENDARGRIDINFO: u32 = 4120u32;
pub const MCM_GETCALID: u32 = 4123u32;
pub const MCM_GETCOLOR: u32 = 4107u32;
pub const MCM_GETCURRENTVIEW: u32 = 4118u32;
pub const MCM_GETCURSEL: u32 = 4097u32;
pub const MCM_GETFIRSTDAYOFWEEK: u32 = 4112u32;
pub const MCM_GETMAXSELCOUNT: u32 = 4099u32;
pub const MCM_GETMAXTODAYWIDTH: u32 = 4117u32;
pub const MCM_GETMINREQRECT: u32 = 4105u32;
pub const MCM_GETMONTHDELTA: u32 = 4115u32;
pub const MCM_GETMONTHRANGE: u32 = 4103u32;
pub const MCM_GETRANGE: u32 = 4113u32;
pub const MCM_GETSELRANGE: u32 = 4101u32;
pub const MCM_GETTODAY: u32 = 4109u32;
pub const MCM_GETUNICODEFORMAT: u32 = 8198u32;
pub const MCM_HITTEST: u32 = 4110u32;
pub const MCM_SETCALENDARBORDER: u32 = 4126u32;
pub const MCM_SETCALID: u32 = 4124u32;
pub const MCM_SETCOLOR: u32 = 4106u32;
pub const MCM_SETCURRENTVIEW: u32 = 4128u32;
pub const MCM_SETCURSEL: u32 = 4098u32;
pub const MCM_SETDAYSTATE: u32 = 4104u32;
pub const MCM_SETFIRSTDAYOFWEEK: u32 = 4111u32;
pub const MCM_SETMAXSELCOUNT: u32 = 4100u32;
pub const MCM_SETMONTHDELTA: u32 = 4116u32;
pub const MCM_SETRANGE: u32 = 4114u32;
pub const MCM_SETSELRANGE: u32 = 4102u32;
pub const MCM_SETTODAY: u32 = 4108u32;
pub const MCM_SETUNICODEFORMAT: u32 = 8197u32;
pub const MCM_SIZERECTTOMIN: u32 = 4125u32;
pub const MCSC_BACKGROUND: u32 = 0u32;
pub const MCSC_MONTHBK: u32 = 4u32;
pub const MCSC_TEXT: u32 = 1u32;
pub const MCSC_TITLEBK: u32 = 2u32;
pub const MCSC_TITLETEXT: u32 = 3u32;
pub const MCSC_TRAILINGTEXT: u32 = 5u32;
pub const MCS_DAYSTATE: u32 = 1u32;
pub const MCS_MULTISELECT: u32 = 2u32;
pub const MCS_NOSELCHANGEONNAV: u32 = 256u32;
pub const MCS_NOTODAY: u32 = 16u32;
pub const MCS_NOTODAYCIRCLE: u32 = 8u32;
pub const MCS_NOTRAILINGDATES: u32 = 64u32;
pub const MCS_SHORTDAYSOFWEEK: u32 = 128u32;
pub const MCS_WEEKNUMBERS: u32 = 4u32;
#[repr(C)]
pub struct MEASUREITEMSTRUCT {
    pub CtlType: u32,
    pub CtlID: u32,
    pub itemID: u32,
    pub itemWidth: u32,
    pub itemHeight: u32,
    pub itemData: usize,
}
impl ::core::marker::Copy for MEASUREITEMSTRUCT {}
impl ::core::clone::Clone for MEASUREITEMSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEASUREITEMSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEASUREITEMSTRUCT").field("CtlType", &self.CtlType).field("CtlID", &self.CtlID).field("itemID", &self.itemID).field("itemWidth", &self.itemWidth).field("itemHeight", &self.itemHeight).field("itemData", &self.itemData).finish()
    }
}
unsafe impl ::windows_core::Abi for MEASUREITEMSTRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MEASUREITEMSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEASUREITEMSTRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MEASUREITEMSTRUCT {}
impl ::core::default::Default for MEASUREITEMSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MENUBANDPARTS(pub i32);
pub const MDP_NEWAPPBUTTON: MENUBANDPARTS = MENUBANDPARTS(1i32);
pub const MDP_SEPERATOR: MENUBANDPARTS = MENUBANDPARTS(2i32);
impl ::core::marker::Copy for MENUBANDPARTS {}
impl ::core::clone::Clone for MENUBANDPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENUBANDPARTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MENUBANDPARTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MENUBANDPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUBANDPARTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MENUBANDSTATES(pub i32);
pub const MDS_NORMAL: MENUBANDSTATES = MENUBANDSTATES(1i32);
pub const MDS_HOT: MENUBANDSTATES = MENUBANDSTATES(2i32);
pub const MDS_PRESSED: MENUBANDSTATES = MENUBANDSTATES(3i32);
pub const MDS_DISABLED: MENUBANDSTATES = MENUBANDSTATES(4i32);
pub const MDS_CHECKED: MENUBANDSTATES = MENUBANDSTATES(5i32);
pub const MDS_HOTCHECKED: MENUBANDSTATES = MENUBANDSTATES(6i32);
impl ::core::marker::Copy for MENUBANDSTATES {}
impl ::core::clone::Clone for MENUBANDSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MENUBANDSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MENUBANDSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for MENUBANDSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUBANDSTATES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MONTHCALPARTS(pub i32);
pub const MC_BACKGROUND: MONTHCALPARTS = MONTHCALPARTS(1i32);
pub const MC_BORDERS: MONTHCALPARTS = MONTHCALPARTS(2i32);
pub const MC_GRIDBACKGROUND: MONTHCALPARTS = MONTHCALPARTS(3i32);
pub const MC_COLHEADERSPLITTER: MONTHCALPARTS = MONTHCALPARTS(4i32);
pub const MC_GRIDCELLBACKGROUND: MONTHCALPARTS = MONTHCALPARTS(5i32);
pub const MC_GRIDCELL: MONTHCALPARTS = MONTHCALPARTS(6i32);
pub const MC_GRIDCELLUPPER: MONTHCALPARTS = MONTHCALPARTS(7i32);
pub const MC_TRAILINGGRIDCELL: MONTHCALPARTS = MONTHCALPARTS(8i32);
pub const MC_TRAILINGGRIDCELLUPPER: MONTHCALPARTS = MONTHCALPARTS(9i32);
pub const MC_NAVNEXT: MONTHCALPARTS = MONTHCALPARTS(10i32);
pub const MC_NAVPREV: MONTHCALPARTS = MONTHCALPARTS(11i32);
impl ::core::marker::Copy for MONTHCALPARTS {}
impl ::core::clone::Clone for MONTHCALPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MONTHCALPARTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MONTHCALPARTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MONTHCALPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONTHCALPARTS").field(&self.0).finish()
    }
}
pub const MONTHCAL_CLASS: &str = "SysMonthCal32";
pub const MONTHCAL_CLASSA: &str = "SysMonthCal32";
pub const MONTHCAL_CLASSW: &str = "SysMonthCal32";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MOREPROGRAMSARROWBACKSTATES(pub i32);
pub const SPSB_NORMAL: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(1i32);
pub const SPSB_HOT: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(2i32);
pub const SPSB_PRESSED: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(3i32);
impl ::core::marker::Copy for MOREPROGRAMSARROWBACKSTATES {}
impl ::core::clone::Clone for MOREPROGRAMSARROWBACKSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MOREPROGRAMSARROWBACKSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MOREPROGRAMSARROWBACKSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for MOREPROGRAMSARROWBACKSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOREPROGRAMSARROWBACKSTATES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MOREPROGRAMSARROWSTATES(pub i32);
pub const SPS_NORMAL: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(1i32);
pub const SPS_HOT: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(2i32);
pub const SPS_PRESSED: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(3i32);
impl ::core::marker::Copy for MOREPROGRAMSARROWSTATES {}
impl ::core::clone::Clone for MOREPROGRAMSARROWSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MOREPROGRAMSARROWSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MOREPROGRAMSARROWSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for MOREPROGRAMSARROWSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOREPROGRAMSARROWSTATES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MOREPROGRAMSTABSTATES(pub i32);
pub const SPMPT_NORMAL: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(1i32);
pub const SPMPT_HOT: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(2i32);
pub const SPMPT_SELECTED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(3i32);
pub const SPMPT_DISABLED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(4i32);
pub const SPMPT_FOCUSED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(5i32);
impl ::core::marker::Copy for MOREPROGRAMSTABSTATES {}
impl ::core::clone::Clone for MOREPROGRAMSTABSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MOREPROGRAMSTABSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MOREPROGRAMSTABSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for MOREPROGRAMSTABSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOREPROGRAMSTABSTATES").field(&self.0).finish()
    }
}
pub const MSGF_COMMCTRL_BEGINDRAG: u32 = 16896u32;
pub const MSGF_COMMCTRL_DRAGSELECT: u32 = 16898u32;
pub const MSGF_COMMCTRL_SIZEHEADER: u32 = 16897u32;
pub const MSGF_COMMCTRL_TOOLBARCUST: u32 = 16899u32;
pub const MULTIFILEOPENORD: u32 = 1537u32;
#[inline]
pub unsafe fn MakeDragList<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hlb: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MakeDragList(hlb: ::win32_foundation::HWND) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(MakeDragList(hlb.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn MenuHelp<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::WPARAM>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::LPARAM>, Param3: ::windows_core::IntoParam<'a, super::WindowsAndMessaging::HMENU>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param5: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(umsg: u32, wparam: Param1, lparam: Param2, hmainmenu: Param3, hinst: Param4, hwndstatus: Param5, lpwids: *const u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MenuHelp(umsg: u32, wparam: ::win32_foundation::WPARAM, lparam: ::win32_foundation::LPARAM, hmainmenu: super::WindowsAndMessaging::HMENU, hinst: ::win32_foundation::HINSTANCE, hwndstatus: ::win32_foundation::HWND, lpwids: *const u32);
        }
        MenuHelp(::core::mem::transmute(umsg), wparam.into_param().abi(), lparam.into_param().abi(), hmainmenu.into_param().abi(), hinst.into_param().abi(), hwndstatus.into_param().abi(), ::core::mem::transmute(lpwids))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NAVNEXTSTATES(pub i32);
pub const MCNN_NORMAL: NAVNEXTSTATES = NAVNEXTSTATES(1i32);
pub const MCNN_HOT: NAVNEXTSTATES = NAVNEXTSTATES(2i32);
pub const MCNN_PRESSED: NAVNEXTSTATES = NAVNEXTSTATES(3i32);
pub const MCNN_DISABLED: NAVNEXTSTATES = NAVNEXTSTATES(4i32);
impl ::core::marker::Copy for NAVNEXTSTATES {}
impl ::core::clone::Clone for NAVNEXTSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NAVNEXTSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NAVNEXTSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for NAVNEXTSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAVNEXTSTATES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NAVPREVSTATES(pub i32);
pub const MCNP_NORMAL: NAVPREVSTATES = NAVPREVSTATES(1i32);
pub const MCNP_HOT: NAVPREVSTATES = NAVPREVSTATES(2i32);
pub const MCNP_PRESSED: NAVPREVSTATES = NAVPREVSTATES(3i32);
pub const MCNP_DISABLED: NAVPREVSTATES = NAVPREVSTATES(4i32);
impl ::core::marker::Copy for NAVPREVSTATES {}
impl ::core::clone::Clone for NAVPREVSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NAVPREVSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NAVPREVSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for NAVPREVSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAVPREVSTATES").field(&self.0).finish()
    }
}
pub const NEWFILEOPENORD: u32 = 1547u32;
pub const NEWFILEOPENV2ORD: u32 = 1552u32;
pub const NEWFILEOPENV3ORD: u32 = 1553u32;
pub const NEWFORMATDLGWITHLINK: u32 = 1591u32;
pub const NFS_ALL: u32 = 16u32;
pub const NFS_BUTTON: u32 = 8u32;
pub const NFS_EDIT: u32 = 1u32;
pub const NFS_LISTCOMBO: u32 = 4u32;
pub const NFS_STATIC: u32 = 2u32;
pub const NFS_USEFONTASSOC: u32 = 32u32;
#[repr(C)]
pub struct NMBCDROPDOWN {
    pub hdr: NMHDR,
    pub rcButton: ::win32_foundation::RECT,
}
impl ::core::marker::Copy for NMBCDROPDOWN {}
impl ::core::clone::Clone for NMBCDROPDOWN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMBCDROPDOWN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMBCDROPDOWN").field("hdr", &self.hdr).field("rcButton", &self.rcButton).finish()
    }
}
unsafe impl ::windows_core::Abi for NMBCDROPDOWN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMBCDROPDOWN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMBCDROPDOWN>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMBCDROPDOWN {}
impl ::core::default::Default for NMBCDROPDOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMBCHOTITEM {
    pub hdr: NMHDR,
    pub dwFlags: NMTBHOTITEM_FLAGS,
}
impl ::core::marker::Copy for NMBCHOTITEM {}
impl ::core::clone::Clone for NMBCHOTITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMBCHOTITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMBCHOTITEM").field("hdr", &self.hdr).field("dwFlags", &self.dwFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for NMBCHOTITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMBCHOTITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMBCHOTITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMBCHOTITEM {}
impl ::core::default::Default for NMBCHOTITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMCBEDRAGBEGINA {
    pub hdr: NMHDR,
    pub iItemid: i32,
    pub szText: [::win32_foundation::CHAR; 260],
}
impl ::core::marker::Copy for NMCBEDRAGBEGINA {}
impl ::core::clone::Clone for NMCBEDRAGBEGINA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMCBEDRAGBEGINA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCBEDRAGBEGINA").field("hdr", &self.hdr).field("iItemid", &self.iItemid).field("szText", &self.szText).finish()
    }
}
unsafe impl ::windows_core::Abi for NMCBEDRAGBEGINA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMCBEDRAGBEGINA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMCBEDRAGBEGINA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMCBEDRAGBEGINA {}
impl ::core::default::Default for NMCBEDRAGBEGINA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMCBEDRAGBEGINW {
    pub hdr: NMHDR,
    pub iItemid: i32,
    pub szText: [u16; 260],
}
impl ::core::marker::Copy for NMCBEDRAGBEGINW {}
impl ::core::clone::Clone for NMCBEDRAGBEGINW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMCBEDRAGBEGINW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCBEDRAGBEGINW").field("hdr", &self.hdr).field("iItemid", &self.iItemid).field("szText", &self.szText).finish()
    }
}
unsafe impl ::windows_core::Abi for NMCBEDRAGBEGINW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMCBEDRAGBEGINW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMCBEDRAGBEGINW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMCBEDRAGBEGINW {}
impl ::core::default::Default for NMCBEDRAGBEGINW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMCBEENDEDITA {
    pub hdr: NMHDR,
    pub fChanged: ::win32_foundation::BOOL,
    pub iNewSelection: i32,
    pub szText: [::win32_foundation::CHAR; 260],
    pub iWhy: i32,
}
impl ::core::marker::Copy for NMCBEENDEDITA {}
impl ::core::clone::Clone for NMCBEENDEDITA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMCBEENDEDITA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCBEENDEDITA").field("hdr", &self.hdr).field("fChanged", &self.fChanged).field("iNewSelection", &self.iNewSelection).field("szText", &self.szText).field("iWhy", &self.iWhy).finish()
    }
}
unsafe impl ::windows_core::Abi for NMCBEENDEDITA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMCBEENDEDITA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMCBEENDEDITA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMCBEENDEDITA {}
impl ::core::default::Default for NMCBEENDEDITA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMCBEENDEDITW {
    pub hdr: NMHDR,
    pub fChanged: ::win32_foundation::BOOL,
    pub iNewSelection: i32,
    pub szText: [u16; 260],
    pub iWhy: i32,
}
impl ::core::marker::Copy for NMCBEENDEDITW {}
impl ::core::clone::Clone for NMCBEENDEDITW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMCBEENDEDITW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCBEENDEDITW").field("hdr", &self.hdr).field("fChanged", &self.fChanged).field("iNewSelection", &self.iNewSelection).field("szText", &self.szText).field("iWhy", &self.iWhy).finish()
    }
}
unsafe impl ::windows_core::Abi for NMCBEENDEDITW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMCBEENDEDITW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMCBEENDEDITW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMCBEENDEDITW {}
impl ::core::default::Default for NMCBEENDEDITW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMCHAR {
    pub hdr: NMHDR,
    pub ch: u32,
    pub dwItemPrev: u32,
    pub dwItemNext: u32,
}
impl ::core::marker::Copy for NMCHAR {}
impl ::core::clone::Clone for NMCHAR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMCHAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCHAR").field("hdr", &self.hdr).field("ch", &self.ch).field("dwItemPrev", &self.dwItemPrev).field("dwItemNext", &self.dwItemNext).finish()
    }
}
unsafe impl ::windows_core::Abi for NMCHAR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMCHAR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMCHAR>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMCHAR {}
impl ::core::default::Default for NMCHAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMCOMBOBOXEXA {
    pub hdr: NMHDR,
    pub ceItem: COMBOBOXEXITEMA,
}
impl ::core::marker::Copy for NMCOMBOBOXEXA {}
impl ::core::clone::Clone for NMCOMBOBOXEXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMCOMBOBOXEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCOMBOBOXEXA").field("hdr", &self.hdr).field("ceItem", &self.ceItem).finish()
    }
}
unsafe impl ::windows_core::Abi for NMCOMBOBOXEXA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMCOMBOBOXEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMCOMBOBOXEXA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMCOMBOBOXEXA {}
impl ::core::default::Default for NMCOMBOBOXEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMCOMBOBOXEXW {
    pub hdr: NMHDR,
    pub ceItem: COMBOBOXEXITEMW,
}
impl ::core::marker::Copy for NMCOMBOBOXEXW {}
impl ::core::clone::Clone for NMCOMBOBOXEXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMCOMBOBOXEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCOMBOBOXEXW").field("hdr", &self.hdr).field("ceItem", &self.ceItem).finish()
    }
}
unsafe impl ::windows_core::Abi for NMCOMBOBOXEXW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMCOMBOBOXEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMCOMBOBOXEXW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMCOMBOBOXEXW {}
impl ::core::default::Default for NMCOMBOBOXEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct NMCUSTOMDRAW {
    pub hdr: NMHDR,
    pub dwDrawStage: NMCUSTOMDRAW_DRAW_STAGE,
    pub hdc: ::win32_graphics::Gdi::HDC,
    pub rc: ::win32_foundation::RECT,
    pub dwItemSpec: usize,
    pub uItemState: u32,
    pub lItemlParam: ::win32_foundation::LPARAM,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for NMCUSTOMDRAW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for NMCUSTOMDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for NMCUSTOMDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCUSTOMDRAW").field("hdr", &self.hdr).field("dwDrawStage", &self.dwDrawStage).field("hdc", &self.hdc).field("rc", &self.rc).field("dwItemSpec", &self.dwItemSpec).field("uItemState", &self.uItemState).field("lItemlParam", &self.lItemlParam).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for NMCUSTOMDRAW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NMCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMCUSTOMDRAW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NMCUSTOMDRAW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NMCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NMCUSTOMDRAW_DRAW_STAGE(pub u32);
pub const CDDS_POSTPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(2u32);
pub const CDDS_PREERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(3u32);
pub const CDDS_PREPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(1u32);
pub const CDDS_ITEMPOSTERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65540u32);
pub const CDDS_ITEMPOSTPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65538u32);
pub const CDDS_ITEMPREERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65539u32);
pub const CDDS_ITEMPREPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65537u32);
pub const CDDS_SUBITEM: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(131072u32);
impl ::core::marker::Copy for NMCUSTOMDRAW_DRAW_STAGE {}
impl ::core::clone::Clone for NMCUSTOMDRAW_DRAW_STAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NMCUSTOMDRAW_DRAW_STAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NMCUSTOMDRAW_DRAW_STAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NMCUSTOMDRAW_DRAW_STAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMCUSTOMDRAW_DRAW_STAGE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct NMCUSTOMSPLITRECTINFO {
    pub hdr: NMHDR,
    pub rcClient: ::win32_foundation::RECT,
    pub rcButton: ::win32_foundation::RECT,
    pub rcSplit: ::win32_foundation::RECT,
}
impl ::core::marker::Copy for NMCUSTOMSPLITRECTINFO {}
impl ::core::clone::Clone for NMCUSTOMSPLITRECTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMCUSTOMSPLITRECTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCUSTOMSPLITRECTINFO").field("hdr", &self.hdr).field("rcClient", &self.rcClient).field("rcButton", &self.rcButton).field("rcSplit", &self.rcSplit).finish()
    }
}
unsafe impl ::windows_core::Abi for NMCUSTOMSPLITRECTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMCUSTOMSPLITRECTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMCUSTOMSPLITRECTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMCUSTOMSPLITRECTINFO {}
impl ::core::default::Default for NMCUSTOMSPLITRECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct NMCUSTOMTEXT {
    pub hdr: NMHDR,
    pub hDC: ::win32_graphics::Gdi::HDC,
    pub lpString: ::windows_core::PCWSTR,
    pub nCount: i32,
    pub lpRect: *mut ::win32_foundation::RECT,
    pub uFormat: u32,
    pub fLink: ::win32_foundation::BOOL,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for NMCUSTOMTEXT {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for NMCUSTOMTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for NMCUSTOMTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMCUSTOMTEXT").field("hdr", &self.hdr).field("hDC", &self.hDC).field("lpString", &self.lpString).field("nCount", &self.nCount).field("lpRect", &self.lpRect).field("uFormat", &self.uFormat).field("fLink", &self.fLink).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for NMCUSTOMTEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NMCUSTOMTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMCUSTOMTEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NMCUSTOMTEXT {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NMCUSTOMTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMDATETIMECHANGE {
    pub nmhdr: NMHDR,
    pub dwFlags: u32,
    pub st: ::win32_foundation::SYSTEMTIME,
}
impl ::core::marker::Copy for NMDATETIMECHANGE {}
impl ::core::clone::Clone for NMDATETIMECHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMDATETIMECHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMECHANGE").field("nmhdr", &self.nmhdr).field("dwFlags", &self.dwFlags).field("st", &self.st).finish()
    }
}
unsafe impl ::windows_core::Abi for NMDATETIMECHANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMDATETIMECHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMDATETIMECHANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMDATETIMECHANGE {}
impl ::core::default::Default for NMDATETIMECHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMDATETIMEFORMATA {
    pub nmhdr: NMHDR,
    pub pszFormat: ::windows_core::PCSTR,
    pub st: ::win32_foundation::SYSTEMTIME,
    pub pszDisplay: ::windows_core::PCSTR,
    pub szDisplay: [::win32_foundation::CHAR; 64],
}
impl ::core::marker::Copy for NMDATETIMEFORMATA {}
impl ::core::clone::Clone for NMDATETIMEFORMATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMDATETIMEFORMATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMEFORMATA").field("nmhdr", &self.nmhdr).field("pszFormat", &self.pszFormat).field("st", &self.st).field("pszDisplay", &self.pszDisplay).field("szDisplay", &self.szDisplay).finish()
    }
}
unsafe impl ::windows_core::Abi for NMDATETIMEFORMATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMDATETIMEFORMATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMDATETIMEFORMATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMDATETIMEFORMATA {}
impl ::core::default::Default for NMDATETIMEFORMATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMDATETIMEFORMATQUERYA {
    pub nmhdr: NMHDR,
    pub pszFormat: ::windows_core::PCSTR,
    pub szMax: ::win32_foundation::SIZE,
}
impl ::core::marker::Copy for NMDATETIMEFORMATQUERYA {}
impl ::core::clone::Clone for NMDATETIMEFORMATQUERYA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMDATETIMEFORMATQUERYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMEFORMATQUERYA").field("nmhdr", &self.nmhdr).field("pszFormat", &self.pszFormat).field("szMax", &self.szMax).finish()
    }
}
unsafe impl ::windows_core::Abi for NMDATETIMEFORMATQUERYA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMDATETIMEFORMATQUERYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMDATETIMEFORMATQUERYA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMDATETIMEFORMATQUERYA {}
impl ::core::default::Default for NMDATETIMEFORMATQUERYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMDATETIMEFORMATQUERYW {
    pub nmhdr: NMHDR,
    pub pszFormat: ::windows_core::PCWSTR,
    pub szMax: ::win32_foundation::SIZE,
}
impl ::core::marker::Copy for NMDATETIMEFORMATQUERYW {}
impl ::core::clone::Clone for NMDATETIMEFORMATQUERYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMDATETIMEFORMATQUERYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMEFORMATQUERYW").field("nmhdr", &self.nmhdr).field("pszFormat", &self.pszFormat).field("szMax", &self.szMax).finish()
    }
}
unsafe impl ::windows_core::Abi for NMDATETIMEFORMATQUERYW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMDATETIMEFORMATQUERYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMDATETIMEFORMATQUERYW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMDATETIMEFORMATQUERYW {}
impl ::core::default::Default for NMDATETIMEFORMATQUERYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMDATETIMEFORMATW {
    pub nmhdr: NMHDR,
    pub pszFormat: ::windows_core::PCWSTR,
    pub st: ::win32_foundation::SYSTEMTIME,
    pub pszDisplay: ::windows_core::PCWSTR,
    pub szDisplay: [u16; 64],
}
impl ::core::marker::Copy for NMDATETIMEFORMATW {}
impl ::core::clone::Clone for NMDATETIMEFORMATW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMDATETIMEFORMATW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMEFORMATW").field("nmhdr", &self.nmhdr).field("pszFormat", &self.pszFormat).field("st", &self.st).field("pszDisplay", &self.pszDisplay).field("szDisplay", &self.szDisplay).finish()
    }
}
unsafe impl ::windows_core::Abi for NMDATETIMEFORMATW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMDATETIMEFORMATW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMDATETIMEFORMATW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMDATETIMEFORMATW {}
impl ::core::default::Default for NMDATETIMEFORMATW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMDATETIMESTRINGA {
    pub nmhdr: NMHDR,
    pub pszUserString: ::windows_core::PCSTR,
    pub st: ::win32_foundation::SYSTEMTIME,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for NMDATETIMESTRINGA {}
impl ::core::clone::Clone for NMDATETIMESTRINGA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMDATETIMESTRINGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMESTRINGA").field("nmhdr", &self.nmhdr).field("pszUserString", &self.pszUserString).field("st", &self.st).field("dwFlags", &self.dwFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for NMDATETIMESTRINGA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMDATETIMESTRINGA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMDATETIMESTRINGA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMDATETIMESTRINGA {}
impl ::core::default::Default for NMDATETIMESTRINGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMDATETIMESTRINGW {
    pub nmhdr: NMHDR,
    pub pszUserString: ::windows_core::PCWSTR,
    pub st: ::win32_foundation::SYSTEMTIME,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for NMDATETIMESTRINGW {}
impl ::core::clone::Clone for NMDATETIMESTRINGW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMDATETIMESTRINGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMESTRINGW").field("nmhdr", &self.nmhdr).field("pszUserString", &self.pszUserString).field("st", &self.st).field("dwFlags", &self.dwFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for NMDATETIMESTRINGW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMDATETIMESTRINGW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMDATETIMESTRINGW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMDATETIMESTRINGW {}
impl ::core::default::Default for NMDATETIMESTRINGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMDATETIMEWMKEYDOWNA {
    pub nmhdr: NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: ::windows_core::PCSTR,
    pub st: ::win32_foundation::SYSTEMTIME,
}
impl ::core::marker::Copy for NMDATETIMEWMKEYDOWNA {}
impl ::core::clone::Clone for NMDATETIMEWMKEYDOWNA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMDATETIMEWMKEYDOWNA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMEWMKEYDOWNA").field("nmhdr", &self.nmhdr).field("nVirtKey", &self.nVirtKey).field("pszFormat", &self.pszFormat).field("st", &self.st).finish()
    }
}
unsafe impl ::windows_core::Abi for NMDATETIMEWMKEYDOWNA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMDATETIMEWMKEYDOWNA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMDATETIMEWMKEYDOWNA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMDATETIMEWMKEYDOWNA {}
impl ::core::default::Default for NMDATETIMEWMKEYDOWNA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMDATETIMEWMKEYDOWNW {
    pub nmhdr: NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: ::windows_core::PCWSTR,
    pub st: ::win32_foundation::SYSTEMTIME,
}
impl ::core::marker::Copy for NMDATETIMEWMKEYDOWNW {}
impl ::core::clone::Clone for NMDATETIMEWMKEYDOWNW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMDATETIMEWMKEYDOWNW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDATETIMEWMKEYDOWNW").field("nmhdr", &self.nmhdr).field("nVirtKey", &self.nVirtKey).field("pszFormat", &self.pszFormat).field("st", &self.st).finish()
    }
}
unsafe impl ::windows_core::Abi for NMDATETIMEWMKEYDOWNW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMDATETIMEWMKEYDOWNW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMDATETIMEWMKEYDOWNW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMDATETIMEWMKEYDOWNW {}
impl ::core::default::Default for NMDATETIMEWMKEYDOWNW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMDAYSTATE {
    pub nmhdr: NMHDR,
    pub stStart: ::win32_foundation::SYSTEMTIME,
    pub cDayState: i32,
    pub prgDayState: *mut u32,
}
impl ::core::marker::Copy for NMDAYSTATE {}
impl ::core::clone::Clone for NMDAYSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMDAYSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMDAYSTATE").field("nmhdr", &self.nmhdr).field("stStart", &self.stStart).field("cDayState", &self.cDayState).field("prgDayState", &self.prgDayState).finish()
    }
}
unsafe impl ::windows_core::Abi for NMDAYSTATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMDAYSTATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMDAYSTATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMDAYSTATE {}
impl ::core::default::Default for NMDAYSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMHDDISPINFOA {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub mask: HDI_MASK,
    pub pszText: ::windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMHDDISPINFOA {}
impl ::core::clone::Clone for NMHDDISPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMHDDISPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMHDDISPINFOA").field("hdr", &self.hdr).field("iItem", &self.iItem).field("mask", &self.mask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMHDDISPINFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMHDDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMHDDISPINFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMHDDISPINFOA {}
impl ::core::default::Default for NMHDDISPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMHDDISPINFOW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub mask: HDI_MASK,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMHDDISPINFOW {}
impl ::core::clone::Clone for NMHDDISPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMHDDISPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMHDDISPINFOW").field("hdr", &self.hdr).field("iItem", &self.iItem).field("mask", &self.mask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMHDDISPINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMHDDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMHDDISPINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMHDDISPINFOW {}
impl ::core::default::Default for NMHDDISPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMHDFILTERBTNCLICK {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub rc: ::win32_foundation::RECT,
}
impl ::core::marker::Copy for NMHDFILTERBTNCLICK {}
impl ::core::clone::Clone for NMHDFILTERBTNCLICK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMHDFILTERBTNCLICK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMHDFILTERBTNCLICK").field("hdr", &self.hdr).field("iItem", &self.iItem).field("rc", &self.rc).finish()
    }
}
unsafe impl ::windows_core::Abi for NMHDFILTERBTNCLICK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMHDFILTERBTNCLICK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMHDFILTERBTNCLICK>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMHDFILTERBTNCLICK {}
impl ::core::default::Default for NMHDFILTERBTNCLICK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMHDR {
    pub hwndFrom: ::win32_foundation::HWND,
    pub idFrom: usize,
    pub code: u32,
}
impl ::core::marker::Copy for NMHDR {}
impl ::core::clone::Clone for NMHDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMHDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMHDR").field("hwndFrom", &self.hwndFrom).field("idFrom", &self.idFrom).field("code", &self.code).finish()
    }
}
unsafe impl ::windows_core::Abi for NMHDR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMHDR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMHDR>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMHDR {}
impl ::core::default::Default for NMHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct NMHEADERA {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iButton: HEADER_CONTROL_NOTIFICATION_BUTTON,
    pub pitem: *mut HDITEMA,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for NMHEADERA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for NMHEADERA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for NMHEADERA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMHEADERA").field("hdr", &self.hdr).field("iItem", &self.iItem).field("iButton", &self.iButton).field("pitem", &self.pitem).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for NMHEADERA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NMHEADERA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMHEADERA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NMHEADERA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NMHEADERA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct NMHEADERW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iButton: HEADER_CONTROL_NOTIFICATION_BUTTON,
    pub pitem: *mut HDITEMW,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for NMHEADERW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for NMHEADERW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for NMHEADERW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMHEADERW").field("hdr", &self.hdr).field("iItem", &self.iItem).field("iButton", &self.iButton).field("pitem", &self.pitem).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for NMHEADERW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NMHEADERW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMHEADERW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NMHEADERW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NMHEADERW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMIPADDRESS {
    pub hdr: NMHDR,
    pub iField: i32,
    pub iValue: i32,
}
impl ::core::marker::Copy for NMIPADDRESS {}
impl ::core::clone::Clone for NMIPADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMIPADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMIPADDRESS").field("hdr", &self.hdr).field("iField", &self.iField).field("iValue", &self.iValue).finish()
    }
}
unsafe impl ::windows_core::Abi for NMIPADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMIPADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMIPADDRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMIPADDRESS {}
impl ::core::default::Default for NMIPADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMITEMACTIVATE {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: ::win32_foundation::POINT,
    pub lParam: ::win32_foundation::LPARAM,
    pub uKeyFlags: u32,
}
impl ::core::marker::Copy for NMITEMACTIVATE {}
impl ::core::clone::Clone for NMITEMACTIVATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMITEMACTIVATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMITEMACTIVATE").field("hdr", &self.hdr).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("uNewState", &self.uNewState).field("uOldState", &self.uOldState).field("uChanged", &self.uChanged).field("ptAction", &self.ptAction).field("lParam", &self.lParam).field("uKeyFlags", &self.uKeyFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for NMITEMACTIVATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMITEMACTIVATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMITEMACTIVATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMITEMACTIVATE {}
impl ::core::default::Default for NMITEMACTIVATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMKEY {
    pub hdr: NMHDR,
    pub nVKey: u32,
    pub uFlags: u32,
}
impl ::core::marker::Copy for NMKEY {}
impl ::core::clone::Clone for NMKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMKEY").field("hdr", &self.hdr).field("nVKey", &self.nVKey).field("uFlags", &self.uFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for NMKEY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMKEY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMKEY>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMKEY {}
impl ::core::default::Default for NMKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMLINK {
    pub hdr: NMHDR,
    pub item: LITEM,
}
impl ::core::marker::Copy for NMLINK {}
impl ::core::clone::Clone for NMLINK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLINK").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLINK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLINK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLINK>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLINK {}
impl ::core::default::Default for NMLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMLISTVIEW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: ::win32_foundation::POINT,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMLISTVIEW {}
impl ::core::clone::Clone for NMLISTVIEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLISTVIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLISTVIEW").field("hdr", &self.hdr).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("uNewState", &self.uNewState).field("uOldState", &self.uOldState).field("uChanged", &self.uChanged).field("ptAction", &self.ptAction).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLISTVIEW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLISTVIEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLISTVIEW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLISTVIEW {}
impl ::core::default::Default for NMLISTVIEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMLVCACHEHINT {
    pub hdr: NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
}
impl ::core::marker::Copy for NMLVCACHEHINT {}
impl ::core::clone::Clone for NMLVCACHEHINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLVCACHEHINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVCACHEHINT").field("hdr", &self.hdr).field("iFrom", &self.iFrom).field("iTo", &self.iTo).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLVCACHEHINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLVCACHEHINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVCACHEHINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLVCACHEHINT {}
impl ::core::default::Default for NMLVCACHEHINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct NMLVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: u32,
    pub clrTextBk: u32,
    pub iSubItem: i32,
    pub dwItemType: NMLVCUSTOMDRAW_ITEM_TYPE,
    pub clrFace: u32,
    pub iIconEffect: i32,
    pub iIconPhase: i32,
    pub iPartId: i32,
    pub iStateId: i32,
    pub rcText: ::win32_foundation::RECT,
    pub uAlign: NMLVCUSTOMDRAW_ALIGN,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for NMLVCUSTOMDRAW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for NMLVCUSTOMDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for NMLVCUSTOMDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVCUSTOMDRAW").field("nmcd", &self.nmcd).field("clrText", &self.clrText).field("clrTextBk", &self.clrTextBk).field("iSubItem", &self.iSubItem).field("dwItemType", &self.dwItemType).field("clrFace", &self.clrFace).field("iIconEffect", &self.iIconEffect).field("iIconPhase", &self.iIconPhase).field("iPartId", &self.iPartId).field("iStateId", &self.iStateId).field("rcText", &self.rcText).field("uAlign", &self.uAlign).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for NMLVCUSTOMDRAW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NMLVCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVCUSTOMDRAW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NMLVCUSTOMDRAW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NMLVCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NMLVCUSTOMDRAW_ALIGN(pub u32);
pub const LVGA_HEADER_CENTER: NMLVCUSTOMDRAW_ALIGN = NMLVCUSTOMDRAW_ALIGN(2u32);
pub const LVGA_HEADER_LEFT: NMLVCUSTOMDRAW_ALIGN = NMLVCUSTOMDRAW_ALIGN(1u32);
pub const LVGA_HEADER_RIGHT: NMLVCUSTOMDRAW_ALIGN = NMLVCUSTOMDRAW_ALIGN(4u32);
impl ::core::marker::Copy for NMLVCUSTOMDRAW_ALIGN {}
impl ::core::clone::Clone for NMLVCUSTOMDRAW_ALIGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NMLVCUSTOMDRAW_ALIGN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NMLVCUSTOMDRAW_ALIGN {
    type Abi = Self;
}
impl ::core::fmt::Debug for NMLVCUSTOMDRAW_ALIGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMLVCUSTOMDRAW_ALIGN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NMLVCUSTOMDRAW_ITEM_TYPE(pub u32);
pub const LVCDI_ITEM: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(0u32);
pub const LVCDI_GROUP: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(1u32);
pub const LVCDI_ITEMSLIST: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(2u32);
impl ::core::marker::Copy for NMLVCUSTOMDRAW_ITEM_TYPE {}
impl ::core::clone::Clone for NMLVCUSTOMDRAW_ITEM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NMLVCUSTOMDRAW_ITEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NMLVCUSTOMDRAW_ITEM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NMLVCUSTOMDRAW_ITEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMLVCUSTOMDRAW_ITEM_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct NMLVDISPINFOA {
    pub hdr: NMHDR,
    pub item: LVITEMA,
}
impl ::core::marker::Copy for NMLVDISPINFOA {}
impl ::core::clone::Clone for NMLVDISPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLVDISPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVDISPINFOA").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLVDISPINFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLVDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVDISPINFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLVDISPINFOA {}
impl ::core::default::Default for NMLVDISPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMLVDISPINFOW {
    pub hdr: NMHDR,
    pub item: LVITEMW,
}
impl ::core::marker::Copy for NMLVDISPINFOW {}
impl ::core::clone::Clone for NMLVDISPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLVDISPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVDISPINFOW").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLVDISPINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLVDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVDISPINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLVDISPINFOW {}
impl ::core::default::Default for NMLVDISPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMLVEMPTYMARKUP {
    pub hdr: NMHDR,
    pub dwFlags: NMLVEMPTYMARKUP_FLAGS,
    pub szMarkup: [u16; 2084],
}
impl ::core::marker::Copy for NMLVEMPTYMARKUP {}
impl ::core::clone::Clone for NMLVEMPTYMARKUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLVEMPTYMARKUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVEMPTYMARKUP").field("hdr", &self.hdr).field("dwFlags", &self.dwFlags).field("szMarkup", &self.szMarkup).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLVEMPTYMARKUP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLVEMPTYMARKUP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVEMPTYMARKUP>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLVEMPTYMARKUP {}
impl ::core::default::Default for NMLVEMPTYMARKUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NMLVEMPTYMARKUP_FLAGS(pub u32);
pub const EMF_CENTERED: NMLVEMPTYMARKUP_FLAGS = NMLVEMPTYMARKUP_FLAGS(1u32);
impl ::core::marker::Copy for NMLVEMPTYMARKUP_FLAGS {}
impl ::core::clone::Clone for NMLVEMPTYMARKUP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NMLVEMPTYMARKUP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NMLVEMPTYMARKUP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NMLVEMPTYMARKUP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMLVEMPTYMARKUP_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct NMLVFINDITEMA {
    pub hdr: NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOA,
}
impl ::core::marker::Copy for NMLVFINDITEMA {}
impl ::core::clone::Clone for NMLVFINDITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLVFINDITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVFINDITEMA").field("hdr", &self.hdr).field("iStart", &self.iStart).field("lvfi", &self.lvfi).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLVFINDITEMA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLVFINDITEMA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVFINDITEMA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLVFINDITEMA {}
impl ::core::default::Default for NMLVFINDITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMLVFINDITEMW {
    pub hdr: NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOW,
}
impl ::core::marker::Copy for NMLVFINDITEMW {}
impl ::core::clone::Clone for NMLVFINDITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLVFINDITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVFINDITEMW").field("hdr", &self.hdr).field("iStart", &self.iStart).field("lvfi", &self.lvfi).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLVFINDITEMW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLVFINDITEMW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVFINDITEMW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLVFINDITEMW {}
impl ::core::default::Default for NMLVFINDITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMLVGETINFOTIPA {
    pub hdr: NMHDR,
    pub dwFlags: u32,
    pub pszText: ::windows_core::PSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMLVGETINFOTIPA {}
impl ::core::clone::Clone for NMLVGETINFOTIPA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLVGETINFOTIPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVGETINFOTIPA").field("hdr", &self.hdr).field("dwFlags", &self.dwFlags).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLVGETINFOTIPA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLVGETINFOTIPA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVGETINFOTIPA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLVGETINFOTIPA {}
impl ::core::default::Default for NMLVGETINFOTIPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMLVGETINFOTIPW {
    pub hdr: NMHDR,
    pub dwFlags: u32,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMLVGETINFOTIPW {}
impl ::core::clone::Clone for NMLVGETINFOTIPW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLVGETINFOTIPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVGETINFOTIPW").field("hdr", &self.hdr).field("dwFlags", &self.dwFlags).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLVGETINFOTIPW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLVGETINFOTIPW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVGETINFOTIPW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLVGETINFOTIPW {}
impl ::core::default::Default for NMLVGETINFOTIPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct NMLVKEYDOWN {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
impl ::core::marker::Copy for NMLVKEYDOWN {}
impl ::core::clone::Clone for NMLVKEYDOWN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for NMLVKEYDOWN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLVKEYDOWN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVKEYDOWN>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLVKEYDOWN {}
impl ::core::default::Default for NMLVKEYDOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMLVLINK {
    pub hdr: NMHDR,
    pub link: LITEM,
    pub iItem: i32,
    pub iSubItem: i32,
}
impl ::core::marker::Copy for NMLVLINK {}
impl ::core::clone::Clone for NMLVLINK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLVLINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVLINK").field("hdr", &self.hdr).field("link", &self.link).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLVLINK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLVLINK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVLINK>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLVLINK {}
impl ::core::default::Default for NMLVLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMLVODSTATECHANGE {
    pub hdr: NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
    pub uNewState: u32,
    pub uOldState: u32,
}
impl ::core::marker::Copy for NMLVODSTATECHANGE {}
impl ::core::clone::Clone for NMLVODSTATECHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLVODSTATECHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVODSTATECHANGE").field("hdr", &self.hdr).field("iFrom", &self.iFrom).field("iTo", &self.iTo).field("uNewState", &self.uNewState).field("uOldState", &self.uOldState).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLVODSTATECHANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLVODSTATECHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVODSTATECHANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLVODSTATECHANGE {}
impl ::core::default::Default for NMLVODSTATECHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMLVSCROLL {
    pub hdr: NMHDR,
    pub dx: i32,
    pub dy: i32,
}
impl ::core::marker::Copy for NMLVSCROLL {}
impl ::core::clone::Clone for NMLVSCROLL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMLVSCROLL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMLVSCROLL").field("hdr", &self.hdr).field("dx", &self.dx).field("dy", &self.dy).finish()
    }
}
unsafe impl ::windows_core::Abi for NMLVSCROLL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMLVSCROLL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMLVSCROLL>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMLVSCROLL {}
impl ::core::default::Default for NMLVSCROLL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMMOUSE {
    pub hdr: NMHDR,
    pub dwItemSpec: usize,
    pub dwItemData: usize,
    pub pt: ::win32_foundation::POINT,
    pub dwHitInfo: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMMOUSE {}
impl ::core::clone::Clone for NMMOUSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMMOUSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMMOUSE").field("hdr", &self.hdr).field("dwItemSpec", &self.dwItemSpec).field("dwItemData", &self.dwItemData).field("pt", &self.pt).field("dwHitInfo", &self.dwHitInfo).finish()
    }
}
unsafe impl ::windows_core::Abi for NMMOUSE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMMOUSE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMMOUSE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMMOUSE {}
impl ::core::default::Default for NMMOUSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMOBJECTNOTIFY {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub piid: *const ::windows_core::GUID,
    pub pObject: *mut ::core::ffi::c_void,
    pub hResult: ::windows_core::HRESULT,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for NMOBJECTNOTIFY {}
impl ::core::clone::Clone for NMOBJECTNOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMOBJECTNOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMOBJECTNOTIFY").field("hdr", &self.hdr).field("iItem", &self.iItem).field("piid", &self.piid).field("pObject", &self.pObject).field("hResult", &self.hResult).field("dwFlags", &self.dwFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for NMOBJECTNOTIFY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMOBJECTNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMOBJECTNOTIFY>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMOBJECTNOTIFY {}
impl ::core::default::Default for NMOBJECTNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMPGCALCSIZE {
    pub hdr: NMHDR,
    pub dwFlag: NMPGCALCSIZE_FLAGS,
    pub iWidth: i32,
    pub iHeight: i32,
}
impl ::core::marker::Copy for NMPGCALCSIZE {}
impl ::core::clone::Clone for NMPGCALCSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMPGCALCSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMPGCALCSIZE").field("hdr", &self.hdr).field("dwFlag", &self.dwFlag).field("iWidth", &self.iWidth).field("iHeight", &self.iHeight).finish()
    }
}
unsafe impl ::windows_core::Abi for NMPGCALCSIZE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMPGCALCSIZE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMPGCALCSIZE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMPGCALCSIZE {}
impl ::core::default::Default for NMPGCALCSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NMPGCALCSIZE_FLAGS(pub u32);
pub const PGF_CALCHEIGHT: NMPGCALCSIZE_FLAGS = NMPGCALCSIZE_FLAGS(2u32);
pub const PGF_CALCWIDTH: NMPGCALCSIZE_FLAGS = NMPGCALCSIZE_FLAGS(1u32);
impl ::core::marker::Copy for NMPGCALCSIZE_FLAGS {}
impl ::core::clone::Clone for NMPGCALCSIZE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NMPGCALCSIZE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NMPGCALCSIZE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NMPGCALCSIZE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMPGCALCSIZE_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct NMPGHOTITEM {
    pub hdr: NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for NMPGHOTITEM {}
impl ::core::clone::Clone for NMPGHOTITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMPGHOTITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMPGHOTITEM").field("hdr", &self.hdr).field("idOld", &self.idOld).field("idNew", &self.idNew).field("dwFlags", &self.dwFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for NMPGHOTITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMPGHOTITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMPGHOTITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMPGHOTITEM {}
impl ::core::default::Default for NMPGHOTITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct NMPGSCROLL {
    pub hdr: NMHDR,
    pub fwKeys: NMPGSCROLL_KEYS,
    pub rcParent: ::win32_foundation::RECT,
    pub iDir: NMPGSCROLL_DIR,
    pub iXpos: i32,
    pub iYpos: i32,
    pub iScroll: i32,
}
impl ::core::marker::Copy for NMPGSCROLL {}
impl ::core::clone::Clone for NMPGSCROLL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for NMPGSCROLL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMPGSCROLL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMPGSCROLL>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMPGSCROLL {}
impl ::core::default::Default for NMPGSCROLL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NMPGSCROLL_DIR(pub u32);
pub const PGF_SCROLLDOWN: NMPGSCROLL_DIR = NMPGSCROLL_DIR(2u32);
pub const PGF_SCROLLLEFT: NMPGSCROLL_DIR = NMPGSCROLL_DIR(4u32);
pub const PGF_SCROLLRIGHT: NMPGSCROLL_DIR = NMPGSCROLL_DIR(8u32);
pub const PGF_SCROLLUP: NMPGSCROLL_DIR = NMPGSCROLL_DIR(1u32);
impl ::core::marker::Copy for NMPGSCROLL_DIR {}
impl ::core::clone::Clone for NMPGSCROLL_DIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NMPGSCROLL_DIR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NMPGSCROLL_DIR {
    type Abi = Self;
}
impl ::core::fmt::Debug for NMPGSCROLL_DIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMPGSCROLL_DIR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NMPGSCROLL_KEYS(pub u16);
pub const PGK_NONE: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(0u16);
pub const PGK_SHIFT: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(1u16);
pub const PGK_CONTROL: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(2u16);
pub const PGK_MENU: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(4u16);
impl ::core::marker::Copy for NMPGSCROLL_KEYS {}
impl ::core::clone::Clone for NMPGSCROLL_KEYS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NMPGSCROLL_KEYS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NMPGSCROLL_KEYS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NMPGSCROLL_KEYS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMPGSCROLL_KEYS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NMPGSCROLL_KEYS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NMPGSCROLL_KEYS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NMPGSCROLL_KEYS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NMPGSCROLL_KEYS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NMPGSCROLL_KEYS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
pub struct NMRBAUTOSIZE {
    pub hdr: NMHDR,
    pub fChanged: ::win32_foundation::BOOL,
    pub rcTarget: ::win32_foundation::RECT,
    pub rcActual: ::win32_foundation::RECT,
}
impl ::core::marker::Copy for NMRBAUTOSIZE {}
impl ::core::clone::Clone for NMRBAUTOSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMRBAUTOSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMRBAUTOSIZE").field("hdr", &self.hdr).field("fChanged", &self.fChanged).field("rcTarget", &self.rcTarget).field("rcActual", &self.rcActual).finish()
    }
}
unsafe impl ::windows_core::Abi for NMRBAUTOSIZE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMRBAUTOSIZE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMRBAUTOSIZE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMRBAUTOSIZE {}
impl ::core::default::Default for NMRBAUTOSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMREBAR {
    pub hdr: NMHDR,
    pub dwMask: NMREBAR_MASK_FLAGS,
    pub uBand: u32,
    pub fStyle: u32,
    pub wID: u32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMREBAR {}
impl ::core::clone::Clone for NMREBAR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMREBAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMREBAR").field("hdr", &self.hdr).field("dwMask", &self.dwMask).field("uBand", &self.uBand).field("fStyle", &self.fStyle).field("wID", &self.wID).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMREBAR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMREBAR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMREBAR>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMREBAR {}
impl ::core::default::Default for NMREBAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMREBARAUTOBREAK {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: ::win32_foundation::LPARAM,
    pub uMsg: u32,
    pub fStyleCurrent: u32,
    pub fAutoBreak: ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for NMREBARAUTOBREAK {}
impl ::core::clone::Clone for NMREBARAUTOBREAK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMREBARAUTOBREAK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMREBARAUTOBREAK").field("hdr", &self.hdr).field("uBand", &self.uBand).field("wID", &self.wID).field("lParam", &self.lParam).field("uMsg", &self.uMsg).field("fStyleCurrent", &self.fStyleCurrent).field("fAutoBreak", &self.fAutoBreak).finish()
    }
}
unsafe impl ::windows_core::Abi for NMREBARAUTOBREAK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMREBARAUTOBREAK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMREBARAUTOBREAK>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMREBARAUTOBREAK {}
impl ::core::default::Default for NMREBARAUTOBREAK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMREBARCHEVRON {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: ::win32_foundation::LPARAM,
    pub rc: ::win32_foundation::RECT,
    pub lParamNM: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMREBARCHEVRON {}
impl ::core::clone::Clone for NMREBARCHEVRON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMREBARCHEVRON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMREBARCHEVRON").field("hdr", &self.hdr).field("uBand", &self.uBand).field("wID", &self.wID).field("lParam", &self.lParam).field("rc", &self.rc).field("lParamNM", &self.lParamNM).finish()
    }
}
unsafe impl ::windows_core::Abi for NMREBARCHEVRON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMREBARCHEVRON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMREBARCHEVRON>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMREBARCHEVRON {}
impl ::core::default::Default for NMREBARCHEVRON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMREBARCHILDSIZE {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub rcChild: ::win32_foundation::RECT,
    pub rcBand: ::win32_foundation::RECT,
}
impl ::core::marker::Copy for NMREBARCHILDSIZE {}
impl ::core::clone::Clone for NMREBARCHILDSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMREBARCHILDSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMREBARCHILDSIZE").field("hdr", &self.hdr).field("uBand", &self.uBand).field("wID", &self.wID).field("rcChild", &self.rcChild).field("rcBand", &self.rcBand).finish()
    }
}
unsafe impl ::windows_core::Abi for NMREBARCHILDSIZE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMREBARCHILDSIZE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMREBARCHILDSIZE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMREBARCHILDSIZE {}
impl ::core::default::Default for NMREBARCHILDSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMREBARSPLITTER {
    pub hdr: NMHDR,
    pub rcSizing: ::win32_foundation::RECT,
}
impl ::core::marker::Copy for NMREBARSPLITTER {}
impl ::core::clone::Clone for NMREBARSPLITTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMREBARSPLITTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMREBARSPLITTER").field("hdr", &self.hdr).field("rcSizing", &self.rcSizing).finish()
    }
}
unsafe impl ::windows_core::Abi for NMREBARSPLITTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMREBARSPLITTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMREBARSPLITTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMREBARSPLITTER {}
impl ::core::default::Default for NMREBARSPLITTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NMREBAR_MASK_FLAGS(pub u32);
pub const RBNM_ID: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(1u32);
pub const RBNM_LPARAM: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(4u32);
pub const RBNM_STYLE: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(2u32);
impl ::core::marker::Copy for NMREBAR_MASK_FLAGS {}
impl ::core::clone::Clone for NMREBAR_MASK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NMREBAR_MASK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NMREBAR_MASK_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NMREBAR_MASK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMREBAR_MASK_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NMREBAR_MASK_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NMREBAR_MASK_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NMREBAR_MASK_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NMREBAR_MASK_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NMREBAR_MASK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
pub struct NMSEARCHWEB {
    pub hdr: NMHDR,
    pub entrypoint: EC_SEARCHWEB_ENTRYPOINT,
    pub hasQueryText: ::win32_foundation::BOOL,
    pub invokeSucceeded: ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for NMSEARCHWEB {}
impl ::core::clone::Clone for NMSEARCHWEB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMSEARCHWEB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMSEARCHWEB").field("hdr", &self.hdr).field("entrypoint", &self.entrypoint).field("hasQueryText", &self.hasQueryText).field("invokeSucceeded", &self.invokeSucceeded).finish()
    }
}
unsafe impl ::windows_core::Abi for NMSEARCHWEB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMSEARCHWEB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMSEARCHWEB>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMSEARCHWEB {}
impl ::core::default::Default for NMSEARCHWEB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMSELCHANGE {
    pub nmhdr: NMHDR,
    pub stSelStart: ::win32_foundation::SYSTEMTIME,
    pub stSelEnd: ::win32_foundation::SYSTEMTIME,
}
impl ::core::marker::Copy for NMSELCHANGE {}
impl ::core::clone::Clone for NMSELCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMSELCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMSELCHANGE").field("nmhdr", &self.nmhdr).field("stSelStart", &self.stSelStart).field("stSelEnd", &self.stSelEnd).finish()
    }
}
unsafe impl ::windows_core::Abi for NMSELCHANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMSELCHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMSELCHANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMSELCHANGE {}
impl ::core::default::Default for NMSELCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct NMTBCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub hbrMonoDither: ::win32_graphics::Gdi::HBRUSH,
    pub hbrLines: ::win32_graphics::Gdi::HBRUSH,
    pub hpenLines: ::win32_graphics::Gdi::HPEN,
    pub clrText: u32,
    pub clrMark: u32,
    pub clrTextHighlight: u32,
    pub clrBtnFace: u32,
    pub clrBtnHighlight: u32,
    pub clrHighlightHotTrack: u32,
    pub rcText: ::win32_foundation::RECT,
    pub nStringBkMode: i32,
    pub nHLStringBkMode: i32,
    pub iListGap: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for NMTBCUSTOMDRAW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for NMTBCUSTOMDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for NMTBCUSTOMDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBCUSTOMDRAW")
            .field("nmcd", &self.nmcd)
            .field("hbrMonoDither", &self.hbrMonoDither)
            .field("hbrLines", &self.hbrLines)
            .field("hpenLines", &self.hpenLines)
            .field("clrText", &self.clrText)
            .field("clrMark", &self.clrMark)
            .field("clrTextHighlight", &self.clrTextHighlight)
            .field("clrBtnFace", &self.clrBtnFace)
            .field("clrBtnHighlight", &self.clrBtnHighlight)
            .field("clrHighlightHotTrack", &self.clrHighlightHotTrack)
            .field("rcText", &self.rcText)
            .field("nStringBkMode", &self.nStringBkMode)
            .field("nHLStringBkMode", &self.nHLStringBkMode)
            .field("iListGap", &self.iListGap)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for NMTBCUSTOMDRAW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NMTBCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTBCUSTOMDRAW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NMTBCUSTOMDRAW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NMTBCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTBDISPINFOA {
    pub hdr: NMHDR,
    pub dwMask: NMTBDISPINFOW_MASK,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: ::windows_core::PSTR,
    pub cchText: i32,
}
impl ::core::marker::Copy for NMTBDISPINFOA {}
impl ::core::clone::Clone for NMTBDISPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTBDISPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBDISPINFOA").field("hdr", &self.hdr).field("dwMask", &self.dwMask).field("idCommand", &self.idCommand).field("lParam", &self.lParam).field("iImage", &self.iImage).field("pszText", &self.pszText).field("cchText", &self.cchText).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTBDISPINFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTBDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTBDISPINFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTBDISPINFOA {}
impl ::core::default::Default for NMTBDISPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTBDISPINFOW {
    pub hdr: NMHDR,
    pub dwMask: NMTBDISPINFOW_MASK,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: ::windows_core::PWSTR,
    pub cchText: i32,
}
impl ::core::marker::Copy for NMTBDISPINFOW {}
impl ::core::clone::Clone for NMTBDISPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTBDISPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBDISPINFOW").field("hdr", &self.hdr).field("dwMask", &self.dwMask).field("idCommand", &self.idCommand).field("lParam", &self.lParam).field("iImage", &self.iImage).field("pszText", &self.pszText).field("cchText", &self.cchText).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTBDISPINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTBDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTBDISPINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTBDISPINFOW {}
impl ::core::default::Default for NMTBDISPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NMTBDISPINFOW_MASK(pub u32);
pub const TBNF_IMAGE: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(1u32);
pub const TBNF_TEXT: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(2u32);
pub const TBNF_DI_SETITEM: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(268435456u32);
impl ::core::marker::Copy for NMTBDISPINFOW_MASK {}
impl ::core::clone::Clone for NMTBDISPINFOW_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NMTBDISPINFOW_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NMTBDISPINFOW_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for NMTBDISPINFOW_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMTBDISPINFOW_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NMTBDISPINFOW_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NMTBDISPINFOW_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NMTBDISPINFOW_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NMTBDISPINFOW_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NMTBDISPINFOW_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
pub struct NMTBGETINFOTIPA {
    pub hdr: NMHDR,
    pub pszText: ::windows_core::PSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMTBGETINFOTIPA {}
impl ::core::clone::Clone for NMTBGETINFOTIPA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTBGETINFOTIPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBGETINFOTIPA").field("hdr", &self.hdr).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iItem", &self.iItem).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTBGETINFOTIPA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTBGETINFOTIPA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTBGETINFOTIPA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTBGETINFOTIPA {}
impl ::core::default::Default for NMTBGETINFOTIPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTBGETINFOTIPW {
    pub hdr: NMHDR,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMTBGETINFOTIPW {}
impl ::core::clone::Clone for NMTBGETINFOTIPW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTBGETINFOTIPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBGETINFOTIPW").field("hdr", &self.hdr).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iItem", &self.iItem).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTBGETINFOTIPW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTBGETINFOTIPW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTBGETINFOTIPW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTBGETINFOTIPW {}
impl ::core::default::Default for NMTBGETINFOTIPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTBHOTITEM {
    pub hdr: NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: NMTBHOTITEM_FLAGS,
}
impl ::core::marker::Copy for NMTBHOTITEM {}
impl ::core::clone::Clone for NMTBHOTITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTBHOTITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBHOTITEM").field("hdr", &self.hdr).field("idOld", &self.idOld).field("idNew", &self.idNew).field("dwFlags", &self.dwFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTBHOTITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTBHOTITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTBHOTITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTBHOTITEM {}
impl ::core::default::Default for NMTBHOTITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NMTBHOTITEM_FLAGS(pub u32);
pub const HICF_ACCELERATOR: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(4u32);
pub const HICF_ARROWKEYS: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(2u32);
pub const HICF_DUPACCEL: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(8u32);
pub const HICF_ENTERING: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(16u32);
pub const HICF_LEAVING: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(32u32);
pub const HICF_LMOUSE: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(128u32);
pub const HICF_MOUSE: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(1u32);
pub const HICF_OTHER: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(0u32);
pub const HICF_RESELECT: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(64u32);
pub const HICF_TOGGLEDROPDOWN: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(256u32);
impl ::core::marker::Copy for NMTBHOTITEM_FLAGS {}
impl ::core::clone::Clone for NMTBHOTITEM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NMTBHOTITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NMTBHOTITEM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NMTBHOTITEM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NMTBHOTITEM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NMTBHOTITEM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NMTBHOTITEM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NMTBHOTITEM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NMTBHOTITEM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NMTBHOTITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
pub struct NMTBRESTORE {
    pub hdr: NMHDR,
    pub pData: *mut u32,
    pub pCurrent: *mut u32,
    pub cbData: u32,
    pub iItem: i32,
    pub cButtons: i32,
    pub cbBytesPerRecord: i32,
    pub tbButton: TBBUTTON,
}
impl ::core::marker::Copy for NMTBRESTORE {}
impl ::core::clone::Clone for NMTBRESTORE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTBRESTORE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBRESTORE").field("hdr", &self.hdr).field("pData", &self.pData).field("pCurrent", &self.pCurrent).field("cbData", &self.cbData).field("iItem", &self.iItem).field("cButtons", &self.cButtons).field("cbBytesPerRecord", &self.cbBytesPerRecord).field("tbButton", &self.tbButton).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTBRESTORE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTBRESTORE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTBRESTORE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTBRESTORE {}
impl ::core::default::Default for NMTBRESTORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTBSAVE {
    pub hdr: NMHDR,
    pub pData: *mut u32,
    pub pCurrent: *mut u32,
    pub cbData: u32,
    pub iItem: i32,
    pub cButtons: i32,
    pub tbButton: TBBUTTON,
}
impl ::core::marker::Copy for NMTBSAVE {}
impl ::core::clone::Clone for NMTBSAVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTBSAVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTBSAVE").field("hdr", &self.hdr).field("pData", &self.pData).field("pCurrent", &self.pCurrent).field("cbData", &self.cbData).field("iItem", &self.iItem).field("cButtons", &self.cButtons).field("tbButton", &self.tbButton).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTBSAVE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTBSAVE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTBSAVE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTBSAVE {}
impl ::core::default::Default for NMTBSAVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct NMTCKEYDOWN {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
impl ::core::marker::Copy for NMTCKEYDOWN {}
impl ::core::clone::Clone for NMTCKEYDOWN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for NMTCKEYDOWN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTCKEYDOWN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTCKEYDOWN>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTCKEYDOWN {}
impl ::core::default::Default for NMTCKEYDOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTOOLBARA {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: ::windows_core::PSTR,
    pub rcButton: ::win32_foundation::RECT,
}
impl ::core::marker::Copy for NMTOOLBARA {}
impl ::core::clone::Clone for NMTOOLBARA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTOOLBARA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTOOLBARA").field("hdr", &self.hdr).field("iItem", &self.iItem).field("tbButton", &self.tbButton).field("cchText", &self.cchText).field("pszText", &self.pszText).field("rcButton", &self.rcButton).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTOOLBARA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTOOLBARA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTOOLBARA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTOOLBARA {}
impl ::core::default::Default for NMTOOLBARA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTOOLBARW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: ::windows_core::PWSTR,
    pub rcButton: ::win32_foundation::RECT,
}
impl ::core::marker::Copy for NMTOOLBARW {}
impl ::core::clone::Clone for NMTOOLBARW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTOOLBARW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTOOLBARW").field("hdr", &self.hdr).field("iItem", &self.iItem).field("tbButton", &self.tbButton).field("cchText", &self.cchText).field("pszText", &self.pszText).field("rcButton", &self.rcButton).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTOOLBARW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTOOLBARW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTOOLBARW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTOOLBARW {}
impl ::core::default::Default for NMTOOLBARW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTOOLTIPSCREATED {
    pub hdr: NMHDR,
    pub hwndToolTips: ::win32_foundation::HWND,
}
impl ::core::marker::Copy for NMTOOLTIPSCREATED {}
impl ::core::clone::Clone for NMTOOLTIPSCREATED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTOOLTIPSCREATED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTOOLTIPSCREATED").field("hdr", &self.hdr).field("hwndToolTips", &self.hwndToolTips).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTOOLTIPSCREATED {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTOOLTIPSCREATED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTOOLTIPSCREATED>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTOOLTIPSCREATED {}
impl ::core::default::Default for NMTOOLTIPSCREATED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTRBTHUMBPOSCHANGING {
    pub hdr: NMHDR,
    pub dwPos: u32,
    pub nReason: i32,
}
impl ::core::marker::Copy for NMTRBTHUMBPOSCHANGING {}
impl ::core::clone::Clone for NMTRBTHUMBPOSCHANGING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTRBTHUMBPOSCHANGING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTRBTHUMBPOSCHANGING").field("hdr", &self.hdr).field("dwPos", &self.dwPos).field("nReason", &self.nReason).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTRBTHUMBPOSCHANGING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTRBTHUMBPOSCHANGING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTRBTHUMBPOSCHANGING>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTRBTHUMBPOSCHANGING {}
impl ::core::default::Default for NMTRBTHUMBPOSCHANGING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTREEVIEWA {
    pub hdr: NMHDR,
    pub action: u32,
    pub itemOld: TVITEMA,
    pub itemNew: TVITEMA,
    pub ptDrag: ::win32_foundation::POINT,
}
impl ::core::marker::Copy for NMTREEVIEWA {}
impl ::core::clone::Clone for NMTREEVIEWA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTREEVIEWA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTREEVIEWA").field("hdr", &self.hdr).field("action", &self.action).field("itemOld", &self.itemOld).field("itemNew", &self.itemNew).field("ptDrag", &self.ptDrag).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTREEVIEWA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTREEVIEWA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTREEVIEWA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTREEVIEWA {}
impl ::core::default::Default for NMTREEVIEWA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTREEVIEWW {
    pub hdr: NMHDR,
    pub action: u32,
    pub itemOld: TVITEMW,
    pub itemNew: TVITEMW,
    pub ptDrag: ::win32_foundation::POINT,
}
impl ::core::marker::Copy for NMTREEVIEWW {}
impl ::core::clone::Clone for NMTREEVIEWW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTREEVIEWW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTREEVIEWW").field("hdr", &self.hdr).field("action", &self.action).field("itemOld", &self.itemOld).field("itemNew", &self.itemNew).field("ptDrag", &self.ptDrag).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTREEVIEWW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTREEVIEWW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTREEVIEWW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTREEVIEWW {}
impl ::core::default::Default for NMTREEVIEWW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct NMTTCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub uDrawFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for NMTTCUSTOMDRAW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for NMTTCUSTOMDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for NMTTCUSTOMDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTTCUSTOMDRAW").field("nmcd", &self.nmcd).field("uDrawFlags", &self.uDrawFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for NMTTCUSTOMDRAW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NMTTCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTTCUSTOMDRAW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NMTTCUSTOMDRAW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NMTTCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTTDISPINFOA {
    pub hdr: NMHDR,
    pub lpszText: ::windows_core::PSTR,
    pub szText: [::win32_foundation::CHAR; 80],
    pub hinst: ::win32_foundation::HINSTANCE,
    pub uFlags: u32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMTTDISPINFOA {}
impl ::core::clone::Clone for NMTTDISPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTTDISPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTTDISPINFOA").field("hdr", &self.hdr).field("lpszText", &self.lpszText).field("szText", &self.szText).field("hinst", &self.hinst).field("uFlags", &self.uFlags).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTTDISPINFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTTDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTTDISPINFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTTDISPINFOA {}
impl ::core::default::Default for NMTTDISPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTTDISPINFOW {
    pub hdr: NMHDR,
    pub lpszText: ::windows_core::PWSTR,
    pub szText: [u16; 80],
    pub hinst: ::win32_foundation::HINSTANCE,
    pub uFlags: u32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMTTDISPINFOW {}
impl ::core::clone::Clone for NMTTDISPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTTDISPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTTDISPINFOW").field("hdr", &self.hdr).field("lpszText", &self.lpszText).field("szText", &self.szText).field("hinst", &self.hinst).field("uFlags", &self.uFlags).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTTDISPINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTTDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTTDISPINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTTDISPINFOW {}
impl ::core::default::Default for NMTTDISPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct NMTVASYNCDRAW {
    pub hdr: NMHDR,
    pub pimldp: *mut IMAGELISTDRAWPARAMS,
    pub hr: ::windows_core::HRESULT,
    pub hItem: HTREEITEM,
    pub lParam: ::win32_foundation::LPARAM,
    pub dwRetFlags: u32,
    pub iRetImageIndex: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for NMTVASYNCDRAW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for NMTVASYNCDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for NMTVASYNCDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVASYNCDRAW").field("hdr", &self.hdr).field("pimldp", &self.pimldp).field("hr", &self.hr).field("hItem", &self.hItem).field("lParam", &self.lParam).field("dwRetFlags", &self.dwRetFlags).field("iRetImageIndex", &self.iRetImageIndex).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for NMTVASYNCDRAW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NMTVASYNCDRAW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTVASYNCDRAW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NMTVASYNCDRAW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NMTVASYNCDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct NMTVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: u32,
    pub clrTextBk: u32,
    pub iLevel: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for NMTVCUSTOMDRAW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for NMTVCUSTOMDRAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for NMTVCUSTOMDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVCUSTOMDRAW").field("nmcd", &self.nmcd).field("clrText", &self.clrText).field("clrTextBk", &self.clrTextBk).field("iLevel", &self.iLevel).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for NMTVCUSTOMDRAW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NMTVCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTVCUSTOMDRAW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NMTVCUSTOMDRAW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NMTVCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTVDISPINFOA {
    pub hdr: NMHDR,
    pub item: TVITEMA,
}
impl ::core::marker::Copy for NMTVDISPINFOA {}
impl ::core::clone::Clone for NMTVDISPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTVDISPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVDISPINFOA").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTVDISPINFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTVDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTVDISPINFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTVDISPINFOA {}
impl ::core::default::Default for NMTVDISPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTVDISPINFOEXA {
    pub hdr: NMHDR,
    pub item: TVITEMEXA,
}
impl ::core::marker::Copy for NMTVDISPINFOEXA {}
impl ::core::clone::Clone for NMTVDISPINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTVDISPINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVDISPINFOEXA").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTVDISPINFOEXA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTVDISPINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTVDISPINFOEXA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTVDISPINFOEXA {}
impl ::core::default::Default for NMTVDISPINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTVDISPINFOEXW {
    pub hdr: NMHDR,
    pub item: TVITEMEXW,
}
impl ::core::marker::Copy for NMTVDISPINFOEXW {}
impl ::core::clone::Clone for NMTVDISPINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTVDISPINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVDISPINFOEXW").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTVDISPINFOEXW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTVDISPINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTVDISPINFOEXW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTVDISPINFOEXW {}
impl ::core::default::Default for NMTVDISPINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTVDISPINFOW {
    pub hdr: NMHDR,
    pub item: TVITEMW,
}
impl ::core::marker::Copy for NMTVDISPINFOW {}
impl ::core::clone::Clone for NMTVDISPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTVDISPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVDISPINFOW").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTVDISPINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTVDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTVDISPINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTVDISPINFOW {}
impl ::core::default::Default for NMTVDISPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTVGETINFOTIPA {
    pub hdr: NMHDR,
    pub pszText: ::windows_core::PSTR,
    pub cchTextMax: i32,
    pub hItem: HTREEITEM,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMTVGETINFOTIPA {}
impl ::core::clone::Clone for NMTVGETINFOTIPA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTVGETINFOTIPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVGETINFOTIPA").field("hdr", &self.hdr).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("hItem", &self.hItem).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTVGETINFOTIPA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTVGETINFOTIPA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTVGETINFOTIPA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTVGETINFOTIPA {}
impl ::core::default::Default for NMTVGETINFOTIPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTVGETINFOTIPW {
    pub hdr: NMHDR,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub hItem: HTREEITEM,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMTVGETINFOTIPW {}
impl ::core::clone::Clone for NMTVGETINFOTIPW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTVGETINFOTIPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVGETINFOTIPW").field("hdr", &self.hdr).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("hItem", &self.hItem).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTVGETINFOTIPW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTVGETINFOTIPW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTVGETINFOTIPW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTVGETINFOTIPW {}
impl ::core::default::Default for NMTVGETINFOTIPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTVITEMCHANGE {
    pub hdr: NMHDR,
    pub uChanged: u32,
    pub hItem: HTREEITEM,
    pub uStateNew: u32,
    pub uStateOld: u32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for NMTVITEMCHANGE {}
impl ::core::clone::Clone for NMTVITEMCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTVITEMCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVITEMCHANGE").field("hdr", &self.hdr).field("uChanged", &self.uChanged).field("hItem", &self.hItem).field("uStateNew", &self.uStateNew).field("uStateOld", &self.uStateOld).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTVITEMCHANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTVITEMCHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTVITEMCHANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTVITEMCHANGE {}
impl ::core::default::Default for NMTVITEMCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct NMTVKEYDOWN {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
impl ::core::marker::Copy for NMTVKEYDOWN {}
impl ::core::clone::Clone for NMTVKEYDOWN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for NMTVKEYDOWN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTVKEYDOWN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTVKEYDOWN>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTVKEYDOWN {}
impl ::core::default::Default for NMTVKEYDOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMTVSTATEIMAGECHANGING {
    pub hdr: NMHDR,
    pub hti: HTREEITEM,
    pub iOldStateImageIndex: i32,
    pub iNewStateImageIndex: i32,
}
impl ::core::marker::Copy for NMTVSTATEIMAGECHANGING {}
impl ::core::clone::Clone for NMTVSTATEIMAGECHANGING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMTVSTATEIMAGECHANGING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMTVSTATEIMAGECHANGING").field("hdr", &self.hdr).field("hti", &self.hti).field("iOldStateImageIndex", &self.iOldStateImageIndex).field("iNewStateImageIndex", &self.iNewStateImageIndex).finish()
    }
}
unsafe impl ::windows_core::Abi for NMTVSTATEIMAGECHANGING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMTVSTATEIMAGECHANGING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMTVSTATEIMAGECHANGING>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMTVSTATEIMAGECHANGING {}
impl ::core::default::Default for NMTVSTATEIMAGECHANGING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMUPDOWN {
    pub hdr: NMHDR,
    pub iPos: i32,
    pub iDelta: i32,
}
impl ::core::marker::Copy for NMUPDOWN {}
impl ::core::clone::Clone for NMUPDOWN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMUPDOWN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMUPDOWN").field("hdr", &self.hdr).field("iPos", &self.iPos).field("iDelta", &self.iDelta).finish()
    }
}
unsafe impl ::windows_core::Abi for NMUPDOWN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMUPDOWN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMUPDOWN>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMUPDOWN {}
impl ::core::default::Default for NMUPDOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NMVIEWCHANGE {
    pub nmhdr: NMHDR,
    pub dwOldView: u32,
    pub dwNewView: u32,
}
impl ::core::marker::Copy for NMVIEWCHANGE {}
impl ::core::clone::Clone for NMVIEWCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NMVIEWCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NMVIEWCHANGE").field("nmhdr", &self.nmhdr).field("dwOldView", &self.dwOldView).field("dwNewView", &self.dwNewView).finish()
    }
}
unsafe impl ::windows_core::Abi for NMVIEWCHANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NMVIEWCHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NMVIEWCHANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NMVIEWCHANGE {}
impl ::core::default::Default for NMVIEWCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const NM_GETCUSTOMSPLITRECT: u32 = 4294966049u32;
pub const ODT_HEADER: u32 = 100u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OFFSETTYPE(pub i32);
pub const OT_TOPLEFT: OFFSETTYPE = OFFSETTYPE(0i32);
pub const OT_TOPRIGHT: OFFSETTYPE = OFFSETTYPE(1i32);
pub const OT_TOPMIDDLE: OFFSETTYPE = OFFSETTYPE(2i32);
pub const OT_BOTTOMLEFT: OFFSETTYPE = OFFSETTYPE(3i32);
pub const OT_BOTTOMRIGHT: OFFSETTYPE = OFFSETTYPE(4i32);
pub const OT_BOTTOMMIDDLE: OFFSETTYPE = OFFSETTYPE(5i32);
pub const OT_MIDDLELEFT: OFFSETTYPE = OFFSETTYPE(6i32);
pub const OT_MIDDLERIGHT: OFFSETTYPE = OFFSETTYPE(7i32);
pub const OT_LEFTOFCAPTION: OFFSETTYPE = OFFSETTYPE(8i32);
pub const OT_RIGHTOFCAPTION: OFFSETTYPE = OFFSETTYPE(9i32);
pub const OT_LEFTOFLASTBUTTON: OFFSETTYPE = OFFSETTYPE(10i32);
pub const OT_RIGHTOFLASTBUTTON: OFFSETTYPE = OFFSETTYPE(11i32);
pub const OT_ABOVELASTBUTTON: OFFSETTYPE = OFFSETTYPE(12i32);
pub const OT_BELOWLASTBUTTON: OFFSETTYPE = OFFSETTYPE(13i32);
impl ::core::marker::Copy for OFFSETTYPE {}
impl ::core::clone::Clone for OFFSETTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFSETTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for OFFSETTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for OFFSETTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFSETTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OPENBOXSTATES(pub i32);
pub const SPOB_NORMAL: OPENBOXSTATES = OPENBOXSTATES(1i32);
pub const SPOB_HOT: OPENBOXSTATES = OPENBOXSTATES(2i32);
pub const SPOB_SELECTED: OPENBOXSTATES = OPENBOXSTATES(3i32);
pub const SPOB_DISABLED: OPENBOXSTATES = OPENBOXSTATES(4i32);
pub const SPOB_FOCUSED: OPENBOXSTATES = OPENBOXSTATES(5i32);
impl ::core::marker::Copy for OPENBOXSTATES {}
impl ::core::clone::Clone for OPENBOXSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPENBOXSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for OPENBOXSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for OPENBOXSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPENBOXSTATES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OPEN_THEME_DATA_FLAGS(pub u32);
pub const OTD_FORCE_RECT_SIZING: OPEN_THEME_DATA_FLAGS = OPEN_THEME_DATA_FLAGS(1u32);
pub const OTD_NONCLIENT: OPEN_THEME_DATA_FLAGS = OPEN_THEME_DATA_FLAGS(2u32);
impl ::core::marker::Copy for OPEN_THEME_DATA_FLAGS {}
impl ::core::clone::Clone for OPEN_THEME_DATA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPEN_THEME_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for OPEN_THEME_DATA_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for OPEN_THEME_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPEN_THEME_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for OPEN_THEME_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OPEN_THEME_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OPEN_THEME_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OPEN_THEME_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OPEN_THEME_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn OpenThemeData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hwnd: Param0, pszclasslist: Param1) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenThemeData(hwnd: ::win32_foundation::HWND, pszclasslist: ::windows_core::PCWSTR) -> isize;
        }
        ::core::mem::transmute(OpenThemeData(hwnd.into_param().abi(), pszclasslist.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn OpenThemeDataEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hwnd: Param0, pszclasslist: Param1, dwflags: OPEN_THEME_DATA_FLAGS) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenThemeDataEx(hwnd: ::win32_foundation::HWND, pszclasslist: ::windows_core::PCWSTR, dwflags: OPEN_THEME_DATA_FLAGS) -> isize;
        }
        ::core::mem::transmute(OpenThemeDataEx(hwnd.into_param().abi(), pszclasslist.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PAGEPARTS(pub i32);
pub const PGRP_UP: PAGEPARTS = PAGEPARTS(1i32);
pub const PGRP_DOWN: PAGEPARTS = PAGEPARTS(2i32);
pub const PGRP_UPHORZ: PAGEPARTS = PAGEPARTS(3i32);
pub const PGRP_DOWNHORZ: PAGEPARTS = PAGEPARTS(4i32);
impl ::core::marker::Copy for PAGEPARTS {}
impl ::core::clone::Clone for PAGEPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PAGEPARTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PAGEPARTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PAGEPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGEPARTS").field(&self.0).finish()
    }
}
pub const PAGESETUPDLGORD: u32 = 1546u32;
pub const PAGESETUPDLGORDMOTIF: u32 = 1550u32;
pub const PBM_DELTAPOS: u32 = 1027u32;
pub const PBM_GETBARCOLOR: u32 = 1039u32;
pub const PBM_GETBKCOLOR: u32 = 1038u32;
pub const PBM_GETPOS: u32 = 1032u32;
pub const PBM_GETRANGE: u32 = 1031u32;
pub const PBM_GETSTATE: u32 = 1041u32;
pub const PBM_GETSTEP: u32 = 1037u32;
pub const PBM_SETBARCOLOR: u32 = 1033u32;
pub const PBM_SETBKCOLOR: u32 = 8193u32;
pub const PBM_SETMARQUEE: u32 = 1034u32;
pub const PBM_SETPOS: u32 = 1026u32;
pub const PBM_SETRANGE: u32 = 1025u32;
pub const PBM_SETRANGE32: u32 = 1030u32;
pub const PBM_SETSTATE: u32 = 1040u32;
pub const PBM_SETSTEP: u32 = 1028u32;
pub const PBM_STEPIT: u32 = 1029u32;
#[repr(C)]
pub struct PBRANGE {
    pub iLow: i32,
    pub iHigh: i32,
}
impl ::core::marker::Copy for PBRANGE {}
impl ::core::clone::Clone for PBRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PBRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PBRANGE").field("iLow", &self.iLow).field("iHigh", &self.iHigh).finish()
    }
}
unsafe impl ::windows_core::Abi for PBRANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PBRANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PBRANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PBRANGE {}
impl ::core::default::Default for PBRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const PBST_ERROR: u32 = 2u32;
pub const PBST_NORMAL: u32 = 1u32;
pub const PBST_PAUSED: u32 = 3u32;
pub const PBS_MARQUEE: u32 = 8u32;
pub const PBS_SMOOTH: u32 = 1u32;
pub const PBS_SMOOTHREVERSE: u32 = 16u32;
pub const PBS_VERTICAL: u32 = 4u32;
pub type PFNDACOMPARE = ::core::option::Option<unsafe extern "system" fn(p1: *const ::core::ffi::c_void, p2: *const ::core::ffi::c_void, lparam: ::win32_foundation::LPARAM) -> i32>;
pub type PFNDACOMPARECONST = ::core::option::Option<unsafe extern "system" fn(p1: *const ::core::ffi::c_void, p2: *const ::core::ffi::c_void, lparam: ::win32_foundation::LPARAM) -> i32>;
pub type PFNDAENUMCALLBACK = ::core::option::Option<unsafe extern "system" fn(p: *const ::core::ffi::c_void, pdata: *const ::core::ffi::c_void) -> i32>;
pub type PFNDAENUMCALLBACKCONST = ::core::option::Option<unsafe extern "system" fn(p: *const ::core::ffi::c_void, pdata: *const ::core::ffi::c_void) -> i32>;
pub type PFNDPAMERGE = ::core::option::Option<unsafe extern "system" fn(umsg: DPAMM_MESSAGE, pvdest: *const ::core::ffi::c_void, pvsrc: *const ::core::ffi::c_void, lparam: ::win32_foundation::LPARAM) -> *mut ::core::ffi::c_void>;
pub type PFNDPAMERGECONST = ::core::option::Option<unsafe extern "system" fn(umsg: DPAMM_MESSAGE, pvdest: *const ::core::ffi::c_void, pvsrc: *const ::core::ffi::c_void, lparam: ::win32_foundation::LPARAM) -> *mut ::core::ffi::c_void>;
#[cfg(feature = "Win32_System_Com")]
pub type PFNDPASTREAM = ::core::option::Option<unsafe extern "system" fn(pinfo: *const DPASTREAMINFO, pstream: ::core::option::Option<::win32_system::Com::IStream>, pvinstdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT>;
pub type PFNLVCOMPARE = ::core::option::Option<unsafe extern "system" fn(param0: ::win32_foundation::LPARAM, param1: ::win32_foundation::LPARAM, param2: ::win32_foundation::LPARAM) -> i32>;
pub type PFNLVGROUPCOMPARE = ::core::option::Option<unsafe extern "system" fn(param0: i32, param1: i32, param2: *mut ::core::ffi::c_void) -> i32>;
pub type PFNPROPSHEETCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: ::win32_foundation::HWND, param1: u32, param2: ::win32_foundation::LPARAM) -> i32>;
pub type PFNTVCOMPARE = ::core::option::Option<unsafe extern "system" fn(lparam1: ::win32_foundation::LPARAM, lparam2: ::win32_foundation::LPARAM, lparamsort: ::win32_foundation::LPARAM) -> i32>;
pub type PFTASKDIALOGCALLBACK = ::core::option::Option<unsafe extern "system" fn(hwnd: ::win32_foundation::HWND, msg: u32, wparam: ::win32_foundation::WPARAM, lparam: ::win32_foundation::LPARAM, lprefdata: isize) -> ::windows_core::HRESULT>;
pub const PGB_BOTTOMORRIGHT: u32 = 1u32;
pub const PGB_TOPORLEFT: u32 = 0u32;
pub const PGF_DEPRESSED: u32 = 4u32;
pub const PGF_GRAYED: u32 = 2u32;
pub const PGF_HOT: u32 = 8u32;
pub const PGF_INVISIBLE: u32 = 0u32;
pub const PGF_NORMAL: u32 = 1u32;
pub const PGM_FIRST: u32 = 5120u32;
pub const PGM_FORWARDMOUSE: u32 = 5123u32;
pub const PGM_GETBKCOLOR: u32 = 5125u32;
pub const PGM_GETBORDER: u32 = 5127u32;
pub const PGM_GETBUTTONSIZE: u32 = 5131u32;
pub const PGM_GETBUTTONSTATE: u32 = 5132u32;
pub const PGM_GETDROPTARGET: u32 = 8196u32;
pub const PGM_GETPOS: u32 = 5129u32;
pub const PGM_RECALCSIZE: u32 = 5122u32;
pub const PGM_SETBKCOLOR: u32 = 5124u32;
pub const PGM_SETBORDER: u32 = 5126u32;
pub const PGM_SETBUTTONSIZE: u32 = 5130u32;
pub const PGM_SETCHILD: u32 = 5121u32;
pub const PGM_SETPOS: u32 = 5128u32;
pub const PGM_SETSCROLLINFO: u32 = 5133u32;
pub const PGS_AUTOSCROLL: u32 = 2u32;
pub const PGS_DRAGNDROP: u32 = 4u32;
pub const PGS_HORZ: u32 = 1u32;
pub const PGS_VERT: u32 = 0u32;
#[repr(C)]
pub struct POINTER_DEVICE_CURSOR_INFO {
    pub cursorId: u32,
    pub cursor: POINTER_DEVICE_CURSOR_TYPE,
}
impl ::core::marker::Copy for POINTER_DEVICE_CURSOR_INFO {}
impl ::core::clone::Clone for POINTER_DEVICE_CURSOR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POINTER_DEVICE_CURSOR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_DEVICE_CURSOR_INFO").field("cursorId", &self.cursorId).field("cursor", &self.cursor).finish()
    }
}
unsafe impl ::windows_core::Abi for POINTER_DEVICE_CURSOR_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for POINTER_DEVICE_CURSOR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POINTER_DEVICE_CURSOR_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for POINTER_DEVICE_CURSOR_INFO {}
impl ::core::default::Default for POINTER_DEVICE_CURSOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct POINTER_DEVICE_CURSOR_TYPE(pub i32);
pub const POINTER_DEVICE_CURSOR_TYPE_UNKNOWN: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(0i32);
pub const POINTER_DEVICE_CURSOR_TYPE_TIP: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(1i32);
pub const POINTER_DEVICE_CURSOR_TYPE_ERASER: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(2i32);
pub const POINTER_DEVICE_CURSOR_TYPE_MAX: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(-1i32);
impl ::core::marker::Copy for POINTER_DEVICE_CURSOR_TYPE {}
impl ::core::clone::Clone for POINTER_DEVICE_CURSOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for POINTER_DEVICE_CURSOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for POINTER_DEVICE_CURSOR_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for POINTER_DEVICE_CURSOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_DEVICE_CURSOR_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct POINTER_DEVICE_INFO {
    pub displayOrientation: u32,
    pub device: ::win32_foundation::HANDLE,
    pub pointerDeviceType: POINTER_DEVICE_TYPE,
    pub monitor: ::win32_graphics::Gdi::HMONITOR,
    pub startingCursorId: u32,
    pub maxActiveContacts: u16,
    pub productString: [u16; 520],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for POINTER_DEVICE_INFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for POINTER_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for POINTER_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_DEVICE_INFO").field("displayOrientation", &self.displayOrientation).field("device", &self.device).field("pointerDeviceType", &self.pointerDeviceType).field("monitor", &self.monitor).field("startingCursorId", &self.startingCursorId).field("maxActiveContacts", &self.maxActiveContacts).field("productString", &self.productString).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for POINTER_DEVICE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for POINTER_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POINTER_DEVICE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for POINTER_DEVICE_INFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for POINTER_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct POINTER_DEVICE_PROPERTY {
    pub logicalMin: i32,
    pub logicalMax: i32,
    pub physicalMin: i32,
    pub physicalMax: i32,
    pub unit: u32,
    pub unitExponent: u32,
    pub usagePageId: u16,
    pub usageId: u16,
}
impl ::core::marker::Copy for POINTER_DEVICE_PROPERTY {}
impl ::core::clone::Clone for POINTER_DEVICE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POINTER_DEVICE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTER_DEVICE_PROPERTY").field("logicalMin", &self.logicalMin).field("logicalMax", &self.logicalMax).field("physicalMin", &self.physicalMin).field("physicalMax", &self.physicalMax).field("unit", &self.unit).field("unitExponent", &self.unitExponent).field("usagePageId", &self.usagePageId).field("usageId", &self.usageId).finish()
    }
}
unsafe impl ::windows_core::Abi for POINTER_DEVICE_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for POINTER_DEVICE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POINTER_DEVICE_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for POINTER_DEVICE_PROPERTY {}
impl ::core::default::Default for POINTER_DEVICE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct POINTER_DEVICE_TYPE(pub i32);
pub const POINTER_DEVICE_TYPE_INTEGRATED_PEN: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(1i32);
pub const POINTER_DEVICE_TYPE_EXTERNAL_PEN: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(2i32);
pub const POINTER_DEVICE_TYPE_TOUCH: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(3i32);
pub const POINTER_DEVICE_TYPE_TOUCH_PAD: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(4i32);
pub const POINTER_DEVICE_TYPE_MAX: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(-1i32);
impl ::core::marker::Copy for POINTER_DEVICE_TYPE {}
impl ::core::clone::Clone for POINTER_DEVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for POINTER_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for POINTER_DEVICE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for POINTER_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_DEVICE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct POINTER_FEEDBACK_MODE(pub i32);
pub const POINTER_FEEDBACK_DEFAULT: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(1i32);
pub const POINTER_FEEDBACK_INDIRECT: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(2i32);
pub const POINTER_FEEDBACK_NONE: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(3i32);
impl ::core::marker::Copy for POINTER_FEEDBACK_MODE {}
impl ::core::clone::Clone for POINTER_FEEDBACK_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for POINTER_FEEDBACK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for POINTER_FEEDBACK_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for POINTER_FEEDBACK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_FEEDBACK_MODE").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct POINTER_TYPE_INFO {
    pub r#type: super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub Anonymous: POINTER_TYPE_INFO_0,
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for POINTER_TYPE_INFO {}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for POINTER_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for POINTER_TYPE_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for POINTER_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POINTER_TYPE_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for POINTER_TYPE_INFO {}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for POINTER_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
pub union POINTER_TYPE_INFO_0 {
    pub touchInfo: super::Input::Pointer::POINTER_TOUCH_INFO,
    pub penInfo: super::Input::Pointer::POINTER_PEN_INFO,
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for POINTER_TYPE_INFO_0 {}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for POINTER_TYPE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for POINTER_TYPE_INFO_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for POINTER_TYPE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POINTER_TYPE_INFO_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for POINTER_TYPE_INFO_0 {}
#[cfg(all(feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for POINTER_TYPE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const PRINTDLGEXORD: u32 = 1549u32;
pub const PRINTDLGORD: u32 = 1538u32;
pub const PRNSETUPDLGORD: u32 = 1539u32;
pub const PROGRESS_CLASS: &str = "msctls_progress32";
pub const PROGRESS_CLASSA: &str = "msctls_progress32";
pub const PROGRESS_CLASSW: &str = "msctls_progress32";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROPERTYORIGIN(pub i32);
pub const PO_STATE: PROPERTYORIGIN = PROPERTYORIGIN(0i32);
pub const PO_PART: PROPERTYORIGIN = PROPERTYORIGIN(1i32);
pub const PO_CLASS: PROPERTYORIGIN = PROPERTYORIGIN(2i32);
pub const PO_GLOBAL: PROPERTYORIGIN = PROPERTYORIGIN(3i32);
pub const PO_NOTFOUND: PROPERTYORIGIN = PROPERTYORIGIN(4i32);
impl ::core::marker::Copy for PROPERTYORIGIN {}
impl ::core::clone::Clone for PROPERTYORIGIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROPERTYORIGIN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROPERTYORIGIN {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROPERTYORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROPERTYORIGIN").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETHEADERA_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: ::win32_foundation::HWND,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERA_V1_0,
    pub pszCaption: ::windows_core::PCSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERA_V1_1,
    pub Anonymous3: PROPSHEETHEADERA_V1_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERA_V1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERA_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERA_V1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERA_V1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V1_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V1_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERA_V1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERA_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERA_V1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERA_V1_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERA_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V1_1 {
    pub nStartPage: u32,
    pub pStartPage: ::windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V1_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERA_V1_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERA_V1_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERA_V1_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERA_V1_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERA_V1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V1_2 {
    pub ppsp: *mut PROPSHEETPAGEA,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V1_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V1_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERA_V1_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERA_V1_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERA_V1_2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERA_V1_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERA_V1_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETHEADERA_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: ::win32_foundation::HWND,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERA_V2_0,
    pub pszCaption: ::windows_core::PCSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERA_V2_1,
    pub Anonymous3: PROPSHEETHEADERA_V2_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
    pub Anonymous4: PROPSHEETHEADERA_V2_3,
    pub hplWatermark: ::win32_graphics::Gdi::HPALETTE,
    pub Anonymous5: PROPSHEETHEADERA_V2_4,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERA_V2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERA_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERA_V2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERA_V2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V2_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V2_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERA_V2_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERA_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERA_V2_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERA_V2_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERA_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V2_1 {
    pub nStartPage: u32,
    pub pStartPage: ::windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V2_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERA_V2_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERA_V2_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERA_V2_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERA_V2_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERA_V2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V2_2 {
    pub ppsp: *mut PROPSHEETPAGEA,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V2_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V2_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERA_V2_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERA_V2_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERA_V2_2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERA_V2_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERA_V2_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V2_3 {
    pub hbmWatermark: ::win32_graphics::Gdi::HBITMAP,
    pub pszbmWatermark: ::windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V2_3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V2_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERA_V2_3 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERA_V2_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERA_V2_3>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERA_V2_3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERA_V2_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERA_V2_4 {
    pub hbmHeader: ::win32_graphics::Gdi::HBITMAP,
    pub pszbmHeader: ::windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERA_V2_4 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERA_V2_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERA_V2_4 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERA_V2_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERA_V2_4>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERA_V2_4 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERA_V2_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETHEADERW_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: ::win32_foundation::HWND,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERW_V1_0,
    pub pszCaption: ::windows_core::PCWSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERW_V1_1,
    pub Anonymous3: PROPSHEETHEADERW_V1_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERW_V1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERW_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERW_V1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERW_V1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERW_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V1_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V1_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERW_V1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERW_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERW_V1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERW_V1_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERW_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V1_1 {
    pub nStartPage: u32,
    pub pStartPage: ::windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V1_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERW_V1_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERW_V1_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERW_V1_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERW_V1_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERW_V1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V1_2 {
    pub ppsp: *mut PROPSHEETPAGEW,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V1_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V1_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERW_V1_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERW_V1_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERW_V1_2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERW_V1_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERW_V1_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETHEADERW_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: ::win32_foundation::HWND,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERW_V2_0,
    pub pszCaption: ::windows_core::PCWSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERW_V2_1,
    pub Anonymous3: PROPSHEETHEADERW_V2_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
    pub Anonymous4: PROPSHEETHEADERW_V2_3,
    pub hplWatermark: ::win32_graphics::Gdi::HPALETTE,
    pub Anonymous5: PROPSHEETHEADERW_V2_4,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERW_V2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERW_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERW_V2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERW_V2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERW_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V2_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V2_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERW_V2_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERW_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERW_V2_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERW_V2_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERW_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V2_1 {
    pub nStartPage: u32,
    pub pStartPage: ::windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V2_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERW_V2_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERW_V2_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERW_V2_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERW_V2_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERW_V2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V2_2 {
    pub ppsp: *mut PROPSHEETPAGEW,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V2_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V2_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERW_V2_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERW_V2_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERW_V2_2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERW_V2_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERW_V2_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V2_3 {
    pub hbmWatermark: ::win32_graphics::Gdi::HBITMAP,
    pub pszbmWatermark: ::windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V2_3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V2_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERW_V2_3 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERW_V2_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERW_V2_3>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERW_V2_3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERW_V2_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETHEADERW_V2_4 {
    pub hbmHeader: ::win32_graphics::Gdi::HBITMAP,
    pub pszbmHeader: ::windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETHEADERW_V2_4 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETHEADERW_V2_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETHEADERW_V2_4 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETHEADERW_V2_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETHEADERW_V2_4>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETHEADERW_V2_4 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETHEADERW_V2_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_0,
    pub Anonymous2: PROPSHEETPAGEA_1,
    pub pszTitle: ::windows_core::PCSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: ::win32_foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: ::windows_core::PCSTR,
    pub pszHeaderSubTitle: ::windows_core::PCSTR,
    pub hActCtx: ::win32_foundation::HANDLE,
    pub Anonymous3: PROPSHEETPAGEA_2,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_0 {
    pub pszTemplate: ::windows_core::PCSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_2 {
    pub hbmHeader: ::win32_graphics::Gdi::HBITMAP,
    pub pszbmHeader: ::windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA_2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEA_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V1_0,
    pub Anonymous2: PROPSHEETPAGEA_V1_1,
    pub pszTitle: ::windows_core::PCSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: ::win32_foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA_V1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA_V1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA_V1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_V1_0 {
    pub pszTemplate: ::windows_core::PCSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V1_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA_V1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA_V1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA_V1_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_V1_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V1_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA_V1_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA_V1_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA_V1_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA_V1_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_V1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEA_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V2_0,
    pub Anonymous2: PROPSHEETPAGEA_V2_1,
    pub pszTitle: ::windows_core::PCSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: ::win32_foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: ::windows_core::PCSTR,
    pub pszHeaderSubTitle: ::windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA_V2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA_V2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA_V2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_V2_0 {
    pub pszTemplate: ::windows_core::PCSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V2_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA_V2_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA_V2_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA_V2_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_V2_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V2_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA_V2_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA_V2_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA_V2_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA_V2_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_V2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEA_V3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V3_0,
    pub Anonymous2: PROPSHEETPAGEA_V3_1,
    pub pszTitle: ::windows_core::PCSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: ::win32_foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: ::windows_core::PCSTR,
    pub pszHeaderSubTitle: ::windows_core::PCSTR,
    pub hActCtx: ::win32_foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA_V3 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA_V3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA_V3>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA_V3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_V3_0 {
    pub pszTemplate: ::windows_core::PCSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V3_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA_V3_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA_V3_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA_V3_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA_V3_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_V3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEA_V3_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows_core::PCSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEA_V3_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEA_V3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEA_V3_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEA_V3_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEA_V3_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEA_V3_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEA_V3_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_0,
    pub Anonymous2: PROPSHEETPAGEW_1,
    pub pszTitle: ::windows_core::PCWSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: ::win32_foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: ::windows_core::PCWSTR,
    pub pszHeaderSubTitle: ::windows_core::PCWSTR,
    pub hActCtx: ::win32_foundation::HANDLE,
    pub Anonymous3: PROPSHEETPAGEW_2,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_0 {
    pub pszTemplate: ::windows_core::PCWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_2 {
    pub hbmHeader: ::win32_graphics::Gdi::HBITMAP,
    pub pszbmHeader: ::windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW_2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW_2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEW_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V1_0,
    pub Anonymous2: PROPSHEETPAGEW_V1_1,
    pub pszTitle: ::windows_core::PCWSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: ::win32_foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW_V1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW_V1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW_V1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_V1_0 {
    pub pszTemplate: ::windows_core::PCWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V1_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW_V1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW_V1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW_V1_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_V1_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V1_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW_V1_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW_V1_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW_V1_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW_V1_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_V1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEW_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V2_0,
    pub Anonymous2: PROPSHEETPAGEW_V2_1,
    pub pszTitle: ::windows_core::PCWSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: ::win32_foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: ::windows_core::PCWSTR,
    pub pszHeaderSubTitle: ::windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW_V2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW_V2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW_V2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_V2_0 {
    pub pszTemplate: ::windows_core::PCWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V2_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW_V2_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW_V2_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW_V2_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_V2_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V2_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW_V2_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW_V2_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW_V2_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW_V2_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_V2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct PROPSHEETPAGEW_V3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V3_0,
    pub Anonymous2: PROPSHEETPAGEW_V3_1,
    pub pszTitle: ::windows_core::PCWSTR,
    pub pfnDlgProc: super::WindowsAndMessaging::DLGPROC,
    pub lParam: ::win32_foundation::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: ::windows_core::PCWSTR,
    pub pszHeaderSubTitle: ::windows_core::PCWSTR,
    pub hActCtx: ::win32_foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW_V3 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW_V3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW_V3>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW_V3 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_V3_0 {
    pub pszTemplate: ::windows_core::PCWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V3_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW_V3_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW_V3_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW_V3_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW_V3_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_V3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub union PROPSHEETPAGEW_V3_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: ::windows_core::PCWSTR,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for PROPSHEETPAGEW_V3_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for PROPSHEETPAGEW_V3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows_core::Abi for PROPSHEETPAGEW_V3_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for PROPSHEETPAGEW_V3_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROPSHEETPAGEW_V3_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for PROPSHEETPAGEW_V3_1 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for PROPSHEETPAGEW_V3_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const PROP_LG_CXDLG: u32 = 252u32;
pub const PROP_LG_CYDLG: u32 = 218u32;
pub const PROP_MED_CXDLG: u32 = 227u32;
pub const PROP_MED_CYDLG: u32 = 215u32;
pub const PROP_SM_CXDLG: u32 = 212u32;
pub const PROP_SM_CYDLG: u32 = 188u32;
pub const PSBTN_APPLYNOW: u32 = 4u32;
pub const PSBTN_BACK: u32 = 0u32;
pub const PSBTN_CANCEL: u32 = 5u32;
pub const PSBTN_FINISH: u32 = 2u32;
pub const PSBTN_HELP: u32 = 6u32;
pub const PSBTN_MAX: u32 = 6u32;
pub const PSBTN_NEXT: u32 = 1u32;
pub const PSBTN_OK: u32 = 3u32;
pub const PSCB_BUTTONPRESSED: u32 = 3u32;
pub const PSCB_INITIALIZED: u32 = 1u32;
pub const PSCB_PRECREATE: u32 = 2u32;
#[repr(C)]
pub struct PSHNOTIFY {
    pub hdr: NMHDR,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for PSHNOTIFY {}
impl ::core::clone::Clone for PSHNOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PSHNOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSHNOTIFY").field("hdr", &self.hdr).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for PSHNOTIFY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSHNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSHNOTIFY>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSHNOTIFY {}
impl ::core::default::Default for PSHNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const PSH_AEROWIZARD: u32 = 16384u32;
pub const PSH_DEFAULT: u32 = 0u32;
pub const PSH_HASHELP: u32 = 512u32;
pub const PSH_HEADER: u32 = 524288u32;
pub const PSH_HEADERBITMAP: u32 = 134217728u32;
pub const PSH_MODELESS: u32 = 1024u32;
pub const PSH_NOAPPLYNOW: u32 = 128u32;
pub const PSH_NOCONTEXTHELP: u32 = 33554432u32;
pub const PSH_NOMARGIN: u32 = 268435456u32;
pub const PSH_PROPSHEETPAGE: u32 = 8u32;
pub const PSH_PROPTITLE: u32 = 1u32;
pub const PSH_RESIZABLE: u32 = 67108864u32;
pub const PSH_RTLREADING: u32 = 2048u32;
pub const PSH_STRETCHWATERMARK: u32 = 262144u32;
pub const PSH_USECALLBACK: u32 = 256u32;
pub const PSH_USEHBMHEADER: u32 = 1048576u32;
pub const PSH_USEHBMWATERMARK: u32 = 65536u32;
pub const PSH_USEHICON: u32 = 2u32;
pub const PSH_USEHPLWATERMARK: u32 = 131072u32;
pub const PSH_USEICONID: u32 = 4u32;
pub const PSH_USEPAGELANG: u32 = 2097152u32;
pub const PSH_USEPSTARTPAGE: u32 = 64u32;
pub const PSH_WATERMARK: u32 = 32768u32;
pub const PSH_WIZARD: u32 = 32u32;
pub const PSH_WIZARD97: u32 = 8192u32;
pub const PSH_WIZARDCONTEXTHELP: u32 = 4096u32;
pub const PSH_WIZARDHASFINISH: u32 = 16u32;
pub const PSH_WIZARD_LITE: u32 = 4194304u32;
pub const PSM_ADDPAGE: u32 = 1127u32;
pub const PSM_APPLY: u32 = 1134u32;
pub const PSM_CANCELTOCLOSE: u32 = 1131u32;
pub const PSM_CHANGED: u32 = 1128u32;
pub const PSM_ENABLEWIZBUTTONS: u32 = 1163u32;
pub const PSM_GETCURRENTPAGEHWND: u32 = 1142u32;
pub const PSM_GETRESULT: u32 = 1159u32;
pub const PSM_GETTABCONTROL: u32 = 1140u32;
pub const PSM_HWNDTOINDEX: u32 = 1153u32;
pub const PSM_IDTOINDEX: u32 = 1157u32;
pub const PSM_INDEXTOHWND: u32 = 1154u32;
pub const PSM_INDEXTOID: u32 = 1158u32;
pub const PSM_INDEXTOPAGE: u32 = 1156u32;
pub const PSM_INSERTPAGE: u32 = 1143u32;
pub const PSM_ISDIALOGMESSAGE: u32 = 1141u32;
pub const PSM_PAGETOINDEX: u32 = 1155u32;
pub const PSM_PRESSBUTTON: u32 = 1137u32;
pub const PSM_QUERYSIBLINGS: u32 = 1132u32;
pub const PSM_REBOOTSYSTEM: u32 = 1130u32;
pub const PSM_RECALCPAGESIZES: u32 = 1160u32;
pub const PSM_REMOVEPAGE: u32 = 1126u32;
pub const PSM_RESTARTWINDOWS: u32 = 1129u32;
pub const PSM_SETBUTTONTEXT: u32 = 1164u32;
pub const PSM_SETBUTTONTEXTW: u32 = 1164u32;
pub const PSM_SETCURSEL: u32 = 1125u32;
pub const PSM_SETCURSELID: u32 = 1138u32;
pub const PSM_SETFINISHTEXT: u32 = 1145u32;
pub const PSM_SETFINISHTEXTA: u32 = 1139u32;
pub const PSM_SETFINISHTEXTW: u32 = 1145u32;
pub const PSM_SETHEADERSUBTITLE: u32 = 1152u32;
pub const PSM_SETHEADERSUBTITLEA: u32 = 1151u32;
pub const PSM_SETHEADERSUBTITLEW: u32 = 1152u32;
pub const PSM_SETHEADERTITLE: u32 = 1150u32;
pub const PSM_SETHEADERTITLEA: u32 = 1149u32;
pub const PSM_SETHEADERTITLEW: u32 = 1150u32;
pub const PSM_SETNEXTTEXT: u32 = 1161u32;
pub const PSM_SETNEXTTEXTW: u32 = 1161u32;
pub const PSM_SETTITLE: u32 = 1144u32;
pub const PSM_SETTITLEA: u32 = 1135u32;
pub const PSM_SETTITLEW: u32 = 1144u32;
pub const PSM_SETWIZBUTTONS: u32 = 1136u32;
pub const PSM_SHOWWIZBUTTONS: u32 = 1162u32;
pub const PSM_UNCHANGED: u32 = 1133u32;
pub const PSNRET_INVALID: u32 = 1u32;
pub const PSNRET_INVALID_NOCHANGEPAGE: u32 = 2u32;
pub const PSNRET_MESSAGEHANDLED: u32 = 3u32;
pub const PSNRET_NOERROR: u32 = 0u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PSPCB_MESSAGE(pub u32);
pub const PSPCB_ADDREF: PSPCB_MESSAGE = PSPCB_MESSAGE(0u32);
pub const PSPCB_CREATE: PSPCB_MESSAGE = PSPCB_MESSAGE(2u32);
pub const PSPCB_RELEASE: PSPCB_MESSAGE = PSPCB_MESSAGE(1u32);
pub const PSPCB_SI_INITDIALOG: PSPCB_MESSAGE = PSPCB_MESSAGE(1025u32);
impl ::core::marker::Copy for PSPCB_MESSAGE {}
impl ::core::clone::Clone for PSPCB_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PSPCB_MESSAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PSPCB_MESSAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PSPCB_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSPCB_MESSAGE").field(&self.0).finish()
    }
}
pub const PSP_DEFAULT: u32 = 0u32;
pub const PSP_DLGINDIRECT: u32 = 1u32;
pub const PSP_HASHELP: u32 = 32u32;
pub const PSP_HIDEHEADER: u32 = 2048u32;
pub const PSP_PREMATURE: u32 = 1024u32;
pub const PSP_RTLREADING: u32 = 16u32;
pub const PSP_USECALLBACK: u32 = 128u32;
pub const PSP_USEFUSIONCONTEXT: u32 = 16384u32;
pub const PSP_USEHEADERSUBTITLE: u32 = 8192u32;
pub const PSP_USEHEADERTITLE: u32 = 4096u32;
pub const PSP_USEHICON: u32 = 2u32;
pub const PSP_USEICONID: u32 = 4u32;
pub const PSP_USEREFPARENT: u32 = 64u32;
pub const PSP_USETITLE: u32 = 8u32;
pub const PSWIZBF_ELEVATIONREQUIRED: u32 = 1u32;
pub const PSWIZB_BACK: u32 = 1u32;
pub const PSWIZB_CANCEL: u32 = 16u32;
pub const PSWIZB_DISABLEDFINISH: u32 = 8u32;
pub const PSWIZB_FINISH: u32 = 4u32;
pub const PSWIZB_NEXT: u32 = 2u32;
pub const PSWIZB_RESTORE: u32 = 1u32;
pub const PSWIZB_SHOW: u32 = 0u32;
#[inline]
pub unsafe fn PackTouchHitTestingProximityEvaluation(phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> ::win32_foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackTouchHitTestingProximityEvaluation(phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> ::win32_foundation::LRESULT;
        }
        ::core::mem::transmute(PackTouchHitTestingProximityEvaluation(::core::mem::transmute(phittestinginput), ::core::mem::transmute(pproximityeval)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn PropertySheetA(param0: *mut PROPSHEETHEADERA_V2) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropertySheetA(param0: *mut PROPSHEETHEADERA_V2) -> isize;
        }
        ::core::mem::transmute(PropertySheetA(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn PropertySheetW(param0: *mut PROPSHEETHEADERW_V2) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropertySheetW(param0: *mut PROPSHEETHEADERW_V2) -> isize;
        }
        ::core::mem::transmute(PropertySheetW(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const RBAB_ADDBAND: u32 = 2u32;
pub const RBAB_AUTOSIZE: u32 = 1u32;
pub const RBBIM_BACKGROUND: u32 = 128u32;
pub const RBBIM_CHEVRONLOCATION: u32 = 4096u32;
pub const RBBIM_CHEVRONSTATE: u32 = 8192u32;
pub const RBBIM_CHILD: u32 = 16u32;
pub const RBBIM_CHILDSIZE: u32 = 32u32;
pub const RBBIM_COLORS: u32 = 2u32;
pub const RBBIM_HEADERSIZE: u32 = 2048u32;
pub const RBBIM_ID: u32 = 256u32;
pub const RBBIM_IDEALSIZE: u32 = 512u32;
pub const RBBIM_IMAGE: u32 = 8u32;
pub const RBBIM_LPARAM: u32 = 1024u32;
pub const RBBIM_SIZE: u32 = 64u32;
pub const RBBIM_STYLE: u32 = 1u32;
pub const RBBIM_TEXT: u32 = 4u32;
pub const RBBS_BREAK: u32 = 1u32;
pub const RBBS_CHILDEDGE: u32 = 4u32;
pub const RBBS_FIXEDBMP: u32 = 32u32;
pub const RBBS_FIXEDSIZE: u32 = 2u32;
pub const RBBS_GRIPPERALWAYS: u32 = 128u32;
pub const RBBS_HIDDEN: u32 = 8u32;
pub const RBBS_HIDETITLE: u32 = 1024u32;
pub const RBBS_NOGRIPPER: u32 = 256u32;
pub const RBBS_NOVERT: u32 = 16u32;
pub const RBBS_TOPALIGN: u32 = 2048u32;
pub const RBBS_USECHEVRON: u32 = 512u32;
pub const RBBS_VARIABLEHEIGHT: u32 = 64u32;
#[repr(C)]
pub struct RBHITTESTINFO {
    pub pt: ::win32_foundation::POINT,
    pub flags: u32,
    pub iBand: i32,
}
impl ::core::marker::Copy for RBHITTESTINFO {}
impl ::core::clone::Clone for RBHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RBHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RBHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).field("iBand", &self.iBand).finish()
    }
}
unsafe impl ::windows_core::Abi for RBHITTESTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RBHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RBHITTESTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RBHITTESTINFO {}
impl ::core::default::Default for RBHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const RBHT_CAPTION: u32 = 2u32;
pub const RBHT_CHEVRON: u32 = 8u32;
pub const RBHT_CLIENT: u32 = 3u32;
pub const RBHT_GRABBER: u32 = 4u32;
pub const RBHT_NOWHERE: u32 = 1u32;
pub const RBHT_SPLITTER: u32 = 16u32;
pub const RBIM_IMAGELIST: u32 = 1u32;
pub const RBSTR_CHANGERECT: u32 = 1u32;
pub const RBS_AUTOSIZE: u32 = 8192u32;
pub const RBS_BANDBORDERS: u32 = 1024u32;
pub const RBS_DBLCLKTOGGLE: u32 = 32768u32;
pub const RBS_FIXEDORDER: u32 = 2048u32;
pub const RBS_REGISTERDROP: u32 = 4096u32;
pub const RBS_TOOLTIPS: u32 = 256u32;
pub const RBS_VARHEIGHT: u32 = 512u32;
pub const RBS_VERTICALGRIPPER: u32 = 16384u32;
pub const RB_BEGINDRAG: u32 = 1048u32;
pub const RB_DELETEBAND: u32 = 1026u32;
pub const RB_DRAGMOVE: u32 = 1050u32;
pub const RB_ENDDRAG: u32 = 1049u32;
pub const RB_GETBANDBORDERS: u32 = 1058u32;
pub const RB_GETBANDCOUNT: u32 = 1036u32;
pub const RB_GETBANDINFO: u32 = 1052u32;
pub const RB_GETBANDINFOA: u32 = 1053u32;
pub const RB_GETBANDINFOW: u32 = 1052u32;
pub const RB_GETBANDMARGINS: u32 = 1064u32;
pub const RB_GETBARHEIGHT: u32 = 1051u32;
pub const RB_GETBARINFO: u32 = 1027u32;
pub const RB_GETBKCOLOR: u32 = 1044u32;
pub const RB_GETCOLORSCHEME: u32 = 8195u32;
pub const RB_GETDROPTARGET: u32 = 8196u32;
pub const RB_GETEXTENDEDSTYLE: u32 = 1066u32;
pub const RB_GETPALETTE: u32 = 1062u32;
pub const RB_GETRECT: u32 = 1033u32;
pub const RB_GETROWCOUNT: u32 = 1037u32;
pub const RB_GETROWHEIGHT: u32 = 1038u32;
pub const RB_GETTEXTCOLOR: u32 = 1046u32;
pub const RB_GETTOOLTIPS: u32 = 1041u32;
pub const RB_GETUNICODEFORMAT: u32 = 8198u32;
pub const RB_HITTEST: u32 = 1032u32;
pub const RB_IDTOINDEX: u32 = 1040u32;
pub const RB_INSERTBAND: u32 = 1034u32;
pub const RB_INSERTBANDA: u32 = 1025u32;
pub const RB_INSERTBANDW: u32 = 1034u32;
pub const RB_MAXIMIZEBAND: u32 = 1055u32;
pub const RB_MINIMIZEBAND: u32 = 1054u32;
pub const RB_MOVEBAND: u32 = 1063u32;
pub const RB_PUSHCHEVRON: u32 = 1067u32;
pub const RB_SETBANDINFO: u32 = 1035u32;
pub const RB_SETBANDINFOA: u32 = 1030u32;
pub const RB_SETBANDINFOW: u32 = 1035u32;
pub const RB_SETBANDWIDTH: u32 = 1068u32;
pub const RB_SETBARINFO: u32 = 1028u32;
pub const RB_SETBKCOLOR: u32 = 1043u32;
pub const RB_SETCOLORSCHEME: u32 = 8194u32;
pub const RB_SETEXTENDEDSTYLE: u32 = 1065u32;
pub const RB_SETPALETTE: u32 = 1061u32;
pub const RB_SETPARENT: u32 = 1031u32;
pub const RB_SETTEXTCOLOR: u32 = 1045u32;
pub const RB_SETTOOLTIPS: u32 = 1042u32;
pub const RB_SETUNICODEFORMAT: u32 = 8197u32;
pub const RB_SETWINDOWTHEME: u32 = 8203u32;
pub const RB_SHOWBAND: u32 = 1059u32;
pub const RB_SIZETORECT: u32 = 1047u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct REBARBANDINFOA {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: u32,
    pub clrBack: u32,
    pub lpText: ::windows_core::PSTR,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: ::win32_foundation::HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: ::win32_graphics::Gdi::HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: ::win32_foundation::LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: ::win32_foundation::RECT,
    pub uChevronState: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for REBARBANDINFOA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for REBARBANDINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for REBARBANDINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REBARBANDINFOA")
            .field("cbSize", &self.cbSize)
            .field("fMask", &self.fMask)
            .field("fStyle", &self.fStyle)
            .field("clrFore", &self.clrFore)
            .field("clrBack", &self.clrBack)
            .field("lpText", &self.lpText)
            .field("cch", &self.cch)
            .field("iImage", &self.iImage)
            .field("hwndChild", &self.hwndChild)
            .field("cxMinChild", &self.cxMinChild)
            .field("cyMinChild", &self.cyMinChild)
            .field("cx", &self.cx)
            .field("hbmBack", &self.hbmBack)
            .field("wID", &self.wID)
            .field("cyChild", &self.cyChild)
            .field("cyMaxChild", &self.cyMaxChild)
            .field("cyIntegral", &self.cyIntegral)
            .field("cxIdeal", &self.cxIdeal)
            .field("lParam", &self.lParam)
            .field("cxHeader", &self.cxHeader)
            .field("rcChevronLocation", &self.rcChevronLocation)
            .field("uChevronState", &self.uChevronState)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for REBARBANDINFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for REBARBANDINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REBARBANDINFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for REBARBANDINFOA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for REBARBANDINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct REBARBANDINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: u32,
    pub clrBack: u32,
    pub lpText: ::windows_core::PWSTR,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: ::win32_foundation::HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: ::win32_graphics::Gdi::HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: ::win32_foundation::LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: ::win32_foundation::RECT,
    pub uChevronState: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for REBARBANDINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for REBARBANDINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for REBARBANDINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REBARBANDINFOW")
            .field("cbSize", &self.cbSize)
            .field("fMask", &self.fMask)
            .field("fStyle", &self.fStyle)
            .field("clrFore", &self.clrFore)
            .field("clrBack", &self.clrBack)
            .field("lpText", &self.lpText)
            .field("cch", &self.cch)
            .field("iImage", &self.iImage)
            .field("hwndChild", &self.hwndChild)
            .field("cxMinChild", &self.cxMinChild)
            .field("cyMinChild", &self.cyMinChild)
            .field("cx", &self.cx)
            .field("hbmBack", &self.hbmBack)
            .field("wID", &self.wID)
            .field("cyChild", &self.cyChild)
            .field("cyMaxChild", &self.cyMaxChild)
            .field("cyIntegral", &self.cyIntegral)
            .field("cxIdeal", &self.cxIdeal)
            .field("lParam", &self.lParam)
            .field("cxHeader", &self.cxHeader)
            .field("rcChevronLocation", &self.rcChevronLocation)
            .field("uChevronState", &self.uChevronState)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows_core::Abi for REBARBANDINFOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for REBARBANDINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REBARBANDINFOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for REBARBANDINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for REBARBANDINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const REBARCLASSNAME: &str = "ReBarWindow32";
pub const REBARCLASSNAMEA: &str = "ReBarWindow32";
pub const REBARCLASSNAMEW: &str = "ReBarWindow32";
#[repr(C)]
pub struct REBARINFO {
    pub cbSize: u32,
    pub fMask: u32,
    pub himl: HIMAGELIST,
}
impl ::core::marker::Copy for REBARINFO {}
impl ::core::clone::Clone for REBARINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REBARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REBARINFO").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("himl", &self.himl).finish()
    }
}
unsafe impl ::windows_core::Abi for REBARINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REBARINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REBARINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for REBARINFO {}
impl ::core::default::Default for REBARINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const REPLACEDLGORD: u32 = 1541u32;
pub const RUNDLGORD: u32 = 1545u32;
#[inline]
pub unsafe fn RegisterPointerDeviceNotifications<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(window: Param0, notifyrange: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterPointerDeviceNotifications(window: ::win32_foundation::HWND, notifyrange: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RegisterPointerDeviceNotifications(window.into_param().abi(), notifyrange.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegisterTouchHitTestingWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, value: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterTouchHitTestingWindow(hwnd: ::win32_foundation::HWND, value: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RegisterTouchHitTestingWindow(hwnd.into_param().abi(), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SBARS_SIZEGRIP: u32 = 256u32;
pub const SBARS_TOOLTIPS: u32 = 2048u32;
pub const SBT_NOBORDERS: u32 = 256u32;
pub const SBT_NOTABPARSING: u32 = 2048u32;
pub const SBT_OWNERDRAW: u32 = 4096u32;
pub const SBT_POPOUT: u32 = 512u32;
pub const SBT_RTLREADING: u32 = 1024u32;
pub const SBT_TOOLTIPS: u32 = 2048u32;
pub const SB_GETBORDERS: u32 = 1031u32;
pub const SB_GETICON: u32 = 1044u32;
pub const SB_GETPARTS: u32 = 1030u32;
pub const SB_GETRECT: u32 = 1034u32;
pub const SB_GETTEXT: u32 = 1037u32;
pub const SB_GETTEXTA: u32 = 1026u32;
pub const SB_GETTEXTLENGTH: u32 = 1036u32;
pub const SB_GETTEXTLENGTHA: u32 = 1027u32;
pub const SB_GETTEXTLENGTHW: u32 = 1036u32;
pub const SB_GETTEXTW: u32 = 1037u32;
pub const SB_GETTIPTEXTA: u32 = 1042u32;
pub const SB_GETTIPTEXTW: u32 = 1043u32;
pub const SB_GETUNICODEFORMAT: u32 = 8198u32;
pub const SB_ISSIMPLE: u32 = 1038u32;
pub const SB_SETBKCOLOR: u32 = 8193u32;
pub const SB_SETICON: u32 = 1039u32;
pub const SB_SETMINHEIGHT: u32 = 1032u32;
pub const SB_SETPARTS: u32 = 1028u32;
pub const SB_SETTEXT: u32 = 1035u32;
pub const SB_SETTEXTA: u32 = 1025u32;
pub const SB_SETTEXTW: u32 = 1035u32;
pub const SB_SETTIPTEXTA: u32 = 1040u32;
pub const SB_SETTIPTEXTW: u32 = 1041u32;
pub const SB_SETUNICODEFORMAT: u32 = 8197u32;
pub const SB_SIMPLE: u32 = 1033u32;
pub const SB_SIMPLEID: u32 = 255u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SIZINGTYPE(pub i32);
pub const ST_TRUESIZE: SIZINGTYPE = SIZINGTYPE(0i32);
pub const ST_STRETCH: SIZINGTYPE = SIZINGTYPE(1i32);
pub const ST_TILE: SIZINGTYPE = SIZINGTYPE(2i32);
impl ::core::marker::Copy for SIZINGTYPE {}
impl ::core::clone::Clone for SIZINGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SIZINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SIZINGTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SIZINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SIZINGTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SOFTWAREEXPLORERSTATES(pub i32);
pub const SPSE_NORMAL: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(1i32);
pub const SPSE_HOT: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(2i32);
pub const SPSE_SELECTED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(3i32);
pub const SPSE_DISABLED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(4i32);
pub const SPSE_FOCUSED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(5i32);
impl ::core::marker::Copy for SOFTWAREEXPLORERSTATES {}
impl ::core::clone::Clone for SOFTWAREEXPLORERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SOFTWAREEXPLORERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SOFTWAREEXPLORERSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for SOFTWAREEXPLORERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SOFTWAREEXPLORERSTATES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STARTPANELPARTS(pub i32);
pub const SPP_USERPANE: STARTPANELPARTS = STARTPANELPARTS(1i32);
pub const SPP_MOREPROGRAMS: STARTPANELPARTS = STARTPANELPARTS(2i32);
pub const SPP_MOREPROGRAMSARROW: STARTPANELPARTS = STARTPANELPARTS(3i32);
pub const SPP_PROGLIST: STARTPANELPARTS = STARTPANELPARTS(4i32);
pub const SPP_PROGLISTSEPARATOR: STARTPANELPARTS = STARTPANELPARTS(5i32);
pub const SPP_PLACESLIST: STARTPANELPARTS = STARTPANELPARTS(6i32);
pub const SPP_PLACESLISTSEPARATOR: STARTPANELPARTS = STARTPANELPARTS(7i32);
pub const SPP_LOGOFF: STARTPANELPARTS = STARTPANELPARTS(8i32);
pub const SPP_LOGOFFBUTTONS: STARTPANELPARTS = STARTPANELPARTS(9i32);
pub const SPP_USERPICTURE: STARTPANELPARTS = STARTPANELPARTS(10i32);
pub const SPP_PREVIEW: STARTPANELPARTS = STARTPANELPARTS(11i32);
pub const SPP_MOREPROGRAMSTAB: STARTPANELPARTS = STARTPANELPARTS(12i32);
pub const SPP_NSCHOST: STARTPANELPARTS = STARTPANELPARTS(13i32);
pub const SPP_SOFTWAREEXPLORER: STARTPANELPARTS = STARTPANELPARTS(14i32);
pub const SPP_OPENBOX: STARTPANELPARTS = STARTPANELPARTS(15i32);
pub const SPP_SEARCHVIEW: STARTPANELPARTS = STARTPANELPARTS(16i32);
pub const SPP_MOREPROGRAMSARROWBACK: STARTPANELPARTS = STARTPANELPARTS(17i32);
pub const SPP_TOPMATCH: STARTPANELPARTS = STARTPANELPARTS(18i32);
pub const SPP_LOGOFFSPLITBUTTONDROPDOWN: STARTPANELPARTS = STARTPANELPARTS(19i32);
impl ::core::marker::Copy for STARTPANELPARTS {}
impl ::core::clone::Clone for STARTPANELPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STARTPANELPARTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for STARTPANELPARTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for STARTPANELPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STARTPANELPARTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STATICPARTS(pub i32);
pub const STAT_TEXT: STATICPARTS = STATICPARTS(1i32);
impl ::core::marker::Copy for STATICPARTS {}
impl ::core::clone::Clone for STATICPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STATICPARTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for STATICPARTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for STATICPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STATICPARTS").field(&self.0).finish()
    }
}
pub const STATUSCLASSNAME: &str = "msctls_statusbar32";
pub const STATUSCLASSNAMEA: &str = "msctls_statusbar32";
pub const STATUSCLASSNAMEW: &str = "msctls_statusbar32";
pub const STD_COPY: u32 = 1u32;
pub const STD_CUT: u32 = 0u32;
pub const STD_DELETE: u32 = 5u32;
pub const STD_FILENEW: u32 = 6u32;
pub const STD_FILEOPEN: u32 = 7u32;
pub const STD_FILESAVE: u32 = 8u32;
pub const STD_FIND: u32 = 12u32;
pub const STD_HELP: u32 = 11u32;
pub const STD_PASTE: u32 = 2u32;
pub const STD_PRINT: u32 = 14u32;
pub const STD_PRINTPRE: u32 = 9u32;
pub const STD_PROPERTIES: u32 = 10u32;
pub const STD_REDOW: u32 = 4u32;
pub const STD_REPLACE: u32 = 13u32;
pub const STD_UNDO: u32 = 3u32;
pub const SZ_THDOCPROP_AUTHOR: &str = "author";
pub const SZ_THDOCPROP_CANONICALNAME: &str = "ThemeName";
pub const SZ_THDOCPROP_DISPLAYNAME: &str = "DisplayName";
pub const SZ_THDOCPROP_TOOLTIP: &str = "ToolTip";
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetScrollInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hwnd: Param0, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, lpsi: *const super::WindowsAndMessaging::SCROLLINFO, redraw: Param3) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetScrollInfo(hwnd: ::win32_foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, lpsi: *const super::WindowsAndMessaging::SCROLLINFO, redraw: ::win32_foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(SetScrollInfo(hwnd.into_param().abi(), ::core::mem::transmute(nbar), ::core::mem::transmute(lpsi), redraw.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetScrollPos<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hwnd: Param0, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, npos: i32, bredraw: Param3) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetScrollPos(hwnd: ::win32_foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, npos: i32, bredraw: ::win32_foundation::BOOL) -> i32;
        }
        ::core::mem::transmute(SetScrollPos(hwnd.into_param().abi(), ::core::mem::transmute(nbar), ::core::mem::transmute(npos), bredraw.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetScrollRange<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hwnd: Param0, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, nminpos: i32, nmaxpos: i32, bredraw: Param4) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetScrollRange(hwnd: ::win32_foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, nminpos: i32, nmaxpos: i32, bredraw: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetScrollRange(hwnd.into_param().abi(), ::core::mem::transmute(nbar), ::core::mem::transmute(nminpos), ::core::mem::transmute(nmaxpos), bredraw.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetThemeAppProperties(dwflags: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThemeAppProperties(dwflags: u32);
        }
        SetThemeAppProperties(::core::mem::transmute(dwflags))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetWindowFeedbackSetting<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, feedback: FEEDBACK_TYPE, dwflags: u32, size: u32, configuration: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWindowFeedbackSetting(hwnd: ::win32_foundation::HWND, feedback: FEEDBACK_TYPE, dwflags: u32, size: u32, configuration: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(SetWindowFeedbackSetting(hwnd.into_param().abi(), ::core::mem::transmute(feedback), ::core::mem::transmute(dwflags), ::core::mem::transmute(size), ::core::mem::transmute(configuration)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetWindowTheme<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hwnd: Param0, pszsubappname: Param1, pszsubidlist: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWindowTheme(hwnd: ::win32_foundation::HWND, pszsubappname: ::windows_core::PCWSTR, pszsubidlist: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        SetWindowTheme(hwnd.into_param().abi(), pszsubappname.into_param().abi(), pszsubidlist.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetWindowThemeAttribute<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, eattribute: WINDOWTHEMEATTRIBUTETYPE, pvattribute: *const ::core::ffi::c_void, cbattribute: u32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWindowThemeAttribute(hwnd: ::win32_foundation::HWND, eattribute: WINDOWTHEMEATTRIBUTETYPE, pvattribute: *const ::core::ffi::c_void, cbattribute: u32) -> ::windows_core::HRESULT;
        }
        SetWindowThemeAttribute(hwnd.into_param().abi(), ::core::mem::transmute(eattribute), ::core::mem::transmute(pvattribute), ::core::mem::transmute(cbattribute)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowHideMenuCtl<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(hwnd: Param0, uflags: usize, lpinfo: *const i32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowHideMenuCtl(hwnd: ::win32_foundation::HWND, uflags: usize, lpinfo: *const i32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ShowHideMenuCtl(hwnd.into_param().abi(), ::core::mem::transmute(uflags), ::core::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ShowScrollBar<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hwnd: Param0, wbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, bshow: Param2) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowScrollBar(hwnd: ::win32_foundation::HWND, wbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, bshow: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ShowScrollBar(hwnd.into_param().abi(), ::core::mem::transmute(wbar), bshow.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn Str_SetPtrW<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(ppsz: *mut ::windows_core::PWSTR, psz: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Str_SetPtrW(ppsz: *mut ::windows_core::PWSTR, psz: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(Str_SetPtrW(::core::mem::transmute(ppsz), psz.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASKBANDPARTS(pub i32);
pub const TDP_GROUPCOUNT: TASKBANDPARTS = TASKBANDPARTS(1i32);
pub const TDP_FLASHBUTTON: TASKBANDPARTS = TASKBANDPARTS(2i32);
pub const TDP_FLASHBUTTONGROUPMENU: TASKBANDPARTS = TASKBANDPARTS(3i32);
impl ::core::marker::Copy for TASKBANDPARTS {}
impl ::core::clone::Clone for TASKBANDPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASKBANDPARTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASKBANDPARTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASKBANDPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKBANDPARTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASKBARPARTS(pub i32);
pub const TBP_BACKGROUNDBOTTOM: TASKBARPARTS = TASKBARPARTS(1i32);
pub const TBP_BACKGROUNDRIGHT: TASKBARPARTS = TASKBARPARTS(2i32);
pub const TBP_BACKGROUNDTOP: TASKBARPARTS = TASKBARPARTS(3i32);
pub const TBP_BACKGROUNDLEFT: TASKBARPARTS = TASKBARPARTS(4i32);
pub const TBP_SIZINGBARBOTTOM: TASKBARPARTS = TASKBARPARTS(5i32);
pub const TBP_SIZINGBARRIGHT: TASKBARPARTS = TASKBARPARTS(6i32);
pub const TBP_SIZINGBARTOP: TASKBARPARTS = TASKBARPARTS(7i32);
pub const TBP_SIZINGBARLEFT: TASKBARPARTS = TASKBARPARTS(8i32);
impl ::core::marker::Copy for TASKBARPARTS {}
impl ::core::clone::Clone for TASKBARPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASKBARPARTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASKBARPARTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASKBARPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKBARPARTS").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct TASKDIALOGCONFIG {
    pub cbSize: u32,
    pub hwndParent: ::win32_foundation::HWND,
    pub hInstance: ::win32_foundation::HINSTANCE,
    pub dwFlags: TASKDIALOG_FLAGS,
    pub dwCommonButtons: TASKDIALOG_COMMON_BUTTON_FLAGS,
    pub pszWindowTitle: ::windows_core::PCWSTR,
    pub Anonymous1: TASKDIALOGCONFIG_0,
    pub pszMainInstruction: ::windows_core::PCWSTR,
    pub pszContent: ::windows_core::PCWSTR,
    pub cButtons: u32,
    pub pButtons: *const TASKDIALOG_BUTTON,
    pub nDefaultButton: i32,
    pub cRadioButtons: u32,
    pub pRadioButtons: *const TASKDIALOG_BUTTON,
    pub nDefaultRadioButton: i32,
    pub pszVerificationText: ::windows_core::PCWSTR,
    pub pszExpandedInformation: ::windows_core::PCWSTR,
    pub pszExpandedControlText: ::windows_core::PCWSTR,
    pub pszCollapsedControlText: ::windows_core::PCWSTR,
    pub Anonymous2: TASKDIALOGCONFIG_1,
    pub pszFooter: ::windows_core::PCWSTR,
    pub pfCallback: PFTASKDIALOGCALLBACK,
    pub lpCallbackData: isize,
    pub cxWidth: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for TASKDIALOGCONFIG {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for TASKDIALOGCONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows_core::Abi for TASKDIALOGCONFIG {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for TASKDIALOGCONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TASKDIALOGCONFIG>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for TASKDIALOGCONFIG {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for TASKDIALOGCONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub union TASKDIALOGCONFIG_0 {
    pub hMainIcon: super::WindowsAndMessaging::HICON,
    pub pszMainIcon: ::windows_core::PCWSTR,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for TASKDIALOGCONFIG_0 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for TASKDIALOGCONFIG_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows_core::Abi for TASKDIALOGCONFIG_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for TASKDIALOGCONFIG_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TASKDIALOGCONFIG_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for TASKDIALOGCONFIG_0 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for TASKDIALOGCONFIG_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub union TASKDIALOGCONFIG_1 {
    pub hFooterIcon: super::WindowsAndMessaging::HICON,
    pub pszFooterIcon: ::windows_core::PCWSTR,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::marker::Copy for TASKDIALOGCONFIG_1 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for TASKDIALOGCONFIG_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows_core::Abi for TASKDIALOGCONFIG_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for TASKDIALOGCONFIG_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TASKDIALOGCONFIG_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for TASKDIALOGCONFIG_1 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for TASKDIALOGCONFIG_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct TASKDIALOG_BUTTON {
    pub nButtonID: i32,
    pub pszButtonText: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for TASKDIALOG_BUTTON {}
impl ::core::clone::Clone for TASKDIALOG_BUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for TASKDIALOG_BUTTON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TASKDIALOG_BUTTON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TASKDIALOG_BUTTON>()) == 0 }
    }
}
impl ::core::cmp::Eq for TASKDIALOG_BUTTON {}
impl ::core::default::Default for TASKDIALOG_BUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASKDIALOG_COMMON_BUTTON_FLAGS(pub i32);
pub const TDCBF_OK_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(1i32);
pub const TDCBF_YES_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(2i32);
pub const TDCBF_NO_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(4i32);
pub const TDCBF_CANCEL_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(8i32);
pub const TDCBF_RETRY_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(16i32);
pub const TDCBF_CLOSE_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(32i32);
impl ::core::marker::Copy for TASKDIALOG_COMMON_BUTTON_FLAGS {}
impl ::core::clone::Clone for TASKDIALOG_COMMON_BUTTON_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASKDIALOG_COMMON_BUTTON_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASKDIALOG_COMMON_BUTTON_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASKDIALOG_COMMON_BUTTON_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOG_COMMON_BUTTON_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASKDIALOG_ELEMENTS(pub i32);
pub const TDE_CONTENT: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(0i32);
pub const TDE_EXPANDED_INFORMATION: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(1i32);
pub const TDE_FOOTER: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(2i32);
pub const TDE_MAIN_INSTRUCTION: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(3i32);
impl ::core::marker::Copy for TASKDIALOG_ELEMENTS {}
impl ::core::clone::Clone for TASKDIALOG_ELEMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASKDIALOG_ELEMENTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASKDIALOG_ELEMENTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASKDIALOG_ELEMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOG_ELEMENTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASKDIALOG_FLAGS(pub i32);
pub const TDF_ENABLE_HYPERLINKS: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(1i32);
pub const TDF_USE_HICON_MAIN: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(2i32);
pub const TDF_USE_HICON_FOOTER: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(4i32);
pub const TDF_ALLOW_DIALOG_CANCELLATION: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(8i32);
pub const TDF_USE_COMMAND_LINKS: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16i32);
pub const TDF_USE_COMMAND_LINKS_NO_ICON: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(32i32);
pub const TDF_EXPAND_FOOTER_AREA: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(64i32);
pub const TDF_EXPANDED_BY_DEFAULT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(128i32);
pub const TDF_VERIFICATION_FLAG_CHECKED: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(256i32);
pub const TDF_SHOW_PROGRESS_BAR: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(512i32);
pub const TDF_SHOW_MARQUEE_PROGRESS_BAR: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(1024i32);
pub const TDF_CALLBACK_TIMER: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(2048i32);
pub const TDF_POSITION_RELATIVE_TO_WINDOW: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(4096i32);
pub const TDF_RTL_LAYOUT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(8192i32);
pub const TDF_NO_DEFAULT_RADIO_BUTTON: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16384i32);
pub const TDF_CAN_BE_MINIMIZED: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(32768i32);
pub const TDF_NO_SET_FOREGROUND: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(65536i32);
pub const TDF_SIZE_TO_CONTENT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16777216i32);
impl ::core::marker::Copy for TASKDIALOG_FLAGS {}
impl ::core::clone::Clone for TASKDIALOG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASKDIALOG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASKDIALOG_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASKDIALOG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOG_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASKDIALOG_ICON_ELEMENTS(pub i32);
pub const TDIE_ICON_MAIN: TASKDIALOG_ICON_ELEMENTS = TASKDIALOG_ICON_ELEMENTS(0i32);
pub const TDIE_ICON_FOOTER: TASKDIALOG_ICON_ELEMENTS = TASKDIALOG_ICON_ELEMENTS(1i32);
impl ::core::marker::Copy for TASKDIALOG_ICON_ELEMENTS {}
impl ::core::clone::Clone for TASKDIALOG_ICON_ELEMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASKDIALOG_ICON_ELEMENTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASKDIALOG_ICON_ELEMENTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASKDIALOG_ICON_ELEMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOG_ICON_ELEMENTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASKDIALOG_MESSAGES(pub i32);
pub const TDM_NAVIGATE_PAGE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1125i32);
pub const TDM_CLICK_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1126i32);
pub const TDM_SET_MARQUEE_PROGRESS_BAR: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1127i32);
pub const TDM_SET_PROGRESS_BAR_STATE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1128i32);
pub const TDM_SET_PROGRESS_BAR_RANGE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1129i32);
pub const TDM_SET_PROGRESS_BAR_POS: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1130i32);
pub const TDM_SET_PROGRESS_BAR_MARQUEE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1131i32);
pub const TDM_SET_ELEMENT_TEXT: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1132i32);
pub const TDM_CLICK_RADIO_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1134i32);
pub const TDM_ENABLE_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1135i32);
pub const TDM_ENABLE_RADIO_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1136i32);
pub const TDM_CLICK_VERIFICATION: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1137i32);
pub const TDM_UPDATE_ELEMENT_TEXT: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1138i32);
pub const TDM_SET_BUTTON_ELEVATION_REQUIRED_STATE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1139i32);
pub const TDM_UPDATE_ICON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1140i32);
impl ::core::marker::Copy for TASKDIALOG_MESSAGES {}
impl ::core::clone::Clone for TASKDIALOG_MESSAGES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASKDIALOG_MESSAGES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASKDIALOG_MESSAGES {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASKDIALOG_MESSAGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOG_MESSAGES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TASKDIALOG_NOTIFICATIONS(pub i32);
pub const TDN_CREATED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(0i32);
pub const TDN_NAVIGATED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(1i32);
pub const TDN_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(2i32);
pub const TDN_HYPERLINK_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(3i32);
pub const TDN_TIMER: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(4i32);
pub const TDN_DESTROYED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(5i32);
pub const TDN_RADIO_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(6i32);
pub const TDN_DIALOG_CONSTRUCTED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(7i32);
pub const TDN_VERIFICATION_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(8i32);
pub const TDN_HELP: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(9i32);
pub const TDN_EXPANDO_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(10i32);
impl ::core::marker::Copy for TASKDIALOG_NOTIFICATIONS {}
impl ::core::clone::Clone for TASKDIALOG_NOTIFICATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASKDIALOG_NOTIFICATIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TASKDIALOG_NOTIFICATIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASKDIALOG_NOTIFICATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKDIALOG_NOTIFICATIONS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct TA_CUBIC_BEZIER {
    pub header: TA_TIMINGFUNCTION,
    pub rX0: f32,
    pub rY0: f32,
    pub rX1: f32,
    pub rY1: f32,
}
impl ::core::marker::Copy for TA_CUBIC_BEZIER {}
impl ::core::clone::Clone for TA_CUBIC_BEZIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TA_CUBIC_BEZIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TA_CUBIC_BEZIER").field("header", &self.header).field("rX0", &self.rX0).field("rY0", &self.rY0).field("rX1", &self.rX1).field("rY1", &self.rY1).finish()
    }
}
unsafe impl ::windows_core::Abi for TA_CUBIC_BEZIER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TA_CUBIC_BEZIER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TA_CUBIC_BEZIER>()) == 0 }
    }
}
impl ::core::cmp::Eq for TA_CUBIC_BEZIER {}
impl ::core::default::Default for TA_CUBIC_BEZIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TA_PROPERTY(pub i32);
pub const TAP_FLAGS: TA_PROPERTY = TA_PROPERTY(0i32);
pub const TAP_TRANSFORMCOUNT: TA_PROPERTY = TA_PROPERTY(1i32);
pub const TAP_STAGGERDELAY: TA_PROPERTY = TA_PROPERTY(2i32);
pub const TAP_STAGGERDELAYCAP: TA_PROPERTY = TA_PROPERTY(3i32);
pub const TAP_STAGGERDELAYFACTOR: TA_PROPERTY = TA_PROPERTY(4i32);
pub const TAP_ZORDER: TA_PROPERTY = TA_PROPERTY(5i32);
impl ::core::marker::Copy for TA_PROPERTY {}
impl ::core::clone::Clone for TA_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TA_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TA_PROPERTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for TA_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TA_PROPERTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TA_PROPERTY_FLAG(pub u32);
pub const TAPF_NONE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(0u32);
pub const TAPF_HASSTAGGER: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(1u32);
pub const TAPF_ISRTLAWARE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(2u32);
pub const TAPF_ALLOWCOLLECTION: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(4u32);
pub const TAPF_HASBACKGROUND: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(8u32);
pub const TAPF_HASPERSPECTIVE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(16u32);
impl ::core::marker::Copy for TA_PROPERTY_FLAG {}
impl ::core::clone::Clone for TA_PROPERTY_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TA_PROPERTY_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TA_PROPERTY_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for TA_PROPERTY_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TA_PROPERTY_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TA_PROPERTY_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TA_PROPERTY_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TA_PROPERTY_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TA_PROPERTY_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TA_PROPERTY_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
pub struct TA_TIMINGFUNCTION {
    pub eTimingFunctionType: TA_TIMINGFUNCTION_TYPE,
}
impl ::core::marker::Copy for TA_TIMINGFUNCTION {}
impl ::core::clone::Clone for TA_TIMINGFUNCTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TA_TIMINGFUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TA_TIMINGFUNCTION").field("eTimingFunctionType", &self.eTimingFunctionType).finish()
    }
}
unsafe impl ::windows_core::Abi for TA_TIMINGFUNCTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TA_TIMINGFUNCTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TA_TIMINGFUNCTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for TA_TIMINGFUNCTION {}
impl ::core::default::Default for TA_TIMINGFUNCTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TA_TIMINGFUNCTION_TYPE(pub i32);
pub const TTFT_UNDEFINED: TA_TIMINGFUNCTION_TYPE = TA_TIMINGFUNCTION_TYPE(0i32);
pub const TTFT_CUBIC_BEZIER: TA_TIMINGFUNCTION_TYPE = TA_TIMINGFUNCTION_TYPE(1i32);
impl ::core::marker::Copy for TA_TIMINGFUNCTION_TYPE {}
impl ::core::clone::Clone for TA_TIMINGFUNCTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TA_TIMINGFUNCTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TA_TIMINGFUNCTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TA_TIMINGFUNCTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TA_TIMINGFUNCTION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct TA_TRANSFORM {
    pub eTransformType: TA_TRANSFORM_TYPE,
    pub dwTimingFunctionId: u32,
    pub dwStartTime: u32,
    pub dwDurationTime: u32,
    pub eFlags: TA_TRANSFORM_FLAG,
}
impl ::core::marker::Copy for TA_TRANSFORM {}
impl ::core::clone::Clone for TA_TRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TA_TRANSFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TA_TRANSFORM").field("eTransformType", &self.eTransformType).field("dwTimingFunctionId", &self.dwTimingFunctionId).field("dwStartTime", &self.dwStartTime).field("dwDurationTime", &self.dwDurationTime).field("eFlags", &self.eFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for TA_TRANSFORM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TA_TRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TA_TRANSFORM>()) == 0 }
    }
}
impl ::core::cmp::Eq for TA_TRANSFORM {}
impl ::core::default::Default for TA_TRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TA_TRANSFORM_2D {
    pub header: TA_TRANSFORM,
    pub rX: f32,
    pub rY: f32,
    pub rInitialX: f32,
    pub rInitialY: f32,
    pub rOriginX: f32,
    pub rOriginY: f32,
}
impl ::core::marker::Copy for TA_TRANSFORM_2D {}
impl ::core::clone::Clone for TA_TRANSFORM_2D {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TA_TRANSFORM_2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TA_TRANSFORM_2D").field("header", &self.header).field("rX", &self.rX).field("rY", &self.rY).field("rInitialX", &self.rInitialX).field("rInitialY", &self.rInitialY).field("rOriginX", &self.rOriginX).field("rOriginY", &self.rOriginY).finish()
    }
}
unsafe impl ::windows_core::Abi for TA_TRANSFORM_2D {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TA_TRANSFORM_2D {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TA_TRANSFORM_2D>()) == 0 }
    }
}
impl ::core::cmp::Eq for TA_TRANSFORM_2D {}
impl ::core::default::Default for TA_TRANSFORM_2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TA_TRANSFORM_CLIP {
    pub header: TA_TRANSFORM,
    pub rLeft: f32,
    pub rTop: f32,
    pub rRight: f32,
    pub rBottom: f32,
    pub rInitialLeft: f32,
    pub rInitialTop: f32,
    pub rInitialRight: f32,
    pub rInitialBottom: f32,
}
impl ::core::marker::Copy for TA_TRANSFORM_CLIP {}
impl ::core::clone::Clone for TA_TRANSFORM_CLIP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TA_TRANSFORM_CLIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TA_TRANSFORM_CLIP").field("header", &self.header).field("rLeft", &self.rLeft).field("rTop", &self.rTop).field("rRight", &self.rRight).field("rBottom", &self.rBottom).field("rInitialLeft", &self.rInitialLeft).field("rInitialTop", &self.rInitialTop).field("rInitialRight", &self.rInitialRight).field("rInitialBottom", &self.rInitialBottom).finish()
    }
}
unsafe impl ::windows_core::Abi for TA_TRANSFORM_CLIP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TA_TRANSFORM_CLIP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TA_TRANSFORM_CLIP>()) == 0 }
    }
}
impl ::core::cmp::Eq for TA_TRANSFORM_CLIP {}
impl ::core::default::Default for TA_TRANSFORM_CLIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TA_TRANSFORM_FLAG(pub i32);
pub const TATF_NONE: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(0i32);
pub const TATF_TARGETVALUES_USER: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(1i32);
pub const TATF_HASINITIALVALUES: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(2i32);
pub const TATF_HASORIGINVALUES: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(4i32);
impl ::core::marker::Copy for TA_TRANSFORM_FLAG {}
impl ::core::clone::Clone for TA_TRANSFORM_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TA_TRANSFORM_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TA_TRANSFORM_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for TA_TRANSFORM_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TA_TRANSFORM_FLAG").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct TA_TRANSFORM_OPACITY {
    pub header: TA_TRANSFORM,
    pub rOpacity: f32,
    pub rInitialOpacity: f32,
}
impl ::core::marker::Copy for TA_TRANSFORM_OPACITY {}
impl ::core::clone::Clone for TA_TRANSFORM_OPACITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TA_TRANSFORM_OPACITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TA_TRANSFORM_OPACITY").field("header", &self.header).field("rOpacity", &self.rOpacity).field("rInitialOpacity", &self.rInitialOpacity).finish()
    }
}
unsafe impl ::windows_core::Abi for TA_TRANSFORM_OPACITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TA_TRANSFORM_OPACITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TA_TRANSFORM_OPACITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for TA_TRANSFORM_OPACITY {}
impl ::core::default::Default for TA_TRANSFORM_OPACITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TA_TRANSFORM_TYPE(pub i32);
pub const TATT_TRANSLATE_2D: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(0i32);
pub const TATT_SCALE_2D: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(1i32);
pub const TATT_OPACITY: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(2i32);
pub const TATT_CLIP: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(3i32);
impl ::core::marker::Copy for TA_TRANSFORM_TYPE {}
impl ::core::clone::Clone for TA_TRANSFORM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TA_TRANSFORM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TA_TRANSFORM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TA_TRANSFORM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TA_TRANSFORM_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct TBADDBITMAP {
    pub hInst: ::win32_foundation::HINSTANCE,
    pub nID: usize,
}
impl ::core::marker::Copy for TBADDBITMAP {}
impl ::core::clone::Clone for TBADDBITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TBADDBITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBADDBITMAP").field("hInst", &self.hInst).field("nID", &self.nID).finish()
    }
}
unsafe impl ::windows_core::Abi for TBADDBITMAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TBADDBITMAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBADDBITMAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for TBADDBITMAP {}
impl ::core::default::Default for TBADDBITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TBBF_LARGE: u32 = 1u32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct TBBUTTON {
    pub iBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub bReserved: [u8; 6],
    pub dwData: usize,
    pub iString: isize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for TBBUTTON {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for TBBUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for TBBUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBBUTTON").field("iBitmap", &self.iBitmap).field("idCommand", &self.idCommand).field("fsState", &self.fsState).field("fsStyle", &self.fsStyle).field("bReserved", &self.bReserved).field("dwData", &self.dwData).field("iString", &self.iString).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows_core::Abi for TBBUTTON {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for TBBUTTON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBBUTTON>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for TBBUTTON {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for TBBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
pub struct TBBUTTON {
    pub iBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub bReserved: [u8; 2],
    pub dwData: usize,
    pub iString: isize,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for TBBUTTON {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for TBBUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for TBBUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBBUTTON").field("iBitmap", &self.iBitmap).field("idCommand", &self.idCommand).field("fsState", &self.fsState).field("fsStyle", &self.fsStyle).field("bReserved", &self.bReserved).field("dwData", &self.dwData).field("iString", &self.iString).finish()
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows_core::Abi for TBBUTTON {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for TBBUTTON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBBUTTON>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for TBBUTTON {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for TBBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TBBUTTONINFOA {
    pub cbSize: u32,
    pub dwMask: TBBUTTONINFOW_MASK,
    pub idCommand: i32,
    pub iImage: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub cx: u16,
    pub lParam: usize,
    pub pszText: ::windows_core::PSTR,
    pub cchText: i32,
}
impl ::core::marker::Copy for TBBUTTONINFOA {}
impl ::core::clone::Clone for TBBUTTONINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TBBUTTONINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBBUTTONINFOA").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("idCommand", &self.idCommand).field("iImage", &self.iImage).field("fsState", &self.fsState).field("fsStyle", &self.fsStyle).field("cx", &self.cx).field("lParam", &self.lParam).field("pszText", &self.pszText).field("cchText", &self.cchText).finish()
    }
}
unsafe impl ::windows_core::Abi for TBBUTTONINFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TBBUTTONINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBBUTTONINFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for TBBUTTONINFOA {}
impl ::core::default::Default for TBBUTTONINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TBBUTTONINFOW {
    pub cbSize: u32,
    pub dwMask: TBBUTTONINFOW_MASK,
    pub idCommand: i32,
    pub iImage: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub cx: u16,
    pub lParam: usize,
    pub pszText: ::windows_core::PWSTR,
    pub cchText: i32,
}
impl ::core::marker::Copy for TBBUTTONINFOW {}
impl ::core::clone::Clone for TBBUTTONINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TBBUTTONINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBBUTTONINFOW").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("idCommand", &self.idCommand).field("iImage", &self.iImage).field("fsState", &self.fsState).field("fsStyle", &self.fsStyle).field("cx", &self.cx).field("lParam", &self.lParam).field("pszText", &self.pszText).field("cchText", &self.cchText).finish()
    }
}
unsafe impl ::windows_core::Abi for TBBUTTONINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TBBUTTONINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBBUTTONINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for TBBUTTONINFOW {}
impl ::core::default::Default for TBBUTTONINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TBBUTTONINFOW_MASK(pub u32);
pub const TBIF_BYINDEX: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(2147483648u32);
pub const TBIF_COMMAND: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(32u32);
pub const TBIF_IMAGE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(1u32);
pub const TBIF_LPARAM: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(16u32);
pub const TBIF_SIZE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(64u32);
pub const TBIF_STATE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(4u32);
pub const TBIF_STYLE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(8u32);
pub const TBIF_TEXT: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(2u32);
impl ::core::marker::Copy for TBBUTTONINFOW_MASK {}
impl ::core::clone::Clone for TBBUTTONINFOW_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TBBUTTONINFOW_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TBBUTTONINFOW_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for TBBUTTONINFOW_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TBBUTTONINFOW_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TBBUTTONINFOW_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TBBUTTONINFOW_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TBBUTTONINFOW_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TBBUTTONINFOW_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TBBUTTONINFOW_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TBCDRF_BLENDICON: u32 = 2097152u32;
pub const TBCDRF_HILITEHOTTRACK: u32 = 131072u32;
pub const TBCDRF_NOBACKGROUND: u32 = 4194304u32;
pub const TBCDRF_NOEDGES: u32 = 65536u32;
pub const TBCDRF_NOETCHEDEFFECT: u32 = 1048576u32;
pub const TBCDRF_NOMARK: u32 = 524288u32;
pub const TBCDRF_NOOFFSET: u32 = 262144u32;
pub const TBCDRF_USECDCOLORS: u32 = 8388608u32;
pub const TBCD_CHANNEL: u32 = 3u32;
pub const TBCD_THUMB: u32 = 2u32;
pub const TBCD_TICS: u32 = 1u32;
pub const TBDDRET_DEFAULT: u32 = 0u32;
pub const TBDDRET_NODEFAULT: u32 = 1u32;
pub const TBDDRET_TREATPRESSED: u32 = 2u32;
#[repr(C)]
pub struct TBINSERTMARK {
    pub iButton: i32,
    pub dwFlags: TBINSERTMARK_FLAGS,
}
impl ::core::marker::Copy for TBINSERTMARK {}
impl ::core::clone::Clone for TBINSERTMARK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TBINSERTMARK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBINSERTMARK").field("iButton", &self.iButton).field("dwFlags", &self.dwFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for TBINSERTMARK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TBINSERTMARK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBINSERTMARK>()) == 0 }
    }
}
impl ::core::cmp::Eq for TBINSERTMARK {}
impl ::core::default::Default for TBINSERTMARK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TBINSERTMARK_FLAGS(pub u32);
pub const TBIMHT_NONE: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(0u32);
pub const TBIMHT_AFTER: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(1u32);
pub const TBIMHT_BACKGROUND: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(2u32);
impl ::core::marker::Copy for TBINSERTMARK_FLAGS {}
impl ::core::clone::Clone for TBINSERTMARK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TBINSERTMARK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TBINSERTMARK_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TBINSERTMARK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TBINSERTMARK_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct TBMETRICS {
    pub cbSize: u32,
    pub dwMask: u32,
    pub cxPad: i32,
    pub cyPad: i32,
    pub cxBarPad: i32,
    pub cyBarPad: i32,
    pub cxButtonSpacing: i32,
    pub cyButtonSpacing: i32,
}
impl ::core::marker::Copy for TBMETRICS {}
impl ::core::clone::Clone for TBMETRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TBMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBMETRICS").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("cxPad", &self.cxPad).field("cyPad", &self.cyPad).field("cxBarPad", &self.cxBarPad).field("cyBarPad", &self.cyBarPad).field("cxButtonSpacing", &self.cxButtonSpacing).field("cyButtonSpacing", &self.cyButtonSpacing).finish()
    }
}
unsafe impl ::windows_core::Abi for TBMETRICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TBMETRICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBMETRICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for TBMETRICS {}
impl ::core::default::Default for TBMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TBMF_BARPAD: u32 = 2u32;
pub const TBMF_BUTTONSPACING: u32 = 4u32;
pub const TBMF_PAD: u32 = 1u32;
pub const TBM_CLEARSEL: u32 = 1043u32;
pub const TBM_CLEARTICS: u32 = 1033u32;
pub const TBM_GETBUDDY: u32 = 1057u32;
pub const TBM_GETCHANNELRECT: u32 = 1050u32;
pub const TBM_GETLINESIZE: u32 = 1048u32;
pub const TBM_GETNUMTICS: u32 = 1040u32;
pub const TBM_GETPAGESIZE: u32 = 1046u32;
pub const TBM_GETPTICS: u32 = 1038u32;
pub const TBM_GETRANGEMAX: u32 = 1026u32;
pub const TBM_GETRANGEMIN: u32 = 1025u32;
pub const TBM_GETSELEND: u32 = 1042u32;
pub const TBM_GETSELSTART: u32 = 1041u32;
pub const TBM_GETTHUMBLENGTH: u32 = 1052u32;
pub const TBM_GETTHUMBRECT: u32 = 1049u32;
pub const TBM_GETTIC: u32 = 1027u32;
pub const TBM_GETTICPOS: u32 = 1039u32;
pub const TBM_GETTOOLTIPS: u32 = 1054u32;
pub const TBM_GETUNICODEFORMAT: u32 = 8198u32;
pub const TBM_SETBUDDY: u32 = 1056u32;
pub const TBM_SETLINESIZE: u32 = 1047u32;
pub const TBM_SETPAGESIZE: u32 = 1045u32;
pub const TBM_SETPOS: u32 = 1029u32;
pub const TBM_SETPOSNOTIFY: u32 = 1058u32;
pub const TBM_SETRANGE: u32 = 1030u32;
pub const TBM_SETRANGEMAX: u32 = 1032u32;
pub const TBM_SETRANGEMIN: u32 = 1031u32;
pub const TBM_SETSEL: u32 = 1034u32;
pub const TBM_SETSELEND: u32 = 1036u32;
pub const TBM_SETSELSTART: u32 = 1035u32;
pub const TBM_SETTHUMBLENGTH: u32 = 1051u32;
pub const TBM_SETTIC: u32 = 1028u32;
pub const TBM_SETTICFREQ: u32 = 1044u32;
pub const TBM_SETTIPSIDE: u32 = 1055u32;
pub const TBM_SETTOOLTIPS: u32 = 1053u32;
pub const TBM_SETUNICODEFORMAT: u32 = 8197u32;
pub const TBNRF_ENDCUSTOMIZE: u32 = 2u32;
pub const TBNRF_HIDEHELP: u32 = 1u32;
#[repr(C)]
pub struct TBREPLACEBITMAP {
    pub hInstOld: ::win32_foundation::HINSTANCE,
    pub nIDOld: usize,
    pub hInstNew: ::win32_foundation::HINSTANCE,
    pub nIDNew: usize,
    pub nButtons: i32,
}
impl ::core::marker::Copy for TBREPLACEBITMAP {}
impl ::core::clone::Clone for TBREPLACEBITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TBREPLACEBITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBREPLACEBITMAP").field("hInstOld", &self.hInstOld).field("nIDOld", &self.nIDOld).field("hInstNew", &self.hInstNew).field("nIDNew", &self.nIDNew).field("nButtons", &self.nButtons).finish()
    }
}
unsafe impl ::windows_core::Abi for TBREPLACEBITMAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TBREPLACEBITMAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBREPLACEBITMAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for TBREPLACEBITMAP {}
impl ::core::default::Default for TBREPLACEBITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
pub struct TBSAVEPARAMSA {
    pub hkr: ::win32_system::Registry::HKEY,
    pub pszSubKey: ::windows_core::PCSTR,
    pub pszValueName: ::windows_core::PCSTR,
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::marker::Copy for TBSAVEPARAMSA {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::clone::Clone for TBSAVEPARAMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::fmt::Debug for TBSAVEPARAMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBSAVEPARAMSA").field("hkr", &self.hkr).field("pszSubKey", &self.pszSubKey).field("pszValueName", &self.pszValueName).finish()
    }
}
#[cfg(feature = "Win32_System_Registry")]
unsafe impl ::windows_core::Abi for TBSAVEPARAMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::cmp::PartialEq for TBSAVEPARAMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBSAVEPARAMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::cmp::Eq for TBSAVEPARAMSA {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::default::Default for TBSAVEPARAMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
pub struct TBSAVEPARAMSW {
    pub hkr: ::win32_system::Registry::HKEY,
    pub pszSubKey: ::windows_core::PCWSTR,
    pub pszValueName: ::windows_core::PCWSTR,
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::marker::Copy for TBSAVEPARAMSW {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::clone::Clone for TBSAVEPARAMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::fmt::Debug for TBSAVEPARAMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TBSAVEPARAMSW").field("hkr", &self.hkr).field("pszSubKey", &self.pszSubKey).field("pszValueName", &self.pszValueName).finish()
    }
}
#[cfg(feature = "Win32_System_Registry")]
unsafe impl ::windows_core::Abi for TBSAVEPARAMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::cmp::PartialEq for TBSAVEPARAMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TBSAVEPARAMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::cmp::Eq for TBSAVEPARAMSW {}
#[cfg(feature = "Win32_System_Registry")]
impl ::core::default::Default for TBSAVEPARAMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TBSTATE_CHECKED: u32 = 1u32;
pub const TBSTATE_ELLIPSES: u32 = 64u32;
pub const TBSTATE_ENABLED: u32 = 4u32;
pub const TBSTATE_HIDDEN: u32 = 8u32;
pub const TBSTATE_INDETERMINATE: u32 = 16u32;
pub const TBSTATE_MARKED: u32 = 128u32;
pub const TBSTATE_PRESSED: u32 = 2u32;
pub const TBSTATE_WRAP: u32 = 32u32;
pub const TBSTYLE_ALTDRAG: u32 = 1024u32;
pub const TBSTYLE_AUTOSIZE: u32 = 16u32;
pub const TBSTYLE_BUTTON: u32 = 0u32;
pub const TBSTYLE_CHECK: u32 = 2u32;
pub const TBSTYLE_CUSTOMERASE: u32 = 8192u32;
pub const TBSTYLE_DROPDOWN: u32 = 8u32;
pub const TBSTYLE_EX_DOUBLEBUFFER: u32 = 128u32;
pub const TBSTYLE_EX_DRAWDDARROWS: u32 = 1u32;
pub const TBSTYLE_EX_HIDECLIPPEDBUTTONS: u32 = 16u32;
pub const TBSTYLE_EX_MIXEDBUTTONS: u32 = 8u32;
pub const TBSTYLE_EX_MULTICOLUMN: u32 = 2u32;
pub const TBSTYLE_EX_VERTICAL: u32 = 4u32;
pub const TBSTYLE_FLAT: u32 = 2048u32;
pub const TBSTYLE_GROUP: u32 = 4u32;
pub const TBSTYLE_LIST: u32 = 4096u32;
pub const TBSTYLE_NOPREFIX: u32 = 32u32;
pub const TBSTYLE_REGISTERDROP: u32 = 16384u32;
pub const TBSTYLE_SEP: u32 = 1u32;
pub const TBSTYLE_TOOLTIPS: u32 = 256u32;
pub const TBSTYLE_TRANSPARENT: u32 = 32768u32;
pub const TBSTYLE_WRAPABLE: u32 = 512u32;
pub const TBS_AUTOTICKS: u32 = 1u32;
pub const TBS_BOTH: u32 = 8u32;
pub const TBS_BOTTOM: u32 = 0u32;
pub const TBS_DOWNISLEFT: u32 = 1024u32;
pub const TBS_ENABLESELRANGE: u32 = 32u32;
pub const TBS_FIXEDLENGTH: u32 = 64u32;
pub const TBS_HORZ: u32 = 0u32;
pub const TBS_LEFT: u32 = 4u32;
pub const TBS_NOTHUMB: u32 = 128u32;
pub const TBS_NOTICKS: u32 = 16u32;
pub const TBS_NOTIFYBEFOREMOVE: u32 = 2048u32;
pub const TBS_REVERSED: u32 = 512u32;
pub const TBS_RIGHT: u32 = 0u32;
pub const TBS_TOOLTIPS: u32 = 256u32;
pub const TBS_TOP: u32 = 4u32;
pub const TBS_TRANSPARENTBKGND: u32 = 4096u32;
pub const TBS_VERT: u32 = 2u32;
pub const TBTS_BOTTOM: u32 = 2u32;
pub const TBTS_LEFT: u32 = 1u32;
pub const TBTS_RIGHT: u32 = 3u32;
pub const TBTS_TOP: u32 = 0u32;
pub const TB_ADDBITMAP: u32 = 1043u32;
pub const TB_ADDBUTTONS: u32 = 1092u32;
pub const TB_ADDBUTTONSA: u32 = 1044u32;
pub const TB_ADDBUTTONSW: u32 = 1092u32;
pub const TB_ADDSTRING: u32 = 1101u32;
pub const TB_ADDSTRINGA: u32 = 1052u32;
pub const TB_ADDSTRINGW: u32 = 1101u32;
pub const TB_AUTOSIZE: u32 = 1057u32;
pub const TB_BOTTOM: u32 = 7u32;
pub const TB_BUTTONCOUNT: u32 = 1048u32;
pub const TB_BUTTONSTRUCTSIZE: u32 = 1054u32;
pub const TB_CHANGEBITMAP: u32 = 1067u32;
pub const TB_CHECKBUTTON: u32 = 1026u32;
pub const TB_COMMANDTOINDEX: u32 = 1049u32;
pub const TB_CUSTOMIZE: u32 = 1051u32;
pub const TB_DELETEBUTTON: u32 = 1046u32;
pub const TB_ENABLEBUTTON: u32 = 1025u32;
pub const TB_ENDTRACK: u32 = 8u32;
pub const TB_GETANCHORHIGHLIGHT: u32 = 1098u32;
pub const TB_GETBITMAP: u32 = 1068u32;
pub const TB_GETBITMAPFLAGS: u32 = 1065u32;
pub const TB_GETBUTTON: u32 = 1047u32;
pub const TB_GETBUTTONINFO: u32 = 1087u32;
pub const TB_GETBUTTONINFOA: u32 = 1089u32;
pub const TB_GETBUTTONINFOW: u32 = 1087u32;
pub const TB_GETBUTTONSIZE: u32 = 1082u32;
pub const TB_GETBUTTONTEXT: u32 = 1099u32;
pub const TB_GETBUTTONTEXTA: u32 = 1069u32;
pub const TB_GETBUTTONTEXTW: u32 = 1099u32;
pub const TB_GETCOLORSCHEME: u32 = 8195u32;
pub const TB_GETDISABLEDIMAGELIST: u32 = 1079u32;
pub const TB_GETEXTENDEDSTYLE: u32 = 1109u32;
pub const TB_GETHOTIMAGELIST: u32 = 1077u32;
pub const TB_GETHOTITEM: u32 = 1095u32;
pub const TB_GETIDEALSIZE: u32 = 1123u32;
pub const TB_GETIMAGELIST: u32 = 1073u32;
pub const TB_GETIMAGELISTCOUNT: u32 = 1122u32;
pub const TB_GETINSERTMARK: u32 = 1103u32;
pub const TB_GETINSERTMARKCOLOR: u32 = 1113u32;
pub const TB_GETITEMDROPDOWNRECT: u32 = 1127u32;
pub const TB_GETITEMRECT: u32 = 1053u32;
pub const TB_GETMAXSIZE: u32 = 1107u32;
pub const TB_GETMETRICS: u32 = 1125u32;
pub const TB_GETOBJECT: u32 = 1086u32;
pub const TB_GETPADDING: u32 = 1110u32;
pub const TB_GETPRESSEDIMAGELIST: u32 = 1129u32;
pub const TB_GETRECT: u32 = 1075u32;
pub const TB_GETROWS: u32 = 1064u32;
pub const TB_GETSTATE: u32 = 1042u32;
pub const TB_GETSTRING: u32 = 1115u32;
pub const TB_GETSTRINGA: u32 = 1116u32;
pub const TB_GETSTRINGW: u32 = 1115u32;
pub const TB_GETSTYLE: u32 = 1081u32;
pub const TB_GETTEXTROWS: u32 = 1085u32;
pub const TB_GETTOOLTIPS: u32 = 1059u32;
pub const TB_GETUNICODEFORMAT: u32 = 8198u32;
pub const TB_HASACCELERATOR: u32 = 1119u32;
pub const TB_HIDEBUTTON: u32 = 1028u32;
pub const TB_HITTEST: u32 = 1093u32;
pub const TB_INDETERMINATE: u32 = 1029u32;
pub const TB_INSERTBUTTON: u32 = 1091u32;
pub const TB_INSERTBUTTONA: u32 = 1045u32;
pub const TB_INSERTBUTTONW: u32 = 1091u32;
pub const TB_INSERTMARKHITTEST: u32 = 1105u32;
pub const TB_ISBUTTONCHECKED: u32 = 1034u32;
pub const TB_ISBUTTONENABLED: u32 = 1033u32;
pub const TB_ISBUTTONHIDDEN: u32 = 1036u32;
pub const TB_ISBUTTONHIGHLIGHTED: u32 = 1038u32;
pub const TB_ISBUTTONINDETERMINATE: u32 = 1037u32;
pub const TB_ISBUTTONPRESSED: u32 = 1035u32;
pub const TB_LINEDOWN: u32 = 1u32;
pub const TB_LINEUP: u32 = 0u32;
pub const TB_LOADIMAGES: u32 = 1074u32;
pub const TB_MAPACCELERATOR: u32 = 1114u32;
pub const TB_MAPACCELERATORA: u32 = 1102u32;
pub const TB_MAPACCELERATORW: u32 = 1114u32;
pub const TB_MARKBUTTON: u32 = 1030u32;
pub const TB_MOVEBUTTON: u32 = 1106u32;
pub const TB_PAGEDOWN: u32 = 3u32;
pub const TB_PAGEUP: u32 = 2u32;
pub const TB_PRESSBUTTON: u32 = 1027u32;
pub const TB_REPLACEBITMAP: u32 = 1070u32;
pub const TB_SAVERESTORE: u32 = 1100u32;
pub const TB_SAVERESTOREA: u32 = 1050u32;
pub const TB_SAVERESTOREW: u32 = 1100u32;
pub const TB_SETANCHORHIGHLIGHT: u32 = 1097u32;
pub const TB_SETBITMAPSIZE: u32 = 1056u32;
pub const TB_SETBOUNDINGSIZE: u32 = 1117u32;
pub const TB_SETBUTTONINFO: u32 = 1088u32;
pub const TB_SETBUTTONINFOA: u32 = 1090u32;
pub const TB_SETBUTTONINFOW: u32 = 1088u32;
pub const TB_SETBUTTONSIZE: u32 = 1055u32;
pub const TB_SETBUTTONWIDTH: u32 = 1083u32;
pub const TB_SETCMDID: u32 = 1066u32;
pub const TB_SETCOLORSCHEME: u32 = 8194u32;
pub const TB_SETDISABLEDIMAGELIST: u32 = 1078u32;
pub const TB_SETDRAWTEXTFLAGS: u32 = 1094u32;
pub const TB_SETEXTENDEDSTYLE: u32 = 1108u32;
pub const TB_SETHOTIMAGELIST: u32 = 1076u32;
pub const TB_SETHOTITEM: u32 = 1096u32;
pub const TB_SETHOTITEM2: u32 = 1118u32;
pub const TB_SETIMAGELIST: u32 = 1072u32;
pub const TB_SETINDENT: u32 = 1071u32;
pub const TB_SETINSERTMARK: u32 = 1104u32;
pub const TB_SETINSERTMARKCOLOR: u32 = 1112u32;
pub const TB_SETLISTGAP: u32 = 1120u32;
pub const TB_SETMAXTEXTROWS: u32 = 1084u32;
pub const TB_SETMETRICS: u32 = 1126u32;
pub const TB_SETPADDING: u32 = 1111u32;
pub const TB_SETPARENT: u32 = 1061u32;
pub const TB_SETPRESSEDIMAGELIST: u32 = 1128u32;
pub const TB_SETROWS: u32 = 1063u32;
pub const TB_SETSTATE: u32 = 1041u32;
pub const TB_SETSTYLE: u32 = 1080u32;
pub const TB_SETTOOLTIPS: u32 = 1060u32;
pub const TB_SETUNICODEFORMAT: u32 = 8197u32;
pub const TB_SETWINDOWTHEME: u32 = 8203u32;
pub const TB_THUMBPOSITION: u32 = 4u32;
pub const TB_THUMBTRACK: u32 = 5u32;
pub const TB_TOP: u32 = 6u32;
#[repr(C)]
pub struct TCHITTESTINFO {
    pub pt: ::win32_foundation::POINT,
    pub flags: TCHITTESTINFO_FLAGS,
}
impl ::core::marker::Copy for TCHITTESTINFO {}
impl ::core::clone::Clone for TCHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows_core::Abi for TCHITTESTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCHITTESTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCHITTESTINFO {}
impl ::core::default::Default for TCHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TCHITTESTINFO_FLAGS(pub u32);
pub const TCHT_NOWHERE: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(1u32);
pub const TCHT_ONITEM: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(6u32);
pub const TCHT_ONITEMICON: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(2u32);
pub const TCHT_ONITEMLABEL: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(4u32);
impl ::core::marker::Copy for TCHITTESTINFO_FLAGS {}
impl ::core::clone::Clone for TCHITTESTINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TCHITTESTINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TCHITTESTINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TCHITTESTINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCHITTESTINFO_FLAGS").field(&self.0).finish()
    }
}
pub const TCIS_BUTTONPRESSED: u32 = 1u32;
pub const TCIS_HIGHLIGHTED: u32 = 2u32;
#[repr(C)]
pub struct TCITEMA {
    pub mask: TCITEMHEADERA_MASK,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub pszText: ::windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for TCITEMA {}
impl ::core::clone::Clone for TCITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCITEMA").field("mask", &self.mask).field("dwState", &self.dwState).field("dwStateMask", &self.dwStateMask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for TCITEMA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCITEMA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCITEMA>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCITEMA {}
impl ::core::default::Default for TCITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TCITEMHEADERA {
    pub mask: TCITEMHEADERA_MASK,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: ::windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
}
impl ::core::marker::Copy for TCITEMHEADERA {}
impl ::core::clone::Clone for TCITEMHEADERA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCITEMHEADERA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCITEMHEADERA").field("mask", &self.mask).field("lpReserved1", &self.lpReserved1).field("lpReserved2", &self.lpReserved2).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).finish()
    }
}
unsafe impl ::windows_core::Abi for TCITEMHEADERA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCITEMHEADERA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCITEMHEADERA>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCITEMHEADERA {}
impl ::core::default::Default for TCITEMHEADERA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TCITEMHEADERA_MASK(pub u32);
pub const TCIF_IMAGE: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(2u32);
pub const TCIF_RTLREADING: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(4u32);
pub const TCIF_TEXT: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(1u32);
pub const TCIF_PARAM: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(8u32);
pub const TCIF_STATE: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(16u32);
impl ::core::marker::Copy for TCITEMHEADERA_MASK {}
impl ::core::clone::Clone for TCITEMHEADERA_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TCITEMHEADERA_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TCITEMHEADERA_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for TCITEMHEADERA_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TCITEMHEADERA_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TCITEMHEADERA_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TCITEMHEADERA_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TCITEMHEADERA_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TCITEMHEADERA_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TCITEMHEADERA_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
pub struct TCITEMHEADERW {
    pub mask: TCITEMHEADERA_MASK,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
}
impl ::core::marker::Copy for TCITEMHEADERW {}
impl ::core::clone::Clone for TCITEMHEADERW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCITEMHEADERW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCITEMHEADERW").field("mask", &self.mask).field("lpReserved1", &self.lpReserved1).field("lpReserved2", &self.lpReserved2).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).finish()
    }
}
unsafe impl ::windows_core::Abi for TCITEMHEADERW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCITEMHEADERW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCITEMHEADERW>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCITEMHEADERW {}
impl ::core::default::Default for TCITEMHEADERW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TCITEMW {
    pub mask: TCITEMHEADERA_MASK,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for TCITEMW {}
impl ::core::clone::Clone for TCITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TCITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCITEMW").field("mask", &self.mask).field("dwState", &self.dwState).field("dwStateMask", &self.dwStateMask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for TCITEMW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCITEMW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCITEMW>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCITEMW {}
impl ::core::default::Default for TCITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TCM_ADJUSTRECT: u32 = 4904u32;
pub const TCM_DELETEALLITEMS: u32 = 4873u32;
pub const TCM_DELETEITEM: u32 = 4872u32;
pub const TCM_DESELECTALL: u32 = 4914u32;
pub const TCM_FIRST: u32 = 4864u32;
pub const TCM_GETCURFOCUS: u32 = 4911u32;
pub const TCM_GETCURSEL: u32 = 4875u32;
pub const TCM_GETEXTENDEDSTYLE: u32 = 4917u32;
pub const TCM_GETIMAGELIST: u32 = 4866u32;
pub const TCM_GETITEM: u32 = 4924u32;
pub const TCM_GETITEMA: u32 = 4869u32;
pub const TCM_GETITEMCOUNT: u32 = 4868u32;
pub const TCM_GETITEMRECT: u32 = 4874u32;
pub const TCM_GETITEMW: u32 = 4924u32;
pub const TCM_GETROWCOUNT: u32 = 4908u32;
pub const TCM_GETTOOLTIPS: u32 = 4909u32;
pub const TCM_GETUNICODEFORMAT: u32 = 8198u32;
pub const TCM_HIGHLIGHTITEM: u32 = 4915u32;
pub const TCM_HITTEST: u32 = 4877u32;
pub const TCM_INSERTITEM: u32 = 4926u32;
pub const TCM_INSERTITEMA: u32 = 4871u32;
pub const TCM_INSERTITEMW: u32 = 4926u32;
pub const TCM_REMOVEIMAGE: u32 = 4906u32;
pub const TCM_SETCURFOCUS: u32 = 4912u32;
pub const TCM_SETCURSEL: u32 = 4876u32;
pub const TCM_SETEXTENDEDSTYLE: u32 = 4916u32;
pub const TCM_SETIMAGELIST: u32 = 4867u32;
pub const TCM_SETITEM: u32 = 4925u32;
pub const TCM_SETITEMA: u32 = 4870u32;
pub const TCM_SETITEMEXTRA: u32 = 4878u32;
pub const TCM_SETITEMSIZE: u32 = 4905u32;
pub const TCM_SETITEMW: u32 = 4925u32;
pub const TCM_SETMINTABWIDTH: u32 = 4913u32;
pub const TCM_SETPADDING: u32 = 4907u32;
pub const TCM_SETTOOLTIPS: u32 = 4910u32;
pub const TCM_SETUNICODEFORMAT: u32 = 8197u32;
pub const TCS_BOTTOM: u32 = 2u32;
pub const TCS_BUTTONS: u32 = 256u32;
pub const TCS_EX_FLATSEPARATORS: u32 = 1u32;
pub const TCS_EX_REGISTERDROP: u32 = 2u32;
pub const TCS_FIXEDWIDTH: u32 = 1024u32;
pub const TCS_FLATBUTTONS: u32 = 8u32;
pub const TCS_FOCUSNEVER: u32 = 32768u32;
pub const TCS_FOCUSONBUTTONDOWN: u32 = 4096u32;
pub const TCS_FORCEICONLEFT: u32 = 16u32;
pub const TCS_FORCELABELLEFT: u32 = 32u32;
pub const TCS_HOTTRACK: u32 = 64u32;
pub const TCS_MULTILINE: u32 = 512u32;
pub const TCS_MULTISELECT: u32 = 4u32;
pub const TCS_OWNERDRAWFIXED: u32 = 8192u32;
pub const TCS_RAGGEDRIGHT: u32 = 2048u32;
pub const TCS_RIGHT: u32 = 2u32;
pub const TCS_RIGHTJUSTIFY: u32 = 0u32;
pub const TCS_SCROLLOPPOSITE: u32 = 1u32;
pub const TCS_SINGLELINE: u32 = 0u32;
pub const TCS_TABS: u32 = 0u32;
pub const TCS_TOOLTIPS: u32 = 16384u32;
pub const TCS_VERTICAL: u32 = 128u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TEXTSHADOWTYPE(pub i32);
pub const TST_NONE: TEXTSHADOWTYPE = TEXTSHADOWTYPE(0i32);
pub const TST_SINGLE: TEXTSHADOWTYPE = TEXTSHADOWTYPE(1i32);
pub const TST_CONTINUOUS: TEXTSHADOWTYPE = TEXTSHADOWTYPE(2i32);
impl ::core::marker::Copy for TEXTSHADOWTYPE {}
impl ::core::clone::Clone for TEXTSHADOWTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TEXTSHADOWTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TEXTSHADOWTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TEXTSHADOWTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXTSHADOWTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct THEMESIZE(pub i32);
pub const TS_MIN: THEMESIZE = THEMESIZE(0i32);
pub const TS_TRUE: THEMESIZE = THEMESIZE(1i32);
pub const TS_DRAW: THEMESIZE = THEMESIZE(2i32);
impl ::core::marker::Copy for THEMESIZE {}
impl ::core::clone::Clone for THEMESIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THEMESIZE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for THEMESIZE {
    type Abi = Self;
}
impl ::core::fmt::Debug for THEMESIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THEMESIZE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct THEME_PROPERTY_SYMBOL_ID(pub u32);
pub const TMT_RESERVEDLOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(0u32);
pub const TMT_RESERVEDHIGH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(7999u32);
pub const TMT_DIBDATA: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2u32);
pub const TMT_GLYPHDIBDATA: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8u32);
pub const TMT_ENUM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(200u32);
pub const TMT_STRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(201u32);
pub const TMT_INT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(202u32);
pub const TMT_BOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(203u32);
pub const TMT_COLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(204u32);
pub const TMT_MARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(205u32);
pub const TMT_FILENAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(206u32);
pub const TMT_SIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(207u32);
pub const TMT_POSITION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(208u32);
pub const TMT_RECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(209u32);
pub const TMT_FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(210u32);
pub const TMT_INTLIST: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(211u32);
pub const TMT_HBITMAP: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(212u32);
pub const TMT_DISKSTREAM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(213u32);
pub const TMT_STREAM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(214u32);
pub const TMT_BITMAPREF: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(215u32);
pub const TMT_FLOAT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(216u32);
pub const TMT_FLOATLIST: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(217u32);
pub const TMT_COLORSCHEMES: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(401u32);
pub const TMT_SIZES: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(402u32);
pub const TMT_CHARSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(403u32);
pub const TMT_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(600u32);
pub const TMT_DISPLAYNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(601u32);
pub const TMT_TOOLTIP: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(602u32);
pub const TMT_COMPANY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(603u32);
pub const TMT_AUTHOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(604u32);
pub const TMT_COPYRIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(605u32);
pub const TMT_URL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(606u32);
pub const TMT_VERSION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(607u32);
pub const TMT_DESCRIPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(608u32);
pub const TMT_FIRST_RCSTRING_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(601u32);
pub const TMT_LAST_RCSTRING_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(608u32);
pub const TMT_CAPTIONFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(801u32);
pub const TMT_SMALLCAPTIONFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(802u32);
pub const TMT_MENUFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(803u32);
pub const TMT_STATUSFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(804u32);
pub const TMT_MSGBOXFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(805u32);
pub const TMT_ICONTITLEFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(806u32);
pub const TMT_HEADING1FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(807u32);
pub const TMT_HEADING2FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(808u32);
pub const TMT_BODYFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(809u32);
pub const TMT_FIRSTFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(801u32);
pub const TMT_LASTFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(809u32);
pub const TMT_FLATMENUS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
pub const TMT_FIRSTBOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
pub const TMT_LASTBOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
pub const TMT_SIZINGBORDERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1201u32);
pub const TMT_SCROLLBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1202u32);
pub const TMT_SCROLLBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1203u32);
pub const TMT_CAPTIONBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1204u32);
pub const TMT_CAPTIONBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1205u32);
pub const TMT_SMCAPTIONBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1206u32);
pub const TMT_SMCAPTIONBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1207u32);
pub const TMT_MENUBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1208u32);
pub const TMT_MENUBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1209u32);
pub const TMT_PADDEDBORDERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1210u32);
pub const TMT_FIRSTSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1201u32);
pub const TMT_LASTSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1210u32);
pub const TMT_MINCOLORDEPTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
pub const TMT_FIRSTINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
pub const TMT_LASTINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
pub const TMT_CSSNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1401u32);
pub const TMT_XMLNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1402u32);
pub const TMT_LASTUPDATED: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1403u32);
pub const TMT_ALIAS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1404u32);
pub const TMT_FIRSTSTRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1401u32);
pub const TMT_LASTSTRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1404u32);
pub const TMT_SCROLLBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1601u32);
pub const TMT_BACKGROUND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1602u32);
pub const TMT_ACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1603u32);
pub const TMT_INACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1604u32);
pub const TMT_MENU: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1605u32);
pub const TMT_WINDOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1606u32);
pub const TMT_WINDOWFRAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1607u32);
pub const TMT_MENUTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1608u32);
pub const TMT_WINDOWTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1609u32);
pub const TMT_CAPTIONTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1610u32);
pub const TMT_ACTIVEBORDER: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1611u32);
pub const TMT_INACTIVEBORDER: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1612u32);
pub const TMT_APPWORKSPACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1613u32);
pub const TMT_HIGHLIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1614u32);
pub const TMT_HIGHLIGHTTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1615u32);
pub const TMT_BTNFACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1616u32);
pub const TMT_BTNSHADOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1617u32);
pub const TMT_GRAYTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1618u32);
pub const TMT_BTNTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1619u32);
pub const TMT_INACTIVECAPTIONTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1620u32);
pub const TMT_BTNHIGHLIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1621u32);
pub const TMT_DKSHADOW3D: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1622u32);
pub const TMT_LIGHT3D: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1623u32);
pub const TMT_INFOTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1624u32);
pub const TMT_INFOBK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1625u32);
pub const TMT_BUTTONALTERNATEFACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1626u32);
pub const TMT_HOTTRACKING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1627u32);
pub const TMT_GRADIENTACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1628u32);
pub const TMT_GRADIENTINACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1629u32);
pub const TMT_MENUHILIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1630u32);
pub const TMT_MENUBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1631u32);
pub const TMT_FIRSTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1601u32);
pub const TMT_LASTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1631u32);
pub const TMT_FROMHUE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1801u32);
pub const TMT_FROMHUE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1802u32);
pub const TMT_FROMHUE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1803u32);
pub const TMT_FROMHUE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1804u32);
pub const TMT_FROMHUE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1805u32);
pub const TMT_TOHUE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1806u32);
pub const TMT_TOHUE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1807u32);
pub const TMT_TOHUE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1808u32);
pub const TMT_TOHUE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1809u32);
pub const TMT_TOHUE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1810u32);
pub const TMT_FROMCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2001u32);
pub const TMT_FROMCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2002u32);
pub const TMT_FROMCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2003u32);
pub const TMT_FROMCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2004u32);
pub const TMT_FROMCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2005u32);
pub const TMT_TOCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2006u32);
pub const TMT_TOCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2007u32);
pub const TMT_TOCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2008u32);
pub const TMT_TOCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2009u32);
pub const TMT_TOCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2010u32);
pub const TMT_TRANSPARENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2201u32);
pub const TMT_AUTOSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2202u32);
pub const TMT_BORDERONLY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2203u32);
pub const TMT_COMPOSITED: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2204u32);
pub const TMT_BGFILL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2205u32);
pub const TMT_GLYPHTRANSPARENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2206u32);
pub const TMT_GLYPHONLY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2207u32);
pub const TMT_ALWAYSSHOWSIZINGBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2208u32);
pub const TMT_MIRRORIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2209u32);
pub const TMT_UNIFORMSIZING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2210u32);
pub const TMT_INTEGRALSIZING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2211u32);
pub const TMT_SOURCEGROW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2212u32);
pub const TMT_SOURCESHRINK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2213u32);
pub const TMT_DRAWBORDERS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2214u32);
pub const TMT_NOETCHEDEFFECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2215u32);
pub const TMT_TEXTAPPLYOVERLAY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2216u32);
pub const TMT_TEXTGLOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2217u32);
pub const TMT_TEXTITALIC: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2218u32);
pub const TMT_COMPOSITEDOPAQUE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2219u32);
pub const TMT_LOCALIZEDMIRRORIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2220u32);
pub const TMT_IMAGECOUNT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2401u32);
pub const TMT_ALPHALEVEL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2402u32);
pub const TMT_BORDERSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2403u32);
pub const TMT_ROUNDCORNERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2404u32);
pub const TMT_ROUNDCORNERHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2405u32);
pub const TMT_GRADIENTRATIO1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2406u32);
pub const TMT_GRADIENTRATIO2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2407u32);
pub const TMT_GRADIENTRATIO3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2408u32);
pub const TMT_GRADIENTRATIO4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2409u32);
pub const TMT_GRADIENTRATIO5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2410u32);
pub const TMT_PROGRESSCHUNKSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2411u32);
pub const TMT_PROGRESSSPACESIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2412u32);
pub const TMT_SATURATION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2413u32);
pub const TMT_TEXTBORDERSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2414u32);
pub const TMT_ALPHATHRESHOLD: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2415u32);
pub const TMT_WIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2416u32);
pub const TMT_HEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2417u32);
pub const TMT_GLYPHINDEX: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2418u32);
pub const TMT_TRUESIZESTRETCHMARK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2419u32);
pub const TMT_MINDPI1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2420u32);
pub const TMT_MINDPI2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2421u32);
pub const TMT_MINDPI3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2422u32);
pub const TMT_MINDPI4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2423u32);
pub const TMT_MINDPI5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2424u32);
pub const TMT_TEXTGLOWSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2425u32);
pub const TMT_FRAMESPERSECOND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2426u32);
pub const TMT_PIXELSPERFRAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2427u32);
pub const TMT_ANIMATIONDELAY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2428u32);
pub const TMT_GLOWINTENSITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2429u32);
pub const TMT_OPACITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2430u32);
pub const TMT_COLORIZATIONCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2431u32);
pub const TMT_COLORIZATIONOPACITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2432u32);
pub const TMT_MINDPI6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2433u32);
pub const TMT_MINDPI7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2434u32);
pub const TMT_GLYPHFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2601u32);
pub const TMT_IMAGEFILE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3001u32);
pub const TMT_IMAGEFILE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3002u32);
pub const TMT_IMAGEFILE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3003u32);
pub const TMT_IMAGEFILE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3004u32);
pub const TMT_IMAGEFILE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3005u32);
pub const TMT_IMAGEFILE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3006u32);
pub const TMT_GLYPHIMAGEFILE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3008u32);
pub const TMT_IMAGEFILE6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3009u32);
pub const TMT_IMAGEFILE7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3010u32);
pub const TMT_TEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3201u32);
pub const TMT_CLASSICVALUE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3202u32);
pub const TMT_OFFSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3401u32);
pub const TMT_TEXTSHADOWOFFSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3402u32);
pub const TMT_MINSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3403u32);
pub const TMT_MINSIZE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3404u32);
pub const TMT_MINSIZE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3405u32);
pub const TMT_MINSIZE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3406u32);
pub const TMT_MINSIZE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3407u32);
pub const TMT_MINSIZE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3408u32);
pub const TMT_NORMALSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3409u32);
pub const TMT_MINSIZE6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3410u32);
pub const TMT_MINSIZE7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3411u32);
pub const TMT_SIZINGMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3601u32);
pub const TMT_CONTENTMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3602u32);
pub const TMT_CAPTIONMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3603u32);
pub const TMT_BORDERCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3801u32);
pub const TMT_FILLCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3802u32);
pub const TMT_TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3803u32);
pub const TMT_EDGELIGHTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3804u32);
pub const TMT_EDGEHIGHLIGHTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3805u32);
pub const TMT_EDGESHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3806u32);
pub const TMT_EDGEDKSHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3807u32);
pub const TMT_EDGEFILLCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3808u32);
pub const TMT_TRANSPARENTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3809u32);
pub const TMT_GRADIENTCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3810u32);
pub const TMT_GRADIENTCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3811u32);
pub const TMT_GRADIENTCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3812u32);
pub const TMT_GRADIENTCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3813u32);
pub const TMT_GRADIENTCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3814u32);
pub const TMT_SHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3815u32);
pub const TMT_GLOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3816u32);
pub const TMT_TEXTBORDERCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3817u32);
pub const TMT_TEXTSHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3818u32);
pub const TMT_GLYPHTEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3819u32);
pub const TMT_GLYPHTRANSPARENTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3820u32);
pub const TMT_FILLCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3821u32);
pub const TMT_BORDERCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3822u32);
pub const TMT_ACCENTCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3823u32);
pub const TMT_TEXTCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3824u32);
pub const TMT_HEADING1TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3825u32);
pub const TMT_HEADING2TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3826u32);
pub const TMT_BODYTEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3827u32);
pub const TMT_BGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4001u32);
pub const TMT_BORDERTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4002u32);
pub const TMT_FILLTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4003u32);
pub const TMT_SIZINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4004u32);
pub const TMT_HALIGN: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4005u32);
pub const TMT_CONTENTALIGNMENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4006u32);
pub const TMT_VALIGN: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4007u32);
pub const TMT_OFFSETTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4008u32);
pub const TMT_ICONEFFECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4009u32);
pub const TMT_TEXTSHADOWTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4010u32);
pub const TMT_IMAGELAYOUT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4011u32);
pub const TMT_GLYPHTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4012u32);
pub const TMT_IMAGESELECTTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4013u32);
pub const TMT_GLYPHFONTSIZINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4014u32);
pub const TMT_TRUESIZESCALINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4015u32);
pub const TMT_USERPICTURE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5001u32);
pub const TMT_DEFAULTPANESIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5002u32);
pub const TMT_BLENDCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5003u32);
pub const TMT_CUSTOMSPLITRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5004u32);
pub const TMT_ANIMATIONBUTTONRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5005u32);
pub const TMT_ANIMATIONDURATION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5006u32);
pub const TMT_TRANSITIONDURATIONS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(6000u32);
pub const TMT_SCALEDBACKGROUND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(7001u32);
pub const TMT_ATLASIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8000u32);
pub const TMT_ATLASINPUTIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8001u32);
pub const TMT_ATLASRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8002u32);
impl ::core::marker::Copy for THEME_PROPERTY_SYMBOL_ID {}
impl ::core::clone::Clone for THEME_PROPERTY_SYMBOL_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THEME_PROPERTY_SYMBOL_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for THEME_PROPERTY_SYMBOL_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for THEME_PROPERTY_SYMBOL_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THEME_PROPERTY_SYMBOL_ID").field(&self.0).finish()
    }
}
pub const TOOLBARCLASSNAME: &str = "ToolbarWindow32";
pub const TOOLBARCLASSNAMEA: &str = "ToolbarWindow32";
pub const TOOLBARCLASSNAMEW: &str = "ToolbarWindow32";
pub const TOOLTIPS_CLASS: &str = "tooltips_class32";
pub const TOOLTIPS_CLASSA: &str = "tooltips_class32";
pub const TOOLTIPS_CLASSW: &str = "tooltips_class32";
#[repr(C)]
pub struct TOUCH_HIT_TESTING_INPUT {
    pub pointerId: u32,
    pub point: ::win32_foundation::POINT,
    pub boundingBox: ::win32_foundation::RECT,
    pub nonOccludedBoundingBox: ::win32_foundation::RECT,
    pub orientation: u32,
}
impl ::core::marker::Copy for TOUCH_HIT_TESTING_INPUT {}
impl ::core::clone::Clone for TOUCH_HIT_TESTING_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOUCH_HIT_TESTING_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOUCH_HIT_TESTING_INPUT").field("pointerId", &self.pointerId).field("point", &self.point).field("boundingBox", &self.boundingBox).field("nonOccludedBoundingBox", &self.nonOccludedBoundingBox).field("orientation", &self.orientation).finish()
    }
}
unsafe impl ::windows_core::Abi for TOUCH_HIT_TESTING_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TOUCH_HIT_TESTING_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TOUCH_HIT_TESTING_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for TOUCH_HIT_TESTING_INPUT {}
impl ::core::default::Default for TOUCH_HIT_TESTING_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    pub score: u16,
    pub adjustedPoint: ::win32_foundation::POINT,
}
impl ::core::marker::Copy for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {}
impl ::core::clone::Clone for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOUCH_HIT_TESTING_PROXIMITY_EVALUATION").field("score", &self.score).field("adjustedPoint", &self.adjustedPoint).finish()
    }
}
unsafe impl ::windows_core::Abi for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TOUCH_HIT_TESTING_PROXIMITY_EVALUATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {}
impl ::core::default::Default for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TRACKBAR_CLASS: &str = "msctls_trackbar32";
pub const TRACKBAR_CLASSA: &str = "msctls_trackbar32";
pub const TRACKBAR_CLASSW: &str = "msctls_trackbar32";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TRAILINGGRIDCELLSTATES(pub i32);
pub const MCTGC_HOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(1i32);
pub const MCTGC_HASSTATE: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(2i32);
pub const MCTGC_HASSTATEHOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(3i32);
pub const MCTGC_TODAY: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(4i32);
pub const MCTGC_TODAYSELECTED: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(5i32);
pub const MCTGC_SELECTED: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(6i32);
pub const MCTGC_SELECTEDHOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(7i32);
impl ::core::marker::Copy for TRAILINGGRIDCELLSTATES {}
impl ::core::clone::Clone for TRAILINGGRIDCELLSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRAILINGGRIDCELLSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TRAILINGGRIDCELLSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for TRAILINGGRIDCELLSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRAILINGGRIDCELLSTATES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TRAILINGGRIDCELLUPPERSTATES(pub i32);
pub const MCTGCU_HOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(1i32);
pub const MCTGCU_HASSTATE: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(2i32);
pub const MCTGCU_HASSTATEHOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(3i32);
pub const MCTGCU_SELECTED: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(4i32);
pub const MCTGCU_SELECTEDHOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(5i32);
impl ::core::marker::Copy for TRAILINGGRIDCELLUPPERSTATES {}
impl ::core::clone::Clone for TRAILINGGRIDCELLUPPERSTATES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRAILINGGRIDCELLUPPERSTATES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TRAILINGGRIDCELLUPPERSTATES {
    type Abi = Self;
}
impl ::core::fmt::Debug for TRAILINGGRIDCELLUPPERSTATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRAILINGGRIDCELLUPPERSTATES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TRAYNOTIFYPARTS(pub i32);
pub const TNP_BACKGROUND: TRAYNOTIFYPARTS = TRAYNOTIFYPARTS(1i32);
pub const TNP_ANIMBACKGROUND: TRAYNOTIFYPARTS = TRAYNOTIFYPARTS(2i32);
impl ::core::marker::Copy for TRAYNOTIFYPARTS {}
impl ::core::clone::Clone for TRAYNOTIFYPARTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRAYNOTIFYPARTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TRAYNOTIFYPARTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TRAYNOTIFYPARTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRAYNOTIFYPARTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TRUESIZESCALINGTYPE(pub i32);
pub const TSST_NONE: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(0i32);
pub const TSST_SIZE: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(1i32);
pub const TSST_DPI: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(2i32);
impl ::core::marker::Copy for TRUESIZESCALINGTYPE {}
impl ::core::clone::Clone for TRUESIZESCALINGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRUESIZESCALINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TRUESIZESCALINGTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TRUESIZESCALINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRUESIZESCALINGTYPE").field(&self.0).finish()
    }
}
pub const TTDT_AUTOMATIC: u32 = 0u32;
pub const TTDT_AUTOPOP: u32 = 2u32;
pub const TTDT_INITIAL: u32 = 3u32;
pub const TTDT_RESHOW: u32 = 1u32;
pub const TTF_DI_SETITEM: u32 = 32768u32;
#[repr(C)]
pub struct TTGETTITLE {
    pub dwSize: u32,
    pub uTitleBitmap: u32,
    pub cch: u32,
    pub pszTitle: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for TTGETTITLE {}
impl ::core::clone::Clone for TTGETTITLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTGETTITLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTGETTITLE").field("dwSize", &self.dwSize).field("uTitleBitmap", &self.uTitleBitmap).field("cch", &self.cch).field("pszTitle", &self.pszTitle).finish()
    }
}
unsafe impl ::windows_core::Abi for TTGETTITLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TTGETTITLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TTGETTITLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for TTGETTITLE {}
impl ::core::default::Default for TTGETTITLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TTHITTESTINFOA {
    pub hwnd: ::win32_foundation::HWND,
    pub pt: ::win32_foundation::POINT,
    pub ti: TTTOOLINFOA,
}
impl ::core::marker::Copy for TTHITTESTINFOA {}
impl ::core::clone::Clone for TTHITTESTINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTHITTESTINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTHITTESTINFOA").field("hwnd", &self.hwnd).field("pt", &self.pt).field("ti", &self.ti).finish()
    }
}
unsafe impl ::windows_core::Abi for TTHITTESTINFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TTHITTESTINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TTHITTESTINFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for TTHITTESTINFOA {}
impl ::core::default::Default for TTHITTESTINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TTHITTESTINFOW {
    pub hwnd: ::win32_foundation::HWND,
    pub pt: ::win32_foundation::POINT,
    pub ti: TTTOOLINFOW,
}
impl ::core::marker::Copy for TTHITTESTINFOW {}
impl ::core::clone::Clone for TTHITTESTINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTHITTESTINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTHITTESTINFOW").field("hwnd", &self.hwnd).field("pt", &self.pt).field("ti", &self.ti).finish()
    }
}
unsafe impl ::windows_core::Abi for TTHITTESTINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TTHITTESTINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TTHITTESTINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for TTHITTESTINFOW {}
impl ::core::default::Default for TTHITTESTINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TTM_ACTIVATE: u32 = 1025u32;
pub const TTM_ADDTOOL: u32 = 1074u32;
pub const TTM_ADDTOOLA: u32 = 1028u32;
pub const TTM_ADDTOOLW: u32 = 1074u32;
pub const TTM_ADJUSTRECT: u32 = 1055u32;
pub const TTM_DELTOOL: u32 = 1075u32;
pub const TTM_DELTOOLA: u32 = 1029u32;
pub const TTM_DELTOOLW: u32 = 1075u32;
pub const TTM_ENUMTOOLS: u32 = 1082u32;
pub const TTM_ENUMTOOLSA: u32 = 1038u32;
pub const TTM_ENUMTOOLSW: u32 = 1082u32;
pub const TTM_GETBUBBLESIZE: u32 = 1054u32;
pub const TTM_GETCURRENTTOOL: u32 = 1083u32;
pub const TTM_GETCURRENTTOOLA: u32 = 1039u32;
pub const TTM_GETCURRENTTOOLW: u32 = 1083u32;
pub const TTM_GETDELAYTIME: u32 = 1045u32;
pub const TTM_GETMARGIN: u32 = 1051u32;
pub const TTM_GETMAXTIPWIDTH: u32 = 1049u32;
pub const TTM_GETTEXT: u32 = 1080u32;
pub const TTM_GETTEXTA: u32 = 1035u32;
pub const TTM_GETTEXTW: u32 = 1080u32;
pub const TTM_GETTIPBKCOLOR: u32 = 1046u32;
pub const TTM_GETTIPTEXTCOLOR: u32 = 1047u32;
pub const TTM_GETTITLE: u32 = 1059u32;
pub const TTM_GETTOOLCOUNT: u32 = 1037u32;
pub const TTM_GETTOOLINFO: u32 = 1077u32;
pub const TTM_GETTOOLINFOA: u32 = 1032u32;
pub const TTM_GETTOOLINFOW: u32 = 1077u32;
pub const TTM_HITTEST: u32 = 1079u32;
pub const TTM_HITTESTA: u32 = 1034u32;
pub const TTM_HITTESTW: u32 = 1079u32;
pub const TTM_NEWTOOLRECT: u32 = 1076u32;
pub const TTM_NEWTOOLRECTA: u32 = 1030u32;
pub const TTM_NEWTOOLRECTW: u32 = 1076u32;
pub const TTM_POP: u32 = 1052u32;
pub const TTM_POPUP: u32 = 1058u32;
pub const TTM_RELAYEVENT: u32 = 1031u32;
pub const TTM_SETDELAYTIME: u32 = 1027u32;
pub const TTM_SETMARGIN: u32 = 1050u32;
pub const TTM_SETMAXTIPWIDTH: u32 = 1048u32;
pub const TTM_SETTIPBKCOLOR: u32 = 1043u32;
pub const TTM_SETTIPTEXTCOLOR: u32 = 1044u32;
pub const TTM_SETTITLE: u32 = 1057u32;
pub const TTM_SETTITLEA: u32 = 1056u32;
pub const TTM_SETTITLEW: u32 = 1057u32;
pub const TTM_SETTOOLINFO: u32 = 1078u32;
pub const TTM_SETTOOLINFOA: u32 = 1033u32;
pub const TTM_SETTOOLINFOW: u32 = 1078u32;
pub const TTM_SETWINDOWTHEME: u32 = 8203u32;
pub const TTM_TRACKACTIVATE: u32 = 1041u32;
pub const TTM_TRACKPOSITION: u32 = 1042u32;
pub const TTM_UPDATE: u32 = 1053u32;
pub const TTM_UPDATETIPTEXT: u32 = 1081u32;
pub const TTM_UPDATETIPTEXTA: u32 = 1036u32;
pub const TTM_UPDATETIPTEXTW: u32 = 1081u32;
pub const TTM_WINDOWFROMPOINT: u32 = 1040u32;
pub const TTS_ALWAYSTIP: u32 = 1u32;
pub const TTS_BALLOON: u32 = 64u32;
pub const TTS_CLOSE: u32 = 128u32;
pub const TTS_NOANIMATE: u32 = 16u32;
pub const TTS_NOFADE: u32 = 32u32;
pub const TTS_NOPREFIX: u32 = 2u32;
pub const TTS_USEVISUALSTYLE: u32 = 256u32;
#[repr(C)]
pub struct TTTOOLINFOA {
    pub cbSize: u32,
    pub uFlags: TTTOOLINFO_FLAGS,
    pub hwnd: ::win32_foundation::HWND,
    pub uId: usize,
    pub rect: ::win32_foundation::RECT,
    pub hinst: ::win32_foundation::HINSTANCE,
    pub lpszText: ::windows_core::PSTR,
    pub lParam: ::win32_foundation::LPARAM,
    pub lpReserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TTTOOLINFOA {}
impl ::core::clone::Clone for TTTOOLINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTTOOLINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTTOOLINFOA").field("cbSize", &self.cbSize).field("uFlags", &self.uFlags).field("hwnd", &self.hwnd).field("uId", &self.uId).field("rect", &self.rect).field("hinst", &self.hinst).field("lpszText", &self.lpszText).field("lParam", &self.lParam).field("lpReserved", &self.lpReserved).finish()
    }
}
unsafe impl ::windows_core::Abi for TTTOOLINFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TTTOOLINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TTTOOLINFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for TTTOOLINFOA {}
impl ::core::default::Default for TTTOOLINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TTTOOLINFOW {
    pub cbSize: u32,
    pub uFlags: TTTOOLINFO_FLAGS,
    pub hwnd: ::win32_foundation::HWND,
    pub uId: usize,
    pub rect: ::win32_foundation::RECT,
    pub hinst: ::win32_foundation::HINSTANCE,
    pub lpszText: ::windows_core::PWSTR,
    pub lParam: ::win32_foundation::LPARAM,
    pub lpReserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TTTOOLINFOW {}
impl ::core::clone::Clone for TTTOOLINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TTTOOLINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTTOOLINFOW").field("cbSize", &self.cbSize).field("uFlags", &self.uFlags).field("hwnd", &self.hwnd).field("uId", &self.uId).field("rect", &self.rect).field("hinst", &self.hinst).field("lpszText", &self.lpszText).field("lParam", &self.lParam).field("lpReserved", &self.lpReserved).finish()
    }
}
unsafe impl ::windows_core::Abi for TTTOOLINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TTTOOLINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TTTOOLINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for TTTOOLINFOW {}
impl ::core::default::Default for TTTOOLINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TTTOOLINFO_FLAGS(pub u32);
pub const TTF_ABSOLUTE: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(128u32);
pub const TTF_CENTERTIP: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(2u32);
pub const TTF_IDISHWND: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(1u32);
pub const TTF_PARSELINKS: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(4096u32);
pub const TTF_RTLREADING: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(4u32);
pub const TTF_SUBCLASS: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(16u32);
pub const TTF_TRACK: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(32u32);
pub const TTF_TRANSPARENT: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(256u32);
impl ::core::marker::Copy for TTTOOLINFO_FLAGS {}
impl ::core::clone::Clone for TTTOOLINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TTTOOLINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TTTOOLINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TTTOOLINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TTTOOLINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TTTOOLINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TTTOOLINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TTTOOLINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TTTOOLINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TTTOOLINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TVCDRF_NOIMAGES: u32 = 65536u32;
pub const TVC_BYKEYBOARD: u32 = 2u32;
pub const TVC_BYMOUSE: u32 = 1u32;
pub const TVC_UNKNOWN: u32 = 0u32;
pub const TVE_COLLAPSE: u32 = 1u32;
pub const TVE_COLLAPSERESET: u32 = 32768u32;
pub const TVE_EXPAND: u32 = 2u32;
pub const TVE_EXPANDPARTIAL: u32 = 16384u32;
pub const TVE_TOGGLE: u32 = 3u32;
#[repr(C)]
pub struct TVGETITEMPARTRECTINFO {
    pub hti: HTREEITEM,
    pub prc: *mut ::win32_foundation::RECT,
    pub partID: TVITEMPART,
}
impl ::core::marker::Copy for TVGETITEMPARTRECTINFO {}
impl ::core::clone::Clone for TVGETITEMPARTRECTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TVGETITEMPARTRECTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVGETITEMPARTRECTINFO").field("hti", &self.hti).field("prc", &self.prc).field("partID", &self.partID).finish()
    }
}
unsafe impl ::windows_core::Abi for TVGETITEMPARTRECTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TVGETITEMPARTRECTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TVGETITEMPARTRECTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TVGETITEMPARTRECTINFO {}
impl ::core::default::Default for TVGETITEMPARTRECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TVGN_CARET: u32 = 9u32;
pub const TVGN_CHILD: u32 = 4u32;
pub const TVGN_DROPHILITE: u32 = 8u32;
pub const TVGN_FIRSTVISIBLE: u32 = 5u32;
pub const TVGN_LASTVISIBLE: u32 = 10u32;
pub const TVGN_NEXT: u32 = 1u32;
pub const TVGN_NEXTSELECTED: u32 = 11u32;
pub const TVGN_NEXTVISIBLE: u32 = 6u32;
pub const TVGN_PARENT: u32 = 3u32;
pub const TVGN_PREVIOUS: u32 = 2u32;
pub const TVGN_PREVIOUSVISIBLE: u32 = 7u32;
pub const TVGN_ROOT: u32 = 0u32;
#[repr(C)]
pub struct TVHITTESTINFO {
    pub pt: ::win32_foundation::POINT,
    pub flags: TVHITTESTINFO_FLAGS,
    pub hItem: HTREEITEM,
}
impl ::core::marker::Copy for TVHITTESTINFO {}
impl ::core::clone::Clone for TVHITTESTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TVHITTESTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).field("hItem", &self.hItem).finish()
    }
}
unsafe impl ::windows_core::Abi for TVHITTESTINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TVHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TVHITTESTINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for TVHITTESTINFO {}
impl ::core::default::Default for TVHITTESTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TVHITTESTINFO_FLAGS(pub u32);
pub const TVHT_ABOVE: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(256u32);
pub const TVHT_BELOW: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(512u32);
pub const TVHT_NOWHERE: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(1u32);
pub const TVHT_ONITEM: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(70u32);
pub const TVHT_ONITEMBUTTON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(16u32);
pub const TVHT_ONITEMICON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(2u32);
pub const TVHT_ONITEMINDENT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(8u32);
pub const TVHT_ONITEMLABEL: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(4u32);
pub const TVHT_ONITEMRIGHT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(32u32);
pub const TVHT_ONITEMSTATEICON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(64u32);
pub const TVHT_TOLEFT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(2048u32);
pub const TVHT_TORIGHT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(1024u32);
impl ::core::marker::Copy for TVHITTESTINFO_FLAGS {}
impl ::core::clone::Clone for TVHITTESTINFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TVHITTESTINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TVHITTESTINFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TVHITTESTINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TVHITTESTINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TVHITTESTINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TVHITTESTINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TVHITTESTINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
pub struct TVINSERTSTRUCTA {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub Anonymous: TVINSERTSTRUCTA_0,
}
impl ::core::marker::Copy for TVINSERTSTRUCTA {}
impl ::core::clone::Clone for TVINSERTSTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for TVINSERTSTRUCTA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TVINSERTSTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TVINSERTSTRUCTA>()) == 0 }
    }
}
impl ::core::cmp::Eq for TVINSERTSTRUCTA {}
impl ::core::default::Default for TVINSERTSTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union TVINSERTSTRUCTA_0 {
    pub itemex: TVITEMEXA,
    pub item: TVITEMA,
}
impl ::core::marker::Copy for TVINSERTSTRUCTA_0 {}
impl ::core::clone::Clone for TVINSERTSTRUCTA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for TVINSERTSTRUCTA_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TVINSERTSTRUCTA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TVINSERTSTRUCTA_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TVINSERTSTRUCTA_0 {}
impl ::core::default::Default for TVINSERTSTRUCTA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TVINSERTSTRUCTW {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub Anonymous: TVINSERTSTRUCTW_0,
}
impl ::core::marker::Copy for TVINSERTSTRUCTW {}
impl ::core::clone::Clone for TVINSERTSTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for TVINSERTSTRUCTW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TVINSERTSTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TVINSERTSTRUCTW>()) == 0 }
    }
}
impl ::core::cmp::Eq for TVINSERTSTRUCTW {}
impl ::core::default::Default for TVINSERTSTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union TVINSERTSTRUCTW_0 {
    pub itemex: TVITEMEXW,
    pub item: TVITEMW,
}
impl ::core::marker::Copy for TVINSERTSTRUCTW_0 {}
impl ::core::clone::Clone for TVINSERTSTRUCTW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for TVINSERTSTRUCTW_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TVINSERTSTRUCTW_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TVINSERTSTRUCTW_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TVINSERTSTRUCTW_0 {}
impl ::core::default::Default for TVINSERTSTRUCTW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TVIS_BOLD: u32 = 16u32;
pub const TVIS_CUT: u32 = 4u32;
pub const TVIS_DROPHILITED: u32 = 8u32;
pub const TVIS_EXPANDED: u32 = 32u32;
pub const TVIS_EXPANDEDONCE: u32 = 64u32;
pub const TVIS_EXPANDPARTIAL: u32 = 128u32;
pub const TVIS_EX_ALL: u32 = 2u32;
pub const TVIS_EX_DISABLED: u32 = 2u32;
pub const TVIS_EX_FLAT: u32 = 1u32;
pub const TVIS_OVERLAYMASK: u32 = 3840u32;
pub const TVIS_SELECTED: u32 = 2u32;
pub const TVIS_STATEIMAGEMASK: u32 = 61440u32;
pub const TVIS_USERMASK: u32 = 61440u32;
#[repr(C)]
pub struct TVITEMA {
    pub mask: TVITEM_MASK,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: ::windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for TVITEMA {}
impl ::core::clone::Clone for TVITEMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TVITEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVITEMA").field("mask", &self.mask).field("hItem", &self.hItem).field("state", &self.state).field("stateMask", &self.stateMask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("iSelectedImage", &self.iSelectedImage).field("cChildren", &self.cChildren).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for TVITEMA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TVITEMA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TVITEMA>()) == 0 }
    }
}
impl ::core::cmp::Eq for TVITEMA {}
impl ::core::default::Default for TVITEMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TVITEMEXA {
    pub mask: TVITEM_MASK,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: ::windows_core::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: ::win32_foundation::LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: ::win32_foundation::HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
impl ::core::marker::Copy for TVITEMEXA {}
impl ::core::clone::Clone for TVITEMEXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TVITEMEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVITEMEXA")
            .field("mask", &self.mask)
            .field("hItem", &self.hItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("iSelectedImage", &self.iSelectedImage)
            .field("cChildren", &self.cChildren)
            .field("lParam", &self.lParam)
            .field("iIntegral", &self.iIntegral)
            .field("uStateEx", &self.uStateEx)
            .field("hwnd", &self.hwnd)
            .field("iExpandedImage", &self.iExpandedImage)
            .field("iReserved", &self.iReserved)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for TVITEMEXA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TVITEMEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TVITEMEXA>()) == 0 }
    }
}
impl ::core::cmp::Eq for TVITEMEXA {}
impl ::core::default::Default for TVITEMEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TVITEMEXW {
    pub mask: TVITEM_MASK,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: ::win32_foundation::LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: ::win32_foundation::HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
impl ::core::marker::Copy for TVITEMEXW {}
impl ::core::clone::Clone for TVITEMEXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TVITEMEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVITEMEXW")
            .field("mask", &self.mask)
            .field("hItem", &self.hItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("iSelectedImage", &self.iSelectedImage)
            .field("cChildren", &self.cChildren)
            .field("lParam", &self.lParam)
            .field("iIntegral", &self.iIntegral)
            .field("uStateEx", &self.uStateEx)
            .field("hwnd", &self.hwnd)
            .field("iExpandedImage", &self.iExpandedImage)
            .field("iReserved", &self.iReserved)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for TVITEMEXW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TVITEMEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TVITEMEXW>()) == 0 }
    }
}
impl ::core::cmp::Eq for TVITEMEXW {}
impl ::core::default::Default for TVITEMEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TVITEMEXW_CHILDREN(pub i32);
pub const I_ZERO: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(0i32);
pub const I_ONE_OR_MORE: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(1i32);
pub const I_CHILDRENCALLBACK: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(-1i32);
pub const I_CHILDRENAUTO: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(-2i32);
impl ::core::marker::Copy for TVITEMEXW_CHILDREN {}
impl ::core::clone::Clone for TVITEMEXW_CHILDREN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TVITEMEXW_CHILDREN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TVITEMEXW_CHILDREN {
    type Abi = Self;
}
impl ::core::fmt::Debug for TVITEMEXW_CHILDREN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TVITEMEXW_CHILDREN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TVITEMPART(pub i32);
pub const TVGIPR_BUTTON: TVITEMPART = TVITEMPART(1i32);
impl ::core::marker::Copy for TVITEMPART {}
impl ::core::clone::Clone for TVITEMPART {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TVITEMPART {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TVITEMPART {
    type Abi = Self;
}
impl ::core::fmt::Debug for TVITEMPART {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TVITEMPART").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct TVITEMW {
    pub mask: TVITEM_MASK,
    pub hItem: HTREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: ::windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for TVITEMW {}
impl ::core::clone::Clone for TVITEMW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TVITEMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVITEMW").field("mask", &self.mask).field("hItem", &self.hItem).field("state", &self.state).field("stateMask", &self.stateMask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("iSelectedImage", &self.iSelectedImage).field("cChildren", &self.cChildren).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for TVITEMW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TVITEMW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TVITEMW>()) == 0 }
    }
}
impl ::core::cmp::Eq for TVITEMW {}
impl ::core::default::Default for TVITEMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TVITEM_MASK(pub u32);
pub const TVIF_CHILDREN: TVITEM_MASK = TVITEM_MASK(64u32);
pub const TVIF_DI_SETITEM: TVITEM_MASK = TVITEM_MASK(4096u32);
pub const TVIF_HANDLE: TVITEM_MASK = TVITEM_MASK(16u32);
pub const TVIF_IMAGE: TVITEM_MASK = TVITEM_MASK(2u32);
pub const TVIF_PARAM: TVITEM_MASK = TVITEM_MASK(4u32);
pub const TVIF_SELECTEDIMAGE: TVITEM_MASK = TVITEM_MASK(32u32);
pub const TVIF_STATE: TVITEM_MASK = TVITEM_MASK(8u32);
pub const TVIF_TEXT: TVITEM_MASK = TVITEM_MASK(1u32);
pub const TVIF_EXPANDEDIMAGE: TVITEM_MASK = TVITEM_MASK(512u32);
pub const TVIF_INTEGRAL: TVITEM_MASK = TVITEM_MASK(128u32);
pub const TVIF_STATEEX: TVITEM_MASK = TVITEM_MASK(256u32);
impl ::core::marker::Copy for TVITEM_MASK {}
impl ::core::clone::Clone for TVITEM_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TVITEM_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TVITEM_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for TVITEM_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TVITEM_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TVITEM_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TVITEM_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TVITEM_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TVITEM_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TVITEM_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TVI_FIRST: HTREEITEM = HTREEITEM(-65535i32 as _);
pub const TVI_LAST: HTREEITEM = HTREEITEM(-65534i32 as _);
pub const TVI_ROOT: HTREEITEM = HTREEITEM(-65536i32 as _);
pub const TVI_SORT: HTREEITEM = HTREEITEM(-65533i32 as _);
pub const TVM_CREATEDRAGIMAGE: u32 = 4370u32;
pub const TVM_DELETEITEM: u32 = 4353u32;
pub const TVM_EDITLABEL: u32 = 4417u32;
pub const TVM_EDITLABELA: u32 = 4366u32;
pub const TVM_EDITLABELW: u32 = 4417u32;
pub const TVM_ENDEDITLABELNOW: u32 = 4374u32;
pub const TVM_ENSUREVISIBLE: u32 = 4372u32;
pub const TVM_EXPAND: u32 = 4354u32;
pub const TVM_GETBKCOLOR: u32 = 4383u32;
pub const TVM_GETCOUNT: u32 = 4357u32;
pub const TVM_GETEDITCONTROL: u32 = 4367u32;
pub const TVM_GETEXTENDEDSTYLE: u32 = 4397u32;
pub const TVM_GETIMAGELIST: u32 = 4360u32;
pub const TVM_GETINDENT: u32 = 4358u32;
pub const TVM_GETINSERTMARKCOLOR: u32 = 4390u32;
pub const TVM_GETISEARCHSTRING: u32 = 4416u32;
pub const TVM_GETISEARCHSTRINGA: u32 = 4375u32;
pub const TVM_GETISEARCHSTRINGW: u32 = 4416u32;
pub const TVM_GETITEM: u32 = 4414u32;
pub const TVM_GETITEMA: u32 = 4364u32;
pub const TVM_GETITEMHEIGHT: u32 = 4380u32;
pub const TVM_GETITEMPARTRECT: u32 = 4424u32;
pub const TVM_GETITEMRECT: u32 = 4356u32;
pub const TVM_GETITEMSTATE: u32 = 4391u32;
pub const TVM_GETITEMW: u32 = 4414u32;
pub const TVM_GETLINECOLOR: u32 = 4393u32;
pub const TVM_GETNEXTITEM: u32 = 4362u32;
pub const TVM_GETSCROLLTIME: u32 = 4386u32;
pub const TVM_GETSELECTEDCOUNT: u32 = 4422u32;
pub const TVM_GETTEXTCOLOR: u32 = 4384u32;
pub const TVM_GETTOOLTIPS: u32 = 4377u32;
pub const TVM_GETUNICODEFORMAT: u32 = 8198u32;
pub const TVM_GETVISIBLECOUNT: u32 = 4368u32;
pub const TVM_HITTEST: u32 = 4369u32;
pub const TVM_INSERTITEM: u32 = 4402u32;
pub const TVM_INSERTITEMA: u32 = 4352u32;
pub const TVM_INSERTITEMW: u32 = 4402u32;
pub const TVM_MAPACCIDTOHTREEITEM: u32 = 4394u32;
pub const TVM_MAPHTREEITEMTOACCID: u32 = 4395u32;
pub const TVM_SELECTITEM: u32 = 4363u32;
pub const TVM_SETAUTOSCROLLINFO: u32 = 4411u32;
pub const TVM_SETBKCOLOR: u32 = 4381u32;
pub const TVM_SETBORDER: u32 = 4387u32;
pub const TVM_SETEXTENDEDSTYLE: u32 = 4396u32;
pub const TVM_SETHOT: u32 = 4410u32;
pub const TVM_SETIMAGELIST: u32 = 4361u32;
pub const TVM_SETINDENT: u32 = 4359u32;
pub const TVM_SETINSERTMARK: u32 = 4378u32;
pub const TVM_SETINSERTMARKCOLOR: u32 = 4389u32;
pub const TVM_SETITEM: u32 = 4415u32;
pub const TVM_SETITEMA: u32 = 4365u32;
pub const TVM_SETITEMHEIGHT: u32 = 4379u32;
pub const TVM_SETITEMW: u32 = 4415u32;
pub const TVM_SETLINECOLOR: u32 = 4392u32;
pub const TVM_SETSCROLLTIME: u32 = 4385u32;
pub const TVM_SETTEXTCOLOR: u32 = 4382u32;
pub const TVM_SETTOOLTIPS: u32 = 4376u32;
pub const TVM_SETUNICODEFORMAT: u32 = 8197u32;
pub const TVM_SHOWINFOTIP: u32 = 4423u32;
pub const TVM_SORTCHILDREN: u32 = 4371u32;
pub const TVM_SORTCHILDRENCB: u32 = 4373u32;
pub const TVNRET_DEFAULT: u32 = 0u32;
pub const TVNRET_SKIPNEW: u32 = 2u32;
pub const TVNRET_SKIPOLD: u32 = 1u32;
pub const TVSBF_XBORDER: u32 = 1u32;
pub const TVSBF_YBORDER: u32 = 2u32;
pub const TVSIL_NORMAL: u32 = 0u32;
pub const TVSIL_STATE: u32 = 2u32;
pub const TVSI_NOSINGLEEXPAND: u32 = 32768u32;
#[repr(C)]
pub struct TVSORTCB {
    pub hParent: HTREEITEM,
    pub lpfnCompare: PFNTVCOMPARE,
    pub lParam: ::win32_foundation::LPARAM,
}
impl ::core::marker::Copy for TVSORTCB {}
impl ::core::clone::Clone for TVSORTCB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TVSORTCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TVSORTCB").field("hParent", &self.hParent).field("lpfnCompare", &self.lpfnCompare.map(|f| f as usize)).field("lParam", &self.lParam).finish()
    }
}
unsafe impl ::windows_core::Abi for TVSORTCB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TVSORTCB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TVSORTCB>()) == 0 }
    }
}
impl ::core::cmp::Eq for TVSORTCB {}
impl ::core::default::Default for TVSORTCB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TVS_CHECKBOXES: u32 = 256u32;
pub const TVS_DISABLEDRAGDROP: u32 = 16u32;
pub const TVS_EDITLABELS: u32 = 8u32;
pub const TVS_EX_AUTOHSCROLL: u32 = 32u32;
pub const TVS_EX_DIMMEDCHECKBOXES: u32 = 512u32;
pub const TVS_EX_DOUBLEBUFFER: u32 = 4u32;
pub const TVS_EX_DRAWIMAGEASYNC: u32 = 1024u32;
pub const TVS_EX_EXCLUSIONCHECKBOXES: u32 = 256u32;
pub const TVS_EX_FADEINOUTEXPANDOS: u32 = 64u32;
pub const TVS_EX_MULTISELECT: u32 = 2u32;
pub const TVS_EX_NOINDENTSTATE: u32 = 8u32;
pub const TVS_EX_NOSINGLECOLLAPSE: u32 = 1u32;
pub const TVS_EX_PARTIALCHECKBOXES: u32 = 128u32;
pub const TVS_EX_RICHTOOLTIP: u32 = 16u32;
pub const TVS_FULLROWSELECT: u32 = 4096u32;
pub const TVS_HASBUTTONS: u32 = 1u32;
pub const TVS_HASLINES: u32 = 2u32;
pub const TVS_INFOTIP: u32 = 2048u32;
pub const TVS_LINESATROOT: u32 = 4u32;
pub const TVS_NOHSCROLL: u32 = 32768u32;
pub const TVS_NONEVENHEIGHT: u32 = 16384u32;
pub const TVS_NOSCROLL: u32 = 8192u32;
pub const TVS_NOTOOLTIPS: u32 = 128u32;
pub const TVS_RTLREADING: u32 = 64u32;
pub const TVS_SHOWSELALWAYS: u32 = 32u32;
pub const TVS_SINGLEEXPAND: u32 = 1024u32;
pub const TVS_TRACKSELECT: u32 = 512u32;
pub const TV_FIRST: u32 = 4352u32;
#[inline]
pub unsafe fn TaskDialog<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HINSTANCE>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(hwndowner: Param0, hinstance: Param1, pszwindowtitle: Param2, pszmaininstruction: Param3, pszcontent: Param4, dwcommonbuttons: TASKDIALOG_COMMON_BUTTON_FLAGS, pszicon: Param6) -> ::windows_core::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TaskDialog(hwndowner: ::win32_foundation::HWND, hinstance: ::win32_foundation::HINSTANCE, pszwindowtitle: ::windows_core::PCWSTR, pszmaininstruction: ::windows_core::PCWSTR, pszcontent: ::windows_core::PCWSTR, dwcommonbuttons: TASKDIALOG_COMMON_BUTTON_FLAGS, pszicon: ::windows_core::PCWSTR, pnbutton: *mut i32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        TaskDialog(hwndowner.into_param().abi(), hinstance.into_param().abi(), pszwindowtitle.into_param().abi(), pszmaininstruction.into_param().abi(), pszcontent.into_param().abi(), ::core::mem::transmute(dwcommonbuttons), pszicon.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn TaskDialogIndirect(ptaskconfig: *const TASKDIALOGCONFIG, pnbutton: *mut i32, pnradiobutton: *mut i32, pfverificationflagchecked: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TaskDialogIndirect(ptaskconfig: *const TASKDIALOGCONFIG, pnbutton: *mut i32, pnradiobutton: *mut i32, pfverificationflagchecked: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        TaskDialogIndirect(::core::mem::transmute(ptaskconfig), ::core::mem::transmute(pnbutton), ::core::mem::transmute(pnradiobutton), ::core::mem::transmute(pfverificationflagchecked)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct UDACCEL {
    pub nSec: u32,
    pub nInc: u32,
}
impl ::core::marker::Copy for UDACCEL {}
impl ::core::clone::Clone for UDACCEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UDACCEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UDACCEL").field("nSec", &self.nSec).field("nInc", &self.nInc).finish()
    }
}
unsafe impl ::windows_core::Abi for UDACCEL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UDACCEL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UDACCEL>()) == 0 }
    }
}
impl ::core::cmp::Eq for UDACCEL {}
impl ::core::default::Default for UDACCEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const UDM_GETACCEL: u32 = 1132u32;
pub const UDM_GETBASE: u32 = 1134u32;
pub const UDM_GETBUDDY: u32 = 1130u32;
pub const UDM_GETPOS: u32 = 1128u32;
pub const UDM_GETPOS32: u32 = 1138u32;
pub const UDM_GETRANGE: u32 = 1126u32;
pub const UDM_GETRANGE32: u32 = 1136u32;
pub const UDM_GETUNICODEFORMAT: u32 = 8198u32;
pub const UDM_SETACCEL: u32 = 1131u32;
pub const UDM_SETBASE: u32 = 1133u32;
pub const UDM_SETBUDDY: u32 = 1129u32;
pub const UDM_SETPOS: u32 = 1127u32;
pub const UDM_SETPOS32: u32 = 1137u32;
pub const UDM_SETRANGE: u32 = 1125u32;
pub const UDM_SETRANGE32: u32 = 1135u32;
pub const UDM_SETUNICODEFORMAT: u32 = 8197u32;
pub const UDS_ALIGNLEFT: u32 = 8u32;
pub const UDS_ALIGNRIGHT: u32 = 4u32;
pub const UDS_ARROWKEYS: u32 = 32u32;
pub const UDS_AUTOBUDDY: u32 = 16u32;
pub const UDS_HORZ: u32 = 64u32;
pub const UDS_HOTTRACK: u32 = 256u32;
pub const UDS_NOTHOUSANDS: u32 = 128u32;
pub const UDS_SETBUDDYINT: u32 = 2u32;
pub const UDS_WRAP: u32 = 1u32;
pub const UD_MAXVAL: u32 = 32767u32;
pub const UPDOWN_CLASS: &str = "msctls_updown32";
pub const UPDOWN_CLASSA: &str = "msctls_updown32";
pub const UPDOWN_CLASSW: &str = "msctls_updown32";
#[repr(C)]
pub struct USAGE_PROPERTIES {
    pub level: u16,
    pub page: u16,
    pub usage: u16,
    pub logicalMinimum: i32,
    pub logicalMaximum: i32,
    pub unit: u16,
    pub exponent: u16,
    pub count: u8,
    pub physicalMinimum: i32,
    pub physicalMaximum: i32,
}
impl ::core::marker::Copy for USAGE_PROPERTIES {}
impl ::core::clone::Clone for USAGE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USAGE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USAGE_PROPERTIES").field("level", &self.level).field("page", &self.page).field("usage", &self.usage).field("logicalMinimum", &self.logicalMinimum).field("logicalMaximum", &self.logicalMaximum).field("unit", &self.unit).field("exponent", &self.exponent).field("count", &self.count).field("physicalMinimum", &self.physicalMinimum).field("physicalMaximum", &self.physicalMaximum).finish()
    }
}
unsafe impl ::windows_core::Abi for USAGE_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USAGE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USAGE_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for USAGE_PROPERTIES {}
impl ::core::default::Default for USAGE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn UninitializeFlatSB<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(param0: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UninitializeFlatSB(param0: ::win32_foundation::HWND) -> ::windows_core::HRESULT;
        }
        UninitializeFlatSB(param0.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UpdatePanningFeedback<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hwnd: Param0, ltotaloverpanoffsetx: i32, ltotaloverpanoffsety: i32, fininertia: Param3) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdatePanningFeedback(hwnd: ::win32_foundation::HWND, ltotaloverpanoffsetx: i32, ltotaloverpanoffsety: i32, fininertia: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(UpdatePanningFeedback(hwnd.into_param().abi(), ::core::mem::transmute(ltotaloverpanoffsetx), ::core::mem::transmute(ltotaloverpanoffsety), fininertia.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VALIGN(pub i32);
pub const VA_TOP: VALIGN = VALIGN(0i32);
pub const VA_CENTER: VALIGN = VALIGN(1i32);
pub const VA_BOTTOM: VALIGN = VALIGN(2i32);
impl ::core::marker::Copy for VALIGN {}
impl ::core::clone::Clone for VALIGN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VALIGN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for VALIGN {
    type Abi = Self;
}
impl ::core::fmt::Debug for VALIGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VALIGN").field(&self.0).finish()
    }
}
pub const VIEW_DETAILS: u32 = 3u32;
pub const VIEW_LARGEICONS: u32 = 0u32;
pub const VIEW_LIST: u32 = 2u32;
pub const VIEW_NETCONNECT: u32 = 9u32;
pub const VIEW_NETDISCONNECT: u32 = 10u32;
pub const VIEW_NEWFOLDER: u32 = 11u32;
pub const VIEW_PARENTFOLDER: u32 = 8u32;
pub const VIEW_SMALLICONS: u32 = 1u32;
pub const VIEW_SORTDATE: u32 = 6u32;
pub const VIEW_SORTNAME: u32 = 4u32;
pub const VIEW_SORTSIZE: u32 = 5u32;
pub const VIEW_SORTTYPE: u32 = 7u32;
pub const VIEW_VIEWMENU: u32 = 12u32;
pub const VSCLASS_CLOCK: &str = "CLOCK";
pub const VSCLASS_EMPTYMARKUP: &str = "EMPTYMARKUP";
pub const VSCLASS_LINK: &str = "LINK";
pub const VSCLASS_MENUBAND: &str = "MENUBAND";
pub const VSCLASS_MONTHCAL: &str = "MONTHCAL";
pub const VSCLASS_PAGE: &str = "PAGE";
pub const VSCLASS_STARTPANEL: &str = "STARTPANEL";
pub const VSCLASS_STATIC: &str = "STATIC";
pub const VSCLASS_TASKBAND: &str = "TASKBAND";
pub const VSCLASS_TASKBAR: &str = "TASKBAR";
pub const VSCLASS_TRAYNOTIFY: &str = "TRAYNOTIFY";
pub const WC_BUTTON: &str = "Button";
pub const WC_BUTTONA: &str = "Button";
pub const WC_BUTTONW: &str = "Button";
pub const WC_COMBOBOX: &str = "ComboBox";
pub const WC_COMBOBOXA: &str = "ComboBox";
pub const WC_COMBOBOXEX: &str = "ComboBoxEx32";
pub const WC_COMBOBOXEXA: &str = "ComboBoxEx32";
pub const WC_COMBOBOXEXW: &str = "ComboBoxEx32";
pub const WC_COMBOBOXW: &str = "ComboBox";
pub const WC_EDIT: &str = "Edit";
pub const WC_EDITA: &str = "Edit";
pub const WC_EDITW: &str = "Edit";
pub const WC_HEADER: &str = "SysHeader32";
pub const WC_HEADERA: &str = "SysHeader32";
pub const WC_HEADERW: &str = "SysHeader32";
pub const WC_IPADDRESS: &str = "SysIPAddress32";
pub const WC_IPADDRESSA: &str = "SysIPAddress32";
pub const WC_IPADDRESSW: &str = "SysIPAddress32";
pub const WC_LINK: &str = "SysLink";
pub const WC_LISTBOX: &str = "ListBox";
pub const WC_LISTBOXA: &str = "ListBox";
pub const WC_LISTBOXW: &str = "ListBox";
pub const WC_LISTVIEW: &str = "SysListView32";
pub const WC_LISTVIEWA: &str = "SysListView32";
pub const WC_LISTVIEWW: &str = "SysListView32";
pub const WC_NATIVEFONTCTL: &str = "NativeFontCtl";
pub const WC_NATIVEFONTCTLA: &str = "NativeFontCtl";
pub const WC_NATIVEFONTCTLW: &str = "NativeFontCtl";
pub const WC_PAGESCROLLER: &str = "SysPager";
pub const WC_PAGESCROLLERA: &str = "SysPager";
pub const WC_PAGESCROLLERW: &str = "SysPager";
pub const WC_SCROLLBAR: &str = "ScrollBar";
pub const WC_SCROLLBARA: &str = "ScrollBar";
pub const WC_SCROLLBARW: &str = "ScrollBar";
pub const WC_STATIC: &str = "Static";
pub const WC_STATICA: &str = "Static";
pub const WC_STATICW: &str = "Static";
pub const WC_TABCONTROL: &str = "SysTabControl32";
pub const WC_TABCONTROLA: &str = "SysTabControl32";
pub const WC_TABCONTROLW: &str = "SysTabControl32";
pub const WC_TREEVIEW: &str = "SysTreeView32";
pub const WC_TREEVIEWA: &str = "SysTreeView32";
pub const WC_TREEVIEWW: &str = "SysTreeView32";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WINDOWTHEMEATTRIBUTETYPE(pub i32);
pub const WTA_NONCLIENT: WINDOWTHEMEATTRIBUTETYPE = WINDOWTHEMEATTRIBUTETYPE(1i32);
impl ::core::marker::Copy for WINDOWTHEMEATTRIBUTETYPE {}
impl ::core::clone::Clone for WINDOWTHEMEATTRIBUTETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WINDOWTHEMEATTRIBUTETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WINDOWTHEMEATTRIBUTETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WINDOWTHEMEATTRIBUTETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOWTHEMEATTRIBUTETYPE").field(&self.0).finish()
    }
}
pub const WIZ_BODYCX: u32 = 184u32;
pub const WIZ_BODYX: u32 = 92u32;
pub const WIZ_CXBMP: u32 = 80u32;
pub const WIZ_CXDLG: u32 = 276u32;
pub const WIZ_CYDLG: u32 = 140u32;
pub const WM_CTLCOLOR: u32 = 25u32;
pub const WM_MOUSEHOVER: u32 = 673u32;
pub const WM_MOUSELEAVE: u32 = 675u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WORD_BREAK_ACTION(pub u32);
pub const WB_CLASSIFY: WORD_BREAK_ACTION = WORD_BREAK_ACTION(3u32);
pub const WB_ISDELIMITER: WORD_BREAK_ACTION = WORD_BREAK_ACTION(2u32);
pub const WB_LEFT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(0u32);
pub const WB_LEFTBREAK: WORD_BREAK_ACTION = WORD_BREAK_ACTION(6u32);
pub const WB_MOVEWORDLEFT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(4u32);
pub const WB_MOVEWORDRIGHT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(5u32);
pub const WB_RIGHT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(1u32);
pub const WB_RIGHTBREAK: WORD_BREAK_ACTION = WORD_BREAK_ACTION(7u32);
impl ::core::marker::Copy for WORD_BREAK_ACTION {}
impl ::core::clone::Clone for WORD_BREAK_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WORD_BREAK_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WORD_BREAK_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for WORD_BREAK_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WORD_BREAK_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WSB_PROP(pub i32);
pub const WSB_PROP_CXHSCROLL: WSB_PROP = WSB_PROP(2i32);
pub const WSB_PROP_CXHTHUMB: WSB_PROP = WSB_PROP(16i32);
pub const WSB_PROP_CXVSCROLL: WSB_PROP = WSB_PROP(8i32);
pub const WSB_PROP_CYHSCROLL: WSB_PROP = WSB_PROP(4i32);
pub const WSB_PROP_CYVSCROLL: WSB_PROP = WSB_PROP(1i32);
pub const WSB_PROP_CYVTHUMB: WSB_PROP = WSB_PROP(32i32);
pub const WSB_PROP_HBKGCOLOR: WSB_PROP = WSB_PROP(128i32);
pub const WSB_PROP_HSTYLE: WSB_PROP = WSB_PROP(512i32);
pub const WSB_PROP_PALETTE: WSB_PROP = WSB_PROP(2048i32);
pub const WSB_PROP_VBKGCOLOR: WSB_PROP = WSB_PROP(64i32);
pub const WSB_PROP_VSTYLE: WSB_PROP = WSB_PROP(256i32);
pub const WSB_PROP_WINSTYLE: WSB_PROP = WSB_PROP(1024i32);
impl ::core::marker::Copy for WSB_PROP {}
impl ::core::clone::Clone for WSB_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSB_PROP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WSB_PROP {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSB_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSB_PROP").field(&self.0).finish()
    }
}
pub const WSB_PROP_MASK: i32 = 4095i32;
#[repr(C)]
pub struct WTA_OPTIONS {
    pub dwFlags: u32,
    pub dwMask: u32,
}
impl ::core::marker::Copy for WTA_OPTIONS {}
impl ::core::clone::Clone for WTA_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WTA_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTA_OPTIONS").field("dwFlags", &self.dwFlags).field("dwMask", &self.dwMask).finish()
    }
}
unsafe impl ::windows_core::Abi for WTA_OPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WTA_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WTA_OPTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WTA_OPTIONS {}
impl ::core::default::Default for WTA_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WTNCA_NODRAWCAPTION: u32 = 1u32;
pub const WTNCA_NODRAWICON: u32 = 2u32;
pub const WTNCA_NOMIRRORHELP: u32 = 8u32;
pub const WTNCA_NOSYSMENU: u32 = 4u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _LI_METRIC(pub i32);
pub const LIM_SMALL: _LI_METRIC = _LI_METRIC(0i32);
pub const LIM_LARGE: _LI_METRIC = _LI_METRIC(1i32);
impl ::core::marker::Copy for _LI_METRIC {}
impl ::core::clone::Clone for _LI_METRIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _LI_METRIC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for _LI_METRIC {
    type Abi = Self;
}
impl ::core::fmt::Debug for _LI_METRIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_LI_METRIC").field(&self.0).finish()
    }
}
pub const chx1: u32 = 1040u32;
pub const chx10: u32 = 1049u32;
pub const chx11: u32 = 1050u32;
pub const chx12: u32 = 1051u32;
pub const chx13: u32 = 1052u32;
pub const chx14: u32 = 1053u32;
pub const chx15: u32 = 1054u32;
pub const chx16: u32 = 1055u32;
pub const chx2: u32 = 1041u32;
pub const chx3: u32 = 1042u32;
pub const chx4: u32 = 1043u32;
pub const chx5: u32 = 1044u32;
pub const chx6: u32 = 1045u32;
pub const chx7: u32 = 1046u32;
pub const chx8: u32 = 1047u32;
pub const chx9: u32 = 1048u32;
pub const cmb1: u32 = 1136u32;
pub const cmb10: u32 = 1145u32;
pub const cmb11: u32 = 1146u32;
pub const cmb12: u32 = 1147u32;
pub const cmb13: u32 = 1148u32;
pub const cmb14: u32 = 1149u32;
pub const cmb15: u32 = 1150u32;
pub const cmb16: u32 = 1151u32;
pub const cmb2: u32 = 1137u32;
pub const cmb3: u32 = 1138u32;
pub const cmb4: u32 = 1139u32;
pub const cmb5: u32 = 1140u32;
pub const cmb6: u32 = 1141u32;
pub const cmb7: u32 = 1142u32;
pub const cmb8: u32 = 1143u32;
pub const cmb9: u32 = 1144u32;
pub const ctl1: u32 = 1184u32;
pub const ctlFirst: u32 = 1024u32;
pub const ctlLast: u32 = 1279u32;
pub const edt1: u32 = 1152u32;
pub const edt10: u32 = 1161u32;
pub const edt11: u32 = 1162u32;
pub const edt12: u32 = 1163u32;
pub const edt13: u32 = 1164u32;
pub const edt14: u32 = 1165u32;
pub const edt15: u32 = 1166u32;
pub const edt16: u32 = 1167u32;
pub const edt2: u32 = 1153u32;
pub const edt3: u32 = 1154u32;
pub const edt4: u32 = 1155u32;
pub const edt5: u32 = 1156u32;
pub const edt6: u32 = 1157u32;
pub const edt7: u32 = 1158u32;
pub const edt8: u32 = 1159u32;
pub const edt9: u32 = 1160u32;
pub const frm1: u32 = 1076u32;
pub const frm2: u32 = 1077u32;
pub const frm3: u32 = 1078u32;
pub const frm4: u32 = 1079u32;
pub const grp1: u32 = 1072u32;
pub const grp2: u32 = 1073u32;
pub const grp3: u32 = 1074u32;
pub const grp4: u32 = 1075u32;
pub const ico1: u32 = 1084u32;
pub const ico2: u32 = 1085u32;
pub const ico3: u32 = 1086u32;
pub const ico4: u32 = 1087u32;
pub const lst1: u32 = 1120u32;
pub const lst10: u32 = 1129u32;
pub const lst11: u32 = 1130u32;
pub const lst12: u32 = 1131u32;
pub const lst13: u32 = 1132u32;
pub const lst14: u32 = 1133u32;
pub const lst15: u32 = 1134u32;
pub const lst16: u32 = 1135u32;
pub const lst2: u32 = 1121u32;
pub const lst3: u32 = 1122u32;
pub const lst4: u32 = 1123u32;
pub const lst5: u32 = 1124u32;
pub const lst6: u32 = 1125u32;
pub const lst7: u32 = 1126u32;
pub const lst8: u32 = 1127u32;
pub const lst9: u32 = 1128u32;
pub const psh1: u32 = 1024u32;
pub const psh10: u32 = 1033u32;
pub const psh11: u32 = 1034u32;
pub const psh12: u32 = 1035u32;
pub const psh13: u32 = 1036u32;
pub const psh14: u32 = 1037u32;
pub const psh15: u32 = 1038u32;
pub const psh16: u32 = 1039u32;
pub const psh2: u32 = 1025u32;
pub const psh3: u32 = 1026u32;
pub const psh4: u32 = 1027u32;
pub const psh5: u32 = 1028u32;
pub const psh6: u32 = 1029u32;
pub const psh7: u32 = 1030u32;
pub const psh8: u32 = 1031u32;
pub const psh9: u32 = 1032u32;
pub const pshHelp: u32 = 1038u32;
pub const rad1: u32 = 1056u32;
pub const rad10: u32 = 1065u32;
pub const rad11: u32 = 1066u32;
pub const rad12: u32 = 1067u32;
pub const rad13: u32 = 1068u32;
pub const rad14: u32 = 1069u32;
pub const rad15: u32 = 1070u32;
pub const rad16: u32 = 1071u32;
pub const rad2: u32 = 1057u32;
pub const rad3: u32 = 1058u32;
pub const rad4: u32 = 1059u32;
pub const rad5: u32 = 1060u32;
pub const rad6: u32 = 1061u32;
pub const rad7: u32 = 1062u32;
pub const rad8: u32 = 1063u32;
pub const rad9: u32 = 1064u32;
pub const rct1: u32 = 1080u32;
pub const rct2: u32 = 1081u32;
pub const rct3: u32 = 1082u32;
pub const rct4: u32 = 1083u32;
pub const scr1: u32 = 1168u32;
pub const scr2: u32 = 1169u32;
pub const scr3: u32 = 1170u32;
pub const scr4: u32 = 1171u32;
pub const scr5: u32 = 1172u32;
pub const scr6: u32 = 1173u32;
pub const scr7: u32 = 1174u32;
pub const scr8: u32 = 1175u32;
pub const stc1: u32 = 1088u32;
pub const stc10: u32 = 1097u32;
pub const stc11: u32 = 1098u32;
pub const stc12: u32 = 1099u32;
pub const stc13: u32 = 1100u32;
pub const stc14: u32 = 1101u32;
pub const stc15: u32 = 1102u32;
pub const stc16: u32 = 1103u32;
pub const stc17: u32 = 1104u32;
pub const stc18: u32 = 1105u32;
pub const stc19: u32 = 1106u32;
pub const stc2: u32 = 1089u32;
pub const stc20: u32 = 1107u32;
pub const stc21: u32 = 1108u32;
pub const stc22: u32 = 1109u32;
pub const stc23: u32 = 1110u32;
pub const stc24: u32 = 1111u32;
pub const stc25: u32 = 1112u32;
pub const stc26: u32 = 1113u32;
pub const stc27: u32 = 1114u32;
pub const stc28: u32 = 1115u32;
pub const stc29: u32 = 1116u32;
pub const stc3: u32 = 1090u32;
pub const stc30: u32 = 1117u32;
pub const stc31: u32 = 1118u32;
pub const stc32: u32 = 1119u32;
pub const stc4: u32 = 1091u32;
pub const stc5: u32 = 1092u32;
pub const stc6: u32 = 1093u32;
pub const stc7: u32 = 1094u32;
pub const stc8: u32 = 1095u32;
pub const stc9: u32 = 1096u32;
