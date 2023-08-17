// Bindings generated by `windows-bindgen` 0.51.1

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod NestedModule {
    #[repr(C)]
    pub struct NestedType {
        pub field: f32,
    }
    impl ::core::marker::Copy for NestedType {}
    impl ::core::clone::Clone for NestedType {
        fn clone(&self) -> Self {
            *self
        }
    }
    impl ::core::fmt::Debug for NestedType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("NestedType")
                .field("field", &self.field)
                .finish()
        }
    }
    impl ::windows_core::TypeKind for NestedType {
        type TypeKind = ::windows_core::CopyType;
    }
    impl ::windows_core::RuntimeType for NestedType {
        const SIGNATURE: ::windows_core::imp::ConstBuffer =
            ::windows_core::imp::ConstBuffer::from_slice(
                b"struct(Test.NestedModule.NestedType;f4)",
            );
    }
    impl ::core::cmp::PartialEq for NestedType {
        fn eq(&self, other: &Self) -> bool {
            self.field == other.field
        }
    }
    impl ::core::cmp::Eq for NestedType {}
    impl ::core::default::Default for NestedType {
        fn default() -> Self {
            unsafe { ::core::mem::zeroed() }
        }
    }
}
#[repr(C)]
pub struct TestType {
    pub field: i32,
}
impl ::core::marker::Copy for TestType {}
impl ::core::clone::Clone for TestType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TestType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TestType")
            .field("field", &self.field)
            .finish()
    }
}
impl ::windows_core::TypeKind for TestType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for TestType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer =
        ::windows_core::imp::ConstBuffer::from_slice(b"struct(Test.TestType;i4)");
}
impl ::core::cmp::PartialEq for TestType {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field
    }
}
impl ::core::cmp::Eq for TestType {}
impl ::core::default::Default for TestType {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
