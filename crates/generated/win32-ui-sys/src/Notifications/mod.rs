pub type INotificationActivationCallback = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct NOTIFICATION_USER_INPUT_DATA {
    pub Key: ::windows_sys_core::PCWSTR,
    pub Value: ::windows_sys_core::PCWSTR,
}
impl ::core::marker::Copy for NOTIFICATION_USER_INPUT_DATA {}
impl ::core::clone::Clone for NOTIFICATION_USER_INPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}