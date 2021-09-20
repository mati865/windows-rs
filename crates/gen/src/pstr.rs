use super::*;

pub fn gen_pstr() -> TokenStream {
    quote! {
        #[derive(::std::clone::Clone, ::std::marker::Copy, ::std::fmt::Debug, ::std::cmp::PartialEq, ::std::cmp::Eq)]
        #[repr(transparent)]
        pub struct PSTR(pub *mut u8);
        impl PSTR {
            pub fn is_null(&self) -> bool {
                self.0.is_null()
            }
        }
        impl ::std::default::Default for PSTR {
            fn default() -> Self {
                Self(::std::ptr::null_mut())
            }
        }
        unsafe impl ::windows::Abi for PSTR {
            type Abi = Self;
            type DefaultType = Self;

            unsafe fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                if let ::windows::Param::Boxed(value) = param {
                    if !value.is_null() {
                        unsafe { ::std::boxed::Box::from_raw(value.0); }
                    }
                }
            }
        }
        impl<'a> ::windows::IntoParam<'a, PSTR> for &str {
            fn into_param(self) -> ::windows::Param<'a, PSTR> {
                ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(self.bytes().chain(::std::iter::once(0)).collect::<std::vec::Vec<u8>>().into_boxed_slice()) as _))
            }
        }
        impl<'a> ::windows::IntoParam<'a, PSTR> for String {
            fn into_param(self) -> ::windows::Param<'a, PSTR> {
                // TODO: call variant above
                ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(self.bytes().chain(::std::iter::once(0)).collect::<std::vec::Vec<u8>>().into_boxed_slice()) as _))
            }
        }
    }
}