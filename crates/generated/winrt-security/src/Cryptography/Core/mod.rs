pub struct AsymmetricAlgorithmNames;
impl AsymmetricAlgorithmNames {
    pub fn RsaPkcs1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaPkcs1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RsaOaepSha1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaOaepSha1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RsaOaepSha256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaOaepSha256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RsaOaepSha384() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaOaepSha384)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RsaOaepSha512() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaOaepSha512)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn EcdsaP256Sha256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EcdsaP256Sha256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn EcdsaP384Sha384() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EcdsaP384Sha384)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn EcdsaP521Sha512() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EcdsaP521Sha512)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DsaSha1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DsaSha1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DsaSha256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DsaSha256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPkcs1Sha1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaSignPkcs1Sha1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPkcs1Sha256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaSignPkcs1Sha256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPkcs1Sha384() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaSignPkcs1Sha384)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPkcs1Sha512() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaSignPkcs1Sha512)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPssSha1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaSignPssSha1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPssSha256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaSignPssSha256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPssSha384() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaSignPssSha384)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn RsaSignPssSha512() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RsaSignPssSha512)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn EcdsaSha256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EcdsaSha256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn EcdsaSha384() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EcdsaSha384)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn EcdsaSha512() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAsymmetricAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EcdsaSha512)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IAsymmetricAlgorithmNamesStatics<R, F: FnOnce(&IAsymmetricAlgorithmNamesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AsymmetricAlgorithmNames, IAsymmetricAlgorithmNamesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IAsymmetricAlgorithmNamesStatics2<R, F: FnOnce(&IAsymmetricAlgorithmNamesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AsymmetricAlgorithmNames, IAsymmetricAlgorithmNamesStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for AsymmetricAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.AsymmetricAlgorithmNames";
}
#[repr(transparent)]
pub struct AsymmetricKeyAlgorithmProvider(::windows_core::IUnknown);
impl AsymmetricKeyAlgorithmProvider {
    pub fn AlgorithmName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AlgorithmName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn CreateKeyPair(&self, keysize: u32) -> ::windows_core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateKeyPair)(::windows_core::Interface::as_raw(this), keysize, result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportDefaultPrivateKeyBlob<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keyblob: Param0) -> ::windows_core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportDefaultPrivateKeyBlob)(::windows_core::Interface::as_raw(this), keyblob.into_param().abi(), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportKeyPairWithBlobType<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keyblob: Param0, blobtype: CryptographicPrivateKeyBlobType) -> ::windows_core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportKeyPairWithBlobType)(::windows_core::Interface::as_raw(this), keyblob.into_param().abi(), blobtype, result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportDefaultPublicKeyBlob<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keyblob: Param0) -> ::windows_core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportDefaultPublicKeyBlob)(::windows_core::Interface::as_raw(this), keyblob.into_param().abi(), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportPublicKeyWithBlobType<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keyblob: Param0, blobtype: CryptographicPublicKeyBlobType) -> ::windows_core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ImportPublicKeyWithBlobType)(::windows_core::Interface::as_raw(this), keyblob.into_param().abi(), blobtype, result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    pub fn CreateKeyPairWithCurveName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, curvename: Param0) -> ::windows_core::Result<CryptographicKey> {
        let this = &::windows_core::Interface::cast::<IAsymmetricKeyAlgorithmProvider2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateKeyPairWithCurveName)(::windows_core::Interface::as_raw(this), curvename.into_param().abi(), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    pub fn CreateKeyPairWithCurveParameters(&self, parameters: &[u8]) -> ::windows_core::Result<CryptographicKey> {
        let this = &::windows_core::Interface::cast::<IAsymmetricKeyAlgorithmProvider2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateKeyPairWithCurveParameters)(::windows_core::Interface::as_raw(this), parameters.len() as u32, ::core::mem::transmute(parameters.as_ptr()), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    pub fn OpenAlgorithm<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(algorithm: Param0) -> ::windows_core::Result<AsymmetricKeyAlgorithmProvider> {
        Self::IAsymmetricKeyAlgorithmProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenAlgorithm)(::windows_core::Interface::as_raw(this), algorithm.into_param().abi(), result__.as_mut_ptr()).from_abi::<AsymmetricKeyAlgorithmProvider>(result__)
        })
    }
    pub fn IAsymmetricKeyAlgorithmProviderStatics<R, F: FnOnce(&IAsymmetricKeyAlgorithmProviderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AsymmetricKeyAlgorithmProvider, IAsymmetricKeyAlgorithmProviderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AsymmetricKeyAlgorithmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsymmetricKeyAlgorithmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsymmetricKeyAlgorithmProvider {}
impl ::core::fmt::Debug for AsymmetricKeyAlgorithmProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsymmetricKeyAlgorithmProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AsymmetricKeyAlgorithmProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider;{e8d2ff37-6259-4e88-b7e0-94191fde699e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AsymmetricKeyAlgorithmProvider {
    type Vtable = IAsymmetricKeyAlgorithmProvider_Vtbl;
    const IID: ::windows_core::GUID = <IAsymmetricKeyAlgorithmProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AsymmetricKeyAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.AsymmetricKeyAlgorithmProvider";
}
impl ::core::convert::From<AsymmetricKeyAlgorithmProvider> for ::windows_core::IUnknown {
    fn from(value: AsymmetricKeyAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsymmetricKeyAlgorithmProvider> for ::windows_core::IUnknown {
    fn from(value: &AsymmetricKeyAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AsymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AsymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AsymmetricKeyAlgorithmProvider> for ::windows_core::IInspectable {
    fn from(value: AsymmetricKeyAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsymmetricKeyAlgorithmProvider> for ::windows_core::IInspectable {
    fn from(value: &AsymmetricKeyAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AsymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AsymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AsymmetricKeyAlgorithmProvider {}
unsafe impl ::core::marker::Sync for AsymmetricKeyAlgorithmProvider {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Capi1KdfTargetAlgorithm(pub i32);
impl Capi1KdfTargetAlgorithm {
    pub const NotAes: Self = Self(0i32);
    pub const Aes: Self = Self(1i32);
}
impl ::core::marker::Copy for Capi1KdfTargetAlgorithm {}
impl ::core::clone::Clone for Capi1KdfTargetAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Capi1KdfTargetAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for Capi1KdfTargetAlgorithm {
    type Abi = Self;
}
impl ::core::fmt::Debug for Capi1KdfTargetAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Capi1KdfTargetAlgorithm").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Capi1KdfTargetAlgorithm {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Core.Capi1KdfTargetAlgorithm;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct CryptographicEngine;
impl CryptographicEngine {
    #[cfg(feature = "Storage_Streams")]
    pub fn Encrypt<'a, Param0: ::windows_core::IntoParam<'a, CryptographicKey>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, iv: Param2) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Encrypt)(::windows_core::Interface::as_raw(this), key.into_param().abi(), data.into_param().abi(), iv.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Decrypt<'a, Param0: ::windows_core::IntoParam<'a, CryptographicKey>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, iv: Param2) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Decrypt)(::windows_core::Interface::as_raw(this), key.into_param().abi(), data.into_param().abi(), iv.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn EncryptAndAuthenticate<'a, Param0: ::windows_core::IntoParam<'a, CryptographicKey>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param3: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, nonce: Param2, authenticateddata: Param3) -> ::windows_core::Result<EncryptedAndAuthenticatedData> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncryptAndAuthenticate)(::windows_core::Interface::as_raw(this), key.into_param().abi(), data.into_param().abi(), nonce.into_param().abi(), authenticateddata.into_param().abi(), result__.as_mut_ptr()).from_abi::<EncryptedAndAuthenticatedData>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn DecryptAndAuthenticate<'a, Param0: ::windows_core::IntoParam<'a, CryptographicKey>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param3: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param4: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, nonce: Param2, authenticationtag: Param3, authenticateddata: Param4) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DecryptAndAuthenticate)(::windows_core::Interface::as_raw(this), key.into_param().abi(), data.into_param().abi(), nonce.into_param().abi(), authenticationtag.into_param().abi(), authenticateddata.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Sign<'a, Param0: ::windows_core::IntoParam<'a, CryptographicKey>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Sign)(::windows_core::Interface::as_raw(this), key.into_param().abi(), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn VerifySignature<'a, Param0: ::windows_core::IntoParam<'a, CryptographicKey>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, signature: Param2) -> ::windows_core::Result<bool> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).VerifySignature)(::windows_core::Interface::as_raw(this), key.into_param().abi(), data.into_param().abi(), signature.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn DeriveKeyMaterial<'a, Param0: ::windows_core::IntoParam<'a, CryptographicKey>, Param1: ::windows_core::IntoParam<'a, KeyDerivationParameters>>(key: Param0, parameters: Param1, desiredkeysize: u32) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicEngineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeriveKeyMaterial)(::windows_core::Interface::as_raw(this), key.into_param().abi(), parameters.into_param().abi(), desiredkeysize, result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SignHashedData<'a, Param0: ::windows_core::IntoParam<'a, CryptographicKey>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SignHashedData)(::windows_core::Interface::as_raw(this), key.into_param().abi(), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn VerifySignatureWithHashInput<'a, Param0: ::windows_core::IntoParam<'a, CryptographicKey>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, signature: Param2) -> ::windows_core::Result<bool> {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).VerifySignatureWithHashInput)(::windows_core::Interface::as_raw(this), key.into_param().abi(), data.into_param().abi(), signature.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn DecryptAsync<'a, Param0: ::windows_core::IntoParam<'a, CryptographicKey>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1, iv: Param2) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>> {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DecryptAsync)(::windows_core::Interface::as_raw(this), key.into_param().abi(), data.into_param().abi(), iv.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SignAsync<'a, Param0: ::windows_core::IntoParam<'a, CryptographicKey>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>> {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SignAsync)(::windows_core::Interface::as_raw(this), key.into_param().abi(), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SignHashedDataAsync<'a, Param0: ::windows_core::IntoParam<'a, CryptographicKey>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(key: Param0, data: Param1) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>> {
        Self::ICryptographicEngineStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SignHashedDataAsync)(::windows_core::Interface::as_raw(this), key.into_param().abi(), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>(result__)
        })
    }
    pub fn ICryptographicEngineStatics<R, F: FnOnce(&ICryptographicEngineStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CryptographicEngine, ICryptographicEngineStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICryptographicEngineStatics2<R, F: FnOnce(&ICryptographicEngineStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CryptographicEngine, ICryptographicEngineStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for CryptographicEngine {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.CryptographicEngine";
}
#[repr(transparent)]
pub struct CryptographicHash(::windows_core::IUnknown);
impl CryptographicHash {
    #[cfg(feature = "Storage_Streams")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), data.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetValueAndReset(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetValueAndReset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for CryptographicHash {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CryptographicHash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CryptographicHash {}
impl ::core::fmt::Debug for CryptographicHash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicHash").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CryptographicHash {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.CryptographicHash;{5904d1b6-ad31-4603-a3a4-b1bda98e2562})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CryptographicHash {
    type Vtable = IHashComputation_Vtbl;
    const IID: ::windows_core::GUID = <IHashComputation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CryptographicHash {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.CryptographicHash";
}
impl ::core::convert::From<CryptographicHash> for ::windows_core::IUnknown {
    fn from(value: CryptographicHash) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CryptographicHash> for ::windows_core::IUnknown {
    fn from(value: &CryptographicHash) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CryptographicHash {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CryptographicHash {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CryptographicHash> for ::windows_core::IInspectable {
    fn from(value: CryptographicHash) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CryptographicHash> for ::windows_core::IInspectable {
    fn from(value: &CryptographicHash) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CryptographicHash {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CryptographicHash {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CryptographicHash {}
unsafe impl ::core::marker::Sync for CryptographicHash {}
#[repr(transparent)]
pub struct CryptographicKey(::windows_core::IUnknown);
impl CryptographicKey {
    pub fn KeySize(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).KeySize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ExportDefaultPrivateKeyBlobType(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExportDefaultPrivateKeyBlobType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ExportPrivateKeyWithBlobType(&self, blobtype: CryptographicPrivateKeyBlobType) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExportPrivateKeyWithBlobType)(::windows_core::Interface::as_raw(this), blobtype, result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ExportDefaultPublicKeyBlobType(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExportDefaultPublicKeyBlobType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ExportPublicKeyWithBlobType(&self, blobtype: CryptographicPublicKeyBlobType) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExportPublicKeyWithBlobType)(::windows_core::Interface::as_raw(this), blobtype, result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for CryptographicKey {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CryptographicKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CryptographicKey {}
impl ::core::fmt::Debug for CryptographicKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicKey").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CryptographicKey {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.CryptographicKey;{ed2a3b70-8e7b-4009-8401-ffd1a62eeb27})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CryptographicKey {
    type Vtable = ICryptographicKey_Vtbl;
    const IID: ::windows_core::GUID = <ICryptographicKey as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CryptographicKey {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.CryptographicKey";
}
impl ::core::convert::From<CryptographicKey> for ::windows_core::IUnknown {
    fn from(value: CryptographicKey) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CryptographicKey> for ::windows_core::IUnknown {
    fn from(value: &CryptographicKey) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CryptographicKey {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CryptographicKey {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CryptographicKey> for ::windows_core::IInspectable {
    fn from(value: CryptographicKey) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CryptographicKey> for ::windows_core::IInspectable {
    fn from(value: &CryptographicKey) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CryptographicKey {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CryptographicKey {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CryptographicKey {}
unsafe impl ::core::marker::Sync for CryptographicKey {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CryptographicPadding(pub i32);
impl CryptographicPadding {
    pub const None: Self = Self(0i32);
    pub const RsaOaep: Self = Self(1i32);
    pub const RsaPkcs1V15: Self = Self(2i32);
    pub const RsaPss: Self = Self(3i32);
}
impl ::core::marker::Copy for CryptographicPadding {}
impl ::core::clone::Clone for CryptographicPadding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CryptographicPadding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CryptographicPadding {
    type Abi = Self;
}
impl ::core::fmt::Debug for CryptographicPadding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicPadding").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CryptographicPadding {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Core.CryptographicPadding;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CryptographicPrivateKeyBlobType(pub i32);
impl CryptographicPrivateKeyBlobType {
    pub const Pkcs8RawPrivateKeyInfo: Self = Self(0i32);
    pub const Pkcs1RsaPrivateKey: Self = Self(1i32);
    pub const BCryptPrivateKey: Self = Self(2i32);
    pub const Capi1PrivateKey: Self = Self(3i32);
    pub const BCryptEccFullPrivateKey: Self = Self(4i32);
}
impl ::core::marker::Copy for CryptographicPrivateKeyBlobType {}
impl ::core::clone::Clone for CryptographicPrivateKeyBlobType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CryptographicPrivateKeyBlobType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CryptographicPrivateKeyBlobType {
    type Abi = Self;
}
impl ::core::fmt::Debug for CryptographicPrivateKeyBlobType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicPrivateKeyBlobType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CryptographicPrivateKeyBlobType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Core.CryptographicPrivateKeyBlobType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CryptographicPublicKeyBlobType(pub i32);
impl CryptographicPublicKeyBlobType {
    pub const X509SubjectPublicKeyInfo: Self = Self(0i32);
    pub const Pkcs1RsaPublicKey: Self = Self(1i32);
    pub const BCryptPublicKey: Self = Self(2i32);
    pub const Capi1PublicKey: Self = Self(3i32);
    pub const BCryptEccFullPublicKey: Self = Self(4i32);
}
impl ::core::marker::Copy for CryptographicPublicKeyBlobType {}
impl ::core::clone::Clone for CryptographicPublicKeyBlobType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CryptographicPublicKeyBlobType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for CryptographicPublicKeyBlobType {
    type Abi = Self;
}
impl ::core::fmt::Debug for CryptographicPublicKeyBlobType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CryptographicPublicKeyBlobType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CryptographicPublicKeyBlobType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Cryptography.Core.CryptographicPublicKeyBlobType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct EccCurveNames;
impl EccCurveNames {
    pub fn BrainpoolP160r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP160r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP160t1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP160t1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP192r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP192r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP192t1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP192t1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP224r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP224r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP224t1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP224t1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP256r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP256r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP256t1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP256t1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP320r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP320r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP320t1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP320t1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP384r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP384r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP384t1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP384t1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP512r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP512r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BrainpoolP512t1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BrainpoolP512t1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Curve25519() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Curve25519)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Ec192wapi() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Ec192wapi)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn NistP192() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NistP192)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn NistP224() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NistP224)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn NistP256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NistP256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn NistP384() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NistP384)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn NistP521() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NistP521)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn NumsP256t1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NumsP256t1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn NumsP384t1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NumsP384t1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn NumsP512t1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).NumsP512t1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SecP160k1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SecP160k1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SecP160r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SecP160r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SecP160r2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SecP160r2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SecP192k1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SecP192k1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SecP192r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SecP192r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SecP224k1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SecP224k1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SecP224r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SecP224r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SecP256k1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SecP256k1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SecP256r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SecP256r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SecP384r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SecP384r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SecP521r1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SecP521r1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Wtls7() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Wtls7)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Wtls9() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Wtls9)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Wtls12() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Wtls12)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn X962P192v1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).X962P192v1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn X962P192v2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).X962P192v2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn X962P192v3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).X962P192v3)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn X962P239v1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).X962P239v1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn X962P239v2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).X962P239v2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn X962P239v3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).X962P239v3)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn X962P256v1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).X962P256v1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllEccCurveNames() -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::IEccCurveNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AllEccCurveNames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn IEccCurveNamesStatics<R, F: FnOnce(&IEccCurveNamesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EccCurveNames, IEccCurveNamesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for EccCurveNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.EccCurveNames";
}
#[repr(transparent)]
pub struct EncryptedAndAuthenticatedData(::windows_core::IUnknown);
impl EncryptedAndAuthenticatedData {
    #[cfg(feature = "Storage_Streams")]
    pub fn EncryptedData(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EncryptedData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn AuthenticationTag(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AuthenticationTag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for EncryptedAndAuthenticatedData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EncryptedAndAuthenticatedData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EncryptedAndAuthenticatedData {}
impl ::core::fmt::Debug for EncryptedAndAuthenticatedData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EncryptedAndAuthenticatedData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EncryptedAndAuthenticatedData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.EncryptedAndAuthenticatedData;{6fa42fe7-1ecb-4b00-bea5-60b83f862f17})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EncryptedAndAuthenticatedData {
    type Vtable = IEncryptedAndAuthenticatedData_Vtbl;
    const IID: ::windows_core::GUID = <IEncryptedAndAuthenticatedData as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EncryptedAndAuthenticatedData {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.EncryptedAndAuthenticatedData";
}
impl ::core::convert::From<EncryptedAndAuthenticatedData> for ::windows_core::IUnknown {
    fn from(value: EncryptedAndAuthenticatedData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EncryptedAndAuthenticatedData> for ::windows_core::IUnknown {
    fn from(value: &EncryptedAndAuthenticatedData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EncryptedAndAuthenticatedData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EncryptedAndAuthenticatedData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EncryptedAndAuthenticatedData> for ::windows_core::IInspectable {
    fn from(value: EncryptedAndAuthenticatedData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EncryptedAndAuthenticatedData> for ::windows_core::IInspectable {
    fn from(value: &EncryptedAndAuthenticatedData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EncryptedAndAuthenticatedData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EncryptedAndAuthenticatedData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EncryptedAndAuthenticatedData {}
unsafe impl ::core::marker::Sync for EncryptedAndAuthenticatedData {}
pub struct HashAlgorithmNames;
impl HashAlgorithmNames {
    pub fn Md5() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Md5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sha1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sha1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sha256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sha256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sha384() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sha384)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sha512() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IHashAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sha512)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IHashAlgorithmNamesStatics<R, F: FnOnce(&IHashAlgorithmNamesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HashAlgorithmNames, IHashAlgorithmNamesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for HashAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.HashAlgorithmNames";
}
#[repr(transparent)]
pub struct HashAlgorithmProvider(::windows_core::IUnknown);
impl HashAlgorithmProvider {
    pub fn AlgorithmName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AlgorithmName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn HashLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).HashLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn HashData<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HashData)(::windows_core::Interface::as_raw(this), data.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn CreateHash(&self) -> ::windows_core::Result<CryptographicHash> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateHash)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CryptographicHash>(result__)
        }
    }
    pub fn OpenAlgorithm<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(algorithm: Param0) -> ::windows_core::Result<HashAlgorithmProvider> {
        Self::IHashAlgorithmProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenAlgorithm)(::windows_core::Interface::as_raw(this), algorithm.into_param().abi(), result__.as_mut_ptr()).from_abi::<HashAlgorithmProvider>(result__)
        })
    }
    pub fn IHashAlgorithmProviderStatics<R, F: FnOnce(&IHashAlgorithmProviderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<HashAlgorithmProvider, IHashAlgorithmProviderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HashAlgorithmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HashAlgorithmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HashAlgorithmProvider {}
impl ::core::fmt::Debug for HashAlgorithmProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HashAlgorithmProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HashAlgorithmProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.HashAlgorithmProvider;{be9b3080-b2c3-422b-bce1-ec90efb5d7b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HashAlgorithmProvider {
    type Vtable = IHashAlgorithmProvider_Vtbl;
    const IID: ::windows_core::GUID = <IHashAlgorithmProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HashAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.HashAlgorithmProvider";
}
impl ::core::convert::From<HashAlgorithmProvider> for ::windows_core::IUnknown {
    fn from(value: HashAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HashAlgorithmProvider> for ::windows_core::IUnknown {
    fn from(value: &HashAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HashAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HashAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HashAlgorithmProvider> for ::windows_core::IInspectable {
    fn from(value: HashAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HashAlgorithmProvider> for ::windows_core::IInspectable {
    fn from(value: &HashAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HashAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HashAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HashAlgorithmProvider {}
unsafe impl ::core::marker::Sync for HashAlgorithmProvider {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricAlgorithmNamesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAsymmetricAlgorithmNamesStatics {
    type Vtable = IAsymmetricAlgorithmNamesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcaf6fce4_67c0_46aa_84f9_752e77449f9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricAlgorithmNamesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RsaPkcs1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RsaOaepSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RsaOaepSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RsaOaepSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RsaOaepSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EcdsaP256Sha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EcdsaP384Sha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EcdsaP521Sha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DsaSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DsaSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RsaSignPkcs1Sha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RsaSignPkcs1Sha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RsaSignPkcs1Sha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RsaSignPkcs1Sha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RsaSignPssSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RsaSignPssSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RsaSignPssSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RsaSignPssSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricAlgorithmNamesStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAsymmetricAlgorithmNamesStatics2 {
    type Vtable = IAsymmetricAlgorithmNamesStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf141c0d6_4bff_4f23_ba66_6045b137d5df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricAlgorithmNamesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EcdsaSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EcdsaSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EcdsaSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAsymmetricKeyAlgorithmProvider {
    type Vtable = IAsymmetricKeyAlgorithmProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8d2ff37_6259_4e88_b7e0_94191fde699e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricKeyAlgorithmProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CreateKeyPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keysize: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ImportDefaultPrivateKeyBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyblob: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportDefaultPrivateKeyBlob: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImportKeyPairWithBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyblob: ::windows_core::RawPtr, blobtype: CryptographicPrivateKeyBlobType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportKeyPairWithBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImportDefaultPublicKeyBlob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyblob: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportDefaultPublicKeyBlob: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ImportPublicKeyWithBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyblob: ::windows_core::RawPtr, blobtype: CryptographicPublicKeyBlobType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportPublicKeyWithBlobType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProvider2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAsymmetricKeyAlgorithmProvider2 {
    type Vtable = IAsymmetricKeyAlgorithmProvider2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e322a7e_7c4d_4997_ac4f_1b848b36306e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricKeyAlgorithmProvider2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateKeyPairWithCurveName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, curvename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateKeyPairWithCurveParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters_array_size: u32, parameters: *const u8, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAsymmetricKeyAlgorithmProviderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAsymmetricKeyAlgorithmProviderStatics {
    type Vtable = IAsymmetricKeyAlgorithmProviderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x425bde18_a7f3_47a6_a8d2_c48d6033a65c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsymmetricKeyAlgorithmProviderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICryptographicEngineStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICryptographicEngineStatics {
    type Vtable = ICryptographicEngineStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fea0639_6ff7_4c85_a095_95eb31715eb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicEngineStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Encrypt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::RawPtr, data: ::windows_core::RawPtr, iv: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Encrypt: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Decrypt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::RawPtr, data: ::windows_core::RawPtr, iv: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Decrypt: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncryptAndAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::RawPtr, data: ::windows_core::RawPtr, nonce: ::windows_core::RawPtr, authenticateddata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncryptAndAuthenticate: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DecryptAndAuthenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::RawPtr, data: ::windows_core::RawPtr, nonce: ::windows_core::RawPtr, authenticationtag: ::windows_core::RawPtr, authenticateddata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DecryptAndAuthenticate: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Sign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::RawPtr, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Sign: usize,
    #[cfg(feature = "Storage_Streams")]
    pub VerifySignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::RawPtr, data: ::windows_core::RawPtr, signature: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VerifySignature: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DeriveKeyMaterial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::RawPtr, parameters: ::windows_core::RawPtr, desiredkeysize: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DeriveKeyMaterial: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICryptographicEngineStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICryptographicEngineStatics2 {
    type Vtable = ICryptographicEngineStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x675948fe_df9f_4191_92c7_6ce6f58420e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicEngineStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SignHashedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::RawPtr, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SignHashedData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub VerifySignatureWithHashInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::RawPtr, data: ::windows_core::RawPtr, signature: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VerifySignatureWithHashInput: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub DecryptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::RawPtr, data: ::windows_core::RawPtr, iv: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    DecryptAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SignAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::RawPtr, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SignAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SignHashedDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::RawPtr, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SignHashedDataAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICryptographicKey(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICryptographicKey {
    type Vtable = ICryptographicKey_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed2a3b70_8e7b_4009_8401_ffd1a62eeb27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICryptographicKey_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ExportDefaultPrivateKeyBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ExportDefaultPrivateKeyBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ExportPrivateKeyWithBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobtype: CryptographicPrivateKeyBlobType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ExportPrivateKeyWithBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ExportDefaultPublicKeyBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ExportDefaultPublicKeyBlobType: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ExportPublicKeyWithBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobtype: CryptographicPublicKeyBlobType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ExportPublicKeyWithBlobType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEccCurveNamesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEccCurveNamesStatics {
    type Vtable = IEccCurveNamesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3ff930c_aeeb_409e_b7d4_9b95295aaecf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEccCurveNamesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BrainpoolP160r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP160t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP192r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP192t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP224r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP224t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP256r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP256t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP320r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP320t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP384r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP384t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP512r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BrainpoolP512t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Curve25519: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Ec192wapi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NistP192: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NistP224: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NistP256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NistP384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NistP521: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NumsP256t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NumsP384t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub NumsP512t1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SecP160k1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SecP160r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SecP160r2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SecP192k1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SecP192r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SecP224k1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SecP224r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SecP256k1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SecP256r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SecP384r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SecP521r1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Wtls7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Wtls9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Wtls12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub X962P192v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub X962P192v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub X962P192v3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub X962P239v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub X962P239v2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub X962P239v3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub X962P256v1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllEccCurveNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllEccCurveNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEncryptedAndAuthenticatedData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEncryptedAndAuthenticatedData {
    type Vtable = IEncryptedAndAuthenticatedData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6fa42fe7_1ecb_4b00_bea5_60b83f862f17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEncryptedAndAuthenticatedData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub EncryptedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncryptedData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AuthenticationTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AuthenticationTag: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHashAlgorithmNamesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHashAlgorithmNamesStatics {
    type Vtable = IHashAlgorithmNamesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b5e0516_de96_4f0a_8d57_dcc9dae36c76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHashAlgorithmNamesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Md5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHashAlgorithmProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHashAlgorithmProvider {
    type Vtable = IHashAlgorithmProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe9b3080_b2c3_422b_bce1_ec90efb5d7b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHashAlgorithmProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HashLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub HashData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    HashData: usize,
    pub CreateHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHashAlgorithmProviderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHashAlgorithmProviderStatics {
    type Vtable = IHashAlgorithmProviderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9fac9741_5cc4_4336_ae38_6212b75a915a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHashAlgorithmProviderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHashComputation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHashComputation {
    type Vtable = IHashComputation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5904d1b6_ad31_4603_a3a4_b1bda98e2562);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHashComputation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Append: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetValueAndReset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetValueAndReset: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmNamesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyDerivationAlgorithmNamesStatics {
    type Vtable = IKeyDerivationAlgorithmNamesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b6e363e_94d2_4739_a57b_022e0c3a402a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationAlgorithmNamesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Pbkdf2Md5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Pbkdf2Sha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Pbkdf2Sha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Pbkdf2Sha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Pbkdf2Sha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sp800108CtrHmacMd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sp800108CtrHmacSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sp800108CtrHmacSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sp800108CtrHmacSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sp800108CtrHmacSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sp80056aConcatMd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sp80056aConcatSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sp80056aConcatSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sp80056aConcatSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sp80056aConcatSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmNamesStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyDerivationAlgorithmNamesStatics2 {
    type Vtable = IKeyDerivationAlgorithmNamesStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57953fab_6044_466f_97f4_337b7808384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationAlgorithmNamesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CapiKdfMd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CapiKdfSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CapiKdfSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CapiKdfSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CapiKdfSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyDerivationAlgorithmProvider {
    type Vtable = IKeyDerivationAlgorithmProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1fba83b_4671_43b7_9158_763aaa98b6bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationAlgorithmProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keymaterial: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateKey: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationAlgorithmProviderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyDerivationAlgorithmProviderStatics {
    type Vtable = IKeyDerivationAlgorithmProviderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a22097a_0a1c_443b_9418_b9498aeb1603);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationAlgorithmProviderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationParameters(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyDerivationParameters {
    type Vtable = IKeyDerivationParameters_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7bf05967_047b_4a8c_964a_469ffd5522e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationParameters_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub KdfGenericBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    KdfGenericBinary: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetKdfGenericBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetKdfGenericBinary: usize,
    pub IterationCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationParameters2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyDerivationParameters2 {
    type Vtable = IKeyDerivationParameters2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd4166d1_417e_4f4c_b666_c0d879f3f8e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationParameters2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Capi1KdfTargetAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Capi1KdfTargetAlgorithm) -> ::windows_core::HRESULT,
    pub SetCapi1KdfTargetAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Capi1KdfTargetAlgorithm) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationParametersStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyDerivationParametersStatics {
    type Vtable = IKeyDerivationParametersStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea961fbe_f37f_4146_9dfe_a456f1735f4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationParametersStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub BuildForPbkdf2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbkdf2salt: ::windows_core::RawPtr, iterationcount: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BuildForPbkdf2: usize,
    #[cfg(feature = "Storage_Streams")]
    pub BuildForSP800108: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::windows_core::RawPtr, context: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BuildForSP800108: usize,
    #[cfg(feature = "Storage_Streams")]
    pub BuildForSP80056a: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithmid: ::windows_core::RawPtr, partyuinfo: ::windows_core::RawPtr, partyvinfo: ::windows_core::RawPtr, supppubinfo: ::windows_core::RawPtr, suppprivinfo: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    BuildForSP80056a: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyDerivationParametersStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyDerivationParametersStatics2 {
    type Vtable = IKeyDerivationParametersStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5783dd5_58e3_4efb_b283_a1653126e1be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyDerivationParametersStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BuildForCapi1Kdf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capi1kdftargetalgorithm: Capi1KdfTargetAlgorithm, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMacAlgorithmNamesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMacAlgorithmNamesStatics {
    type Vtable = IMacAlgorithmNamesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41412678_fb1e_43a4_895e_a9026e4390a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMacAlgorithmNamesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HmacMd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HmacSha1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HmacSha256: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HmacSha384: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HmacSha512: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AesCmac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMacAlgorithmProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMacAlgorithmProvider {
    type Vtable = IMacAlgorithmProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a3fc5c3_1cbd_41ce_a092_aa0bc5d2d2f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMacAlgorithmProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MacLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keymaterial: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateKey: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMacAlgorithmProvider2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMacAlgorithmProvider2 {
    type Vtable = IMacAlgorithmProvider2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6da32a15_d931_42ed_8e7e_c301caee119c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMacAlgorithmProvider2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keymaterial: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateHash: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMacAlgorithmProviderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMacAlgorithmProviderStatics {
    type Vtable = IMacAlgorithmProviderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9bdc147_cc77_4df0_9e4e_b921e080644c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMacAlgorithmProviderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPersistedKeyProviderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPersistedKeyProviderStatics {
    type Vtable = IPersistedKeyProviderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77274814_d9d4_4cf5_b668_e0457df30894);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistedKeyProviderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates"))]
    pub OpenKeyPairFromCertificateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: ::windows_core::RawPtr, hashalgorithmname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, padding: CryptographicPadding, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Certificates")))]
    OpenKeyPairFromCertificateAsync: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub OpenPublicKeyFromCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificate: ::windows_core::RawPtr, hashalgorithmname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, padding: CryptographicPadding, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    OpenPublicKeyFromCertificate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISymmetricAlgorithmNamesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISymmetricAlgorithmNamesStatics {
    type Vtable = ISymmetricAlgorithmNamesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6870727b_c996_4eae_84d7_79b2aeb73b9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISymmetricAlgorithmNamesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DesCbc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DesEcb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TripleDesCbc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TripleDesEcb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Rc2Cbc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Rc2Ecb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AesCbc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AesEcb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AesGcm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AesCcm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AesCbcPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AesEcbPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DesCbcPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DesEcbPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TripleDesCbcPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TripleDesEcbPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Rc2CbcPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Rc2EcbPkcs7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Rc4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISymmetricKeyAlgorithmProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISymmetricKeyAlgorithmProvider {
    type Vtable = ISymmetricKeyAlgorithmProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d7e4a33_3bd0_4902_8ac8_470d50d21376);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISymmetricKeyAlgorithmProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AlgorithmName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BlockLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateSymmetricKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keymaterial: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateSymmetricKey: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISymmetricKeyAlgorithmProviderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISymmetricKeyAlgorithmProviderStatics {
    type Vtable = ISymmetricKeyAlgorithmProviderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d3b2326_1f37_491f_b60e_f5431b26b483);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISymmetricKeyAlgorithmProviderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, algorithm: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
pub struct KeyDerivationAlgorithmNames;
impl KeyDerivationAlgorithmNames {
    pub fn Pbkdf2Md5() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Pbkdf2Md5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Pbkdf2Sha1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Pbkdf2Sha1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Pbkdf2Sha256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Pbkdf2Sha256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Pbkdf2Sha384() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Pbkdf2Sha384)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Pbkdf2Sha512() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Pbkdf2Sha512)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sp800108CtrHmacMd5() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sp800108CtrHmacMd5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sp800108CtrHmacSha1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sp800108CtrHmacSha1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sp800108CtrHmacSha256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sp800108CtrHmacSha256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sp800108CtrHmacSha384() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sp800108CtrHmacSha384)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sp800108CtrHmacSha512() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sp800108CtrHmacSha512)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sp80056aConcatMd5() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sp80056aConcatMd5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sp80056aConcatSha1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sp80056aConcatSha1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sp80056aConcatSha256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sp80056aConcatSha256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sp80056aConcatSha384() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sp80056aConcatSha384)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Sp80056aConcatSha512() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Sp80056aConcatSha512)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn CapiKdfMd5() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CapiKdfMd5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn CapiKdfSha1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CapiKdfSha1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn CapiKdfSha256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CapiKdfSha256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn CapiKdfSha384() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CapiKdfSha384)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn CapiKdfSha512() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKeyDerivationAlgorithmNamesStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CapiKdfSha512)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IKeyDerivationAlgorithmNamesStatics<R, F: FnOnce(&IKeyDerivationAlgorithmNamesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KeyDerivationAlgorithmNames, IKeyDerivationAlgorithmNamesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKeyDerivationAlgorithmNamesStatics2<R, F: FnOnce(&IKeyDerivationAlgorithmNamesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KeyDerivationAlgorithmNames, IKeyDerivationAlgorithmNamesStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for KeyDerivationAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.KeyDerivationAlgorithmNames";
}
#[repr(transparent)]
pub struct KeyDerivationAlgorithmProvider(::windows_core::IUnknown);
impl KeyDerivationAlgorithmProvider {
    pub fn AlgorithmName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AlgorithmName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keymaterial: Param0) -> ::windows_core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateKey)(::windows_core::Interface::as_raw(this), keymaterial.into_param().abi(), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    pub fn OpenAlgorithm<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(algorithm: Param0) -> ::windows_core::Result<KeyDerivationAlgorithmProvider> {
        Self::IKeyDerivationAlgorithmProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenAlgorithm)(::windows_core::Interface::as_raw(this), algorithm.into_param().abi(), result__.as_mut_ptr()).from_abi::<KeyDerivationAlgorithmProvider>(result__)
        })
    }
    pub fn IKeyDerivationAlgorithmProviderStatics<R, F: FnOnce(&IKeyDerivationAlgorithmProviderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KeyDerivationAlgorithmProvider, IKeyDerivationAlgorithmProviderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for KeyDerivationAlgorithmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyDerivationAlgorithmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyDerivationAlgorithmProvider {}
impl ::core::fmt::Debug for KeyDerivationAlgorithmProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyDerivationAlgorithmProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyDerivationAlgorithmProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.KeyDerivationAlgorithmProvider;{e1fba83b-4671-43b7-9158-763aaa98b6bf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeyDerivationAlgorithmProvider {
    type Vtable = IKeyDerivationAlgorithmProvider_Vtbl;
    const IID: ::windows_core::GUID = <IKeyDerivationAlgorithmProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeyDerivationAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.KeyDerivationAlgorithmProvider";
}
impl ::core::convert::From<KeyDerivationAlgorithmProvider> for ::windows_core::IUnknown {
    fn from(value: KeyDerivationAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyDerivationAlgorithmProvider> for ::windows_core::IUnknown {
    fn from(value: &KeyDerivationAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeyDerivationAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeyDerivationAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyDerivationAlgorithmProvider> for ::windows_core::IInspectable {
    fn from(value: KeyDerivationAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyDerivationAlgorithmProvider> for ::windows_core::IInspectable {
    fn from(value: &KeyDerivationAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeyDerivationAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeyDerivationAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyDerivationAlgorithmProvider {}
unsafe impl ::core::marker::Sync for KeyDerivationAlgorithmProvider {}
#[repr(transparent)]
pub struct KeyDerivationParameters(::windows_core::IUnknown);
impl KeyDerivationParameters {
    #[cfg(feature = "Storage_Streams")]
    pub fn KdfGenericBinary(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KdfGenericBinary)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetKdfGenericBinary<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKdfGenericBinary)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IterationCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).IterationCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Capi1KdfTargetAlgorithm(&self) -> ::windows_core::Result<Capi1KdfTargetAlgorithm> {
        let this = &::windows_core::Interface::cast::<IKeyDerivationParameters2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<Capi1KdfTargetAlgorithm>::zeroed();
            (::windows_core::Interface::vtable(this).Capi1KdfTargetAlgorithm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Capi1KdfTargetAlgorithm>(result__)
        }
    }
    pub fn SetCapi1KdfTargetAlgorithm(&self, value: Capi1KdfTargetAlgorithm) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IKeyDerivationParameters2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCapi1KdfTargetAlgorithm)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn BuildForPbkdf2<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(pbkdf2salt: Param0, iterationcount: u32) -> ::windows_core::Result<KeyDerivationParameters> {
        Self::IKeyDerivationParametersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BuildForPbkdf2)(::windows_core::Interface::as_raw(this), pbkdf2salt.into_param().abi(), iterationcount, result__.as_mut_ptr()).from_abi::<KeyDerivationParameters>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn BuildForSP800108<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(label: Param0, context: Param1) -> ::windows_core::Result<KeyDerivationParameters> {
        Self::IKeyDerivationParametersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BuildForSP800108)(::windows_core::Interface::as_raw(this), label.into_param().abi(), context.into_param().abi(), result__.as_mut_ptr()).from_abi::<KeyDerivationParameters>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn BuildForSP80056a<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param1: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param2: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param3: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param4: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(algorithmid: Param0, partyuinfo: Param1, partyvinfo: Param2, supppubinfo: Param3, suppprivinfo: Param4) -> ::windows_core::Result<KeyDerivationParameters> {
        Self::IKeyDerivationParametersStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BuildForSP80056a)(::windows_core::Interface::as_raw(this), algorithmid.into_param().abi(), partyuinfo.into_param().abi(), partyvinfo.into_param().abi(), supppubinfo.into_param().abi(), suppprivinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<KeyDerivationParameters>(result__)
        })
    }
    pub fn BuildForCapi1Kdf(capi1kdftargetalgorithm: Capi1KdfTargetAlgorithm) -> ::windows_core::Result<KeyDerivationParameters> {
        Self::IKeyDerivationParametersStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BuildForCapi1Kdf)(::windows_core::Interface::as_raw(this), capi1kdftargetalgorithm, result__.as_mut_ptr()).from_abi::<KeyDerivationParameters>(result__)
        })
    }
    pub fn IKeyDerivationParametersStatics<R, F: FnOnce(&IKeyDerivationParametersStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KeyDerivationParameters, IKeyDerivationParametersStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IKeyDerivationParametersStatics2<R, F: FnOnce(&IKeyDerivationParametersStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KeyDerivationParameters, IKeyDerivationParametersStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for KeyDerivationParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyDerivationParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyDerivationParameters {}
impl ::core::fmt::Debug for KeyDerivationParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyDerivationParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyDerivationParameters {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.KeyDerivationParameters;{7bf05967-047b-4a8c-964a-469ffd5522e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeyDerivationParameters {
    type Vtable = IKeyDerivationParameters_Vtbl;
    const IID: ::windows_core::GUID = <IKeyDerivationParameters as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeyDerivationParameters {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.KeyDerivationParameters";
}
impl ::core::convert::From<KeyDerivationParameters> for ::windows_core::IUnknown {
    fn from(value: KeyDerivationParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyDerivationParameters> for ::windows_core::IUnknown {
    fn from(value: &KeyDerivationParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeyDerivationParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeyDerivationParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyDerivationParameters> for ::windows_core::IInspectable {
    fn from(value: KeyDerivationParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyDerivationParameters> for ::windows_core::IInspectable {
    fn from(value: &KeyDerivationParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeyDerivationParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeyDerivationParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyDerivationParameters {}
unsafe impl ::core::marker::Sync for KeyDerivationParameters {}
pub struct MacAlgorithmNames;
impl MacAlgorithmNames {
    pub fn HmacMd5() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HmacMd5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn HmacSha1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HmacSha1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn HmacSha256() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HmacSha256)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn HmacSha384() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HmacSha384)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn HmacSha512() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HmacSha512)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AesCmac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMacAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AesCmac)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IMacAlgorithmNamesStatics<R, F: FnOnce(&IMacAlgorithmNamesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MacAlgorithmNames, IMacAlgorithmNamesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for MacAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.MacAlgorithmNames";
}
#[repr(transparent)]
pub struct MacAlgorithmProvider(::windows_core::IUnknown);
impl MacAlgorithmProvider {
    pub fn AlgorithmName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AlgorithmName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn MacLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MacLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keymaterial: Param0) -> ::windows_core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateKey)(::windows_core::Interface::as_raw(this), keymaterial.into_param().abi(), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateHash<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keymaterial: Param0) -> ::windows_core::Result<CryptographicHash> {
        let this = &::windows_core::Interface::cast::<IMacAlgorithmProvider2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateHash)(::windows_core::Interface::as_raw(this), keymaterial.into_param().abi(), result__.as_mut_ptr()).from_abi::<CryptographicHash>(result__)
        }
    }
    pub fn OpenAlgorithm<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(algorithm: Param0) -> ::windows_core::Result<MacAlgorithmProvider> {
        Self::IMacAlgorithmProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenAlgorithm)(::windows_core::Interface::as_raw(this), algorithm.into_param().abi(), result__.as_mut_ptr()).from_abi::<MacAlgorithmProvider>(result__)
        })
    }
    pub fn IMacAlgorithmProviderStatics<R, F: FnOnce(&IMacAlgorithmProviderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<MacAlgorithmProvider, IMacAlgorithmProviderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MacAlgorithmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MacAlgorithmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MacAlgorithmProvider {}
impl ::core::fmt::Debug for MacAlgorithmProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MacAlgorithmProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MacAlgorithmProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.MacAlgorithmProvider;{4a3fc5c3-1cbd-41ce-a092-aa0bc5d2d2f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MacAlgorithmProvider {
    type Vtable = IMacAlgorithmProvider_Vtbl;
    const IID: ::windows_core::GUID = <IMacAlgorithmProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MacAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.MacAlgorithmProvider";
}
impl ::core::convert::From<MacAlgorithmProvider> for ::windows_core::IUnknown {
    fn from(value: MacAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MacAlgorithmProvider> for ::windows_core::IUnknown {
    fn from(value: &MacAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MacAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MacAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MacAlgorithmProvider> for ::windows_core::IInspectable {
    fn from(value: MacAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MacAlgorithmProvider> for ::windows_core::IInspectable {
    fn from(value: &MacAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MacAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MacAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MacAlgorithmProvider {}
unsafe impl ::core::marker::Sync for MacAlgorithmProvider {}
pub struct PersistedKeyProvider;
impl PersistedKeyProvider {
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates"))]
    pub fn OpenKeyPairFromCertificateAsync<'a, Param0: ::windows_core::IntoParam<'a, super::Certificates::Certificate>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(certificate: Param0, hashalgorithmname: Param1, padding: CryptographicPadding) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<CryptographicKey>> {
        Self::IPersistedKeyProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenKeyPairFromCertificateAsync)(::windows_core::Interface::as_raw(this), certificate.into_param().abi(), hashalgorithmname.into_param().abi(), padding, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<CryptographicKey>>(result__)
        })
    }
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn OpenPublicKeyFromCertificate<'a, Param0: ::windows_core::IntoParam<'a, super::Certificates::Certificate>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(certificate: Param0, hashalgorithmname: Param1, padding: CryptographicPadding) -> ::windows_core::Result<CryptographicKey> {
        Self::IPersistedKeyProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenPublicKeyFromCertificate)(::windows_core::Interface::as_raw(this), certificate.into_param().abi(), hashalgorithmname.into_param().abi(), padding, result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        })
    }
    pub fn IPersistedKeyProviderStatics<R, F: FnOnce(&IPersistedKeyProviderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PersistedKeyProvider, IPersistedKeyProviderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for PersistedKeyProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.PersistedKeyProvider";
}
pub struct SymmetricAlgorithmNames;
impl SymmetricAlgorithmNames {
    pub fn DesCbc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DesCbc)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DesEcb() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DesEcb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn TripleDesCbc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TripleDesCbc)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn TripleDesEcb() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TripleDesEcb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Rc2Cbc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Rc2Cbc)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Rc2Ecb() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Rc2Ecb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AesCbc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AesCbc)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AesEcb() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AesEcb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AesGcm() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AesGcm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AesCcm() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AesCcm)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AesCbcPkcs7() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AesCbcPkcs7)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn AesEcbPkcs7() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AesEcbPkcs7)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DesCbcPkcs7() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DesCbcPkcs7)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn DesEcbPkcs7() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DesEcbPkcs7)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn TripleDesCbcPkcs7() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TripleDesCbcPkcs7)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn TripleDesEcbPkcs7() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TripleDesEcbPkcs7)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Rc2CbcPkcs7() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Rc2CbcPkcs7)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Rc2EcbPkcs7() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Rc2EcbPkcs7)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Rc4() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISymmetricAlgorithmNamesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Rc4)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn ISymmetricAlgorithmNamesStatics<R, F: FnOnce(&ISymmetricAlgorithmNamesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SymmetricAlgorithmNames, ISymmetricAlgorithmNamesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SymmetricAlgorithmNames {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.SymmetricAlgorithmNames";
}
#[repr(transparent)]
pub struct SymmetricKeyAlgorithmProvider(::windows_core::IUnknown);
impl SymmetricKeyAlgorithmProvider {
    pub fn AlgorithmName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AlgorithmName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn BlockLength(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).BlockLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateSymmetricKey<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, keymaterial: Param0) -> ::windows_core::Result<CryptographicKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateSymmetricKey)(::windows_core::Interface::as_raw(this), keymaterial.into_param().abi(), result__.as_mut_ptr()).from_abi::<CryptographicKey>(result__)
        }
    }
    pub fn OpenAlgorithm<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(algorithm: Param0) -> ::windows_core::Result<SymmetricKeyAlgorithmProvider> {
        Self::ISymmetricKeyAlgorithmProviderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenAlgorithm)(::windows_core::Interface::as_raw(this), algorithm.into_param().abi(), result__.as_mut_ptr()).from_abi::<SymmetricKeyAlgorithmProvider>(result__)
        })
    }
    pub fn ISymmetricKeyAlgorithmProviderStatics<R, F: FnOnce(&ISymmetricKeyAlgorithmProviderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SymmetricKeyAlgorithmProvider, ISymmetricKeyAlgorithmProviderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SymmetricKeyAlgorithmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SymmetricKeyAlgorithmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SymmetricKeyAlgorithmProvider {}
impl ::core::fmt::Debug for SymmetricKeyAlgorithmProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SymmetricKeyAlgorithmProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SymmetricKeyAlgorithmProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Cryptography.Core.SymmetricKeyAlgorithmProvider;{3d7e4a33-3bd0-4902-8ac8-470d50d21376})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SymmetricKeyAlgorithmProvider {
    type Vtable = ISymmetricKeyAlgorithmProvider_Vtbl;
    const IID: ::windows_core::GUID = <ISymmetricKeyAlgorithmProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SymmetricKeyAlgorithmProvider {
    const NAME: &'static str = "Windows.Security.Cryptography.Core.SymmetricKeyAlgorithmProvider";
}
impl ::core::convert::From<SymmetricKeyAlgorithmProvider> for ::windows_core::IUnknown {
    fn from(value: SymmetricKeyAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SymmetricKeyAlgorithmProvider> for ::windows_core::IUnknown {
    fn from(value: &SymmetricKeyAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SymmetricKeyAlgorithmProvider> for ::windows_core::IInspectable {
    fn from(value: SymmetricKeyAlgorithmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SymmetricKeyAlgorithmProvider> for ::windows_core::IInspectable {
    fn from(value: &SymmetricKeyAlgorithmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SymmetricKeyAlgorithmProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SymmetricKeyAlgorithmProvider {}
unsafe impl ::core::marker::Sync for SymmetricKeyAlgorithmProvider {}