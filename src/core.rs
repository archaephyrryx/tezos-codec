pub mod base58;

use tedium::FixedBytes;

#[macro_export]
macro_rules! boilerplate {
    (@ refonly $($tname:ident = $n:literal),+ $(,)?) => {
       $(
            impl AsPayload for $tname {
                fn as_payload(&self) -> &[u8] {
                    self.as_array_ref()
                }
            }

            impl AsRef<[u8; $n]> for $tname {
                fn as_ref(&self) -> &[u8; $n] {
                    self.as_array_ref()
                }
            }

            impl AsRef<[u8]> for $tname {
                fn as_ref(&self) -> &[u8] {
                    self.as_array_ref()
                }
            }

            impl AsRef<FixedBytes<$n>> for $tname {
                fn as_ref(&self) -> &FixedBytes<$n> {
                    self.as_fixed_bytes()
                }
            }
        )+
    };
    ($($tname:ident = $n:literal),+ $(,)?) => {
        $(
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
            pub struct $tname(::tedium::FixedBytes<$n>);

            impl $crate::traits::AsPayload for $tname {
                fn as_payload(&self) -> &[u8] {
                    self.0.bytes()
                }
            }

            impl $crate::traits::BinaryDataType<$n> for $tname {
                fn as_array_ref(&self) -> &[u8; $n] {
                    self.0.bytes()
                }

                fn as_fixed_bytes(&self) -> &::tedium::FixedBytes<$n> {
                    &self.0
                }
            }

            impl $tname {
                pub const fn from_byte_array(b: [u8; $n]) -> Self {
                    Self(::tedium::FixedBytes::<$n>::from_array(b))
                }

                pub const fn from_fixed_bytes(bytes: ::tedium::FixedBytes<$n>) -> Self {
                    Self(bytes)
                }
            }

            impl From<[u8; $n]> for $tname {
                fn from(value: [u8; $n]) -> Self {
                    Self::from_byte_array(value)
                }
            }

            impl From<&'_ [u8; $n]> for $tname {
                fn from(value: &'_ [u8; $n]) -> Self {
                    Self::from_byte_array(*value)
                }
            }

            impl From<::tedium::FixedBytes<$n>> for $tname {
                fn from(value: ::tedium::FixedBytes<$n>) -> Self {
                    Self::from_fixed_bytes(value)
                }
            }

            impl AsRef<[u8; $n]> for $tname {
                fn as_ref(&self) -> &[u8; $n] {
                    self.0.bytes()
                }
            }

            impl AsRef<[u8]> for $tname {
                fn as_ref(&self) -> &[u8] {
                    self.0.bytes()
                }
            }

            impl AsRef<::tedium::FixedBytes<$n>> for $tname {
                fn as_ref(&self) -> &::tedium::FixedBytes<$n> {
                    &self.0
                }
            }

            impl TryFrom<&'_ [u8]> for $tname {
                type Error = std::array::TryFromSliceError;

                fn try_from(value: &'_ [u8]) -> Result<Self, Self::Error> {
                    Ok(Self::from_byte_array(value.try_into()?))
                }
            }
        )+
    };
}

use crate::{ traits::{ AsPayload, Crypto, StaticPrefix, DynamicPrefix }, impl_crypto_display };

boilerplate!(OperationHash = 32);
impl_crypto_display!(OperationHash);

impl OperationHash {
    /// Preimage of ciphertext prefix `o`
    ///
    /// TODO: implement mutation tests to verify the correct ciphertext prefix
    pub const BASE58_PREFIX: [u8; 2] = [5, 116];
}

impl StaticPrefix for OperationHash {
    const PREFIX: &'static [u8] = &Self::BASE58_PREFIX;
}

impl Crypto for OperationHash {}

boilerplate!(ChainId = 4);
impl_crypto_display!(ChainId);

impl ChainId {
    /// Preimage of ciphertext prefix `net`
    ///
    /// TODO: implement mutation tests to verify the correct ciphertext prefix
    pub const BASE58_PREFIX: [u8; 3] = [87, 82, 0];
}

impl StaticPrefix for ChainId {
    const PREFIX: &'static [u8] = &Self::BASE58_PREFIX;
}

impl Crypto for ChainId {}

boilerplate!(BlockHash = 32);
impl_crypto_display!(BlockHash);

impl BlockHash {
    /// Preimage of ciphertext prefix `B`
    ///
    /// TODO: implement mutation tests to verify the correct ciphertext prefix
    pub const BASE58_PREFIX: [u8; 2] = [1, 52];
}

impl StaticPrefix for BlockHash {
    const PREFIX: &'static [u8] = &Self::BASE58_PREFIX;
}

impl Crypto for BlockHash {}

boilerplate!(ContextHash = 32);
impl_crypto_display!(ContextHash);
impl ContextHash {
    /// Preimage of ciphertext prefix `Co`.
    ///
    /// TODO: implement mutation tests to verify the correct ciphertext prefix
    pub const BASE58_PREFIX : [u8; 2] = [79, 199];
}
impl StaticPrefix for ContextHash {
    const PREFIX : &'static [u8] = &Self::BASE58_PREFIX;
}
impl Crypto for ContextHash {}

boilerplate!(OperationListListHash = 32);
impl_crypto_display!(OperationListListHash);

impl OperationListListHash {
    /// Preimage of ciphertext prefix `LLo`
    ///
    /// TODO: implement mutation tests to verify the correct ciphertext prefix
    pub const BASE58_PREFIX : [u8; 3] = [29, 159, 109];
}

impl StaticPrefix for OperationListListHash {
    const PREFIX: &'static [u8] = &Self::BASE58_PREFIX;
}

impl Crypto for OperationListListHash {}







boilerplate!(ProtocolHash = 32);
impl_crypto_display!(ProtocolHash);

impl ProtocolHash {
    /// Preimage of ciphertext prefix `P`.
    ///
    /// TODO: implement mutation tests to verify the correct ciphertext prefix
    pub const BASE58_PREFIX: [u8; 2] = [2, 170];
}

impl StaticPrefix for ProtocolHash {
    const PREFIX: &'static [u8] = &Self::BASE58_PREFIX;
}

impl Crypto for ProtocolHash {}

boilerplate!(SignatureV0 = 64);
impl_crypto_display!(SignatureV0);

impl SignatureV0 {
    pub const BASE58_PREFIX: [u8; 3] = [4, 130, 43];
}

impl StaticPrefix for SignatureV0 {
    const PREFIX: &'static [u8] = &Self::BASE58_PREFIX;
}

impl Crypto for SignatureV0 {}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PublicKeyHashV0 {
    Ed25519(FixedBytes<20>),
    Secp256k1(FixedBytes<20>),
    P256(FixedBytes<20>),
}

boilerplate!(@refonly PublicKeyHashV0 = 20);
impl_crypto_display!(PublicKeyHashV0);

impl PublicKeyHashV0 {
    /// Preimage of ciphertext prefix `tz1`
    ///
    /// TODO: implement mutation tests to verify the correct ciphertext prefix
    pub const ED25519_BASE58_PREFIX: [u8; 3] = [6, 161, 159];

    /// Preimage of ciphertext prefix `tz2`
    ///
    /// TODO: implement mutation tests to verify the correct ciphertext prefix
    pub const SECP256K1_BASE58_PREFIX: [u8; 3] = [6, 161, 161];

    /// Preimage of ciphertext prefix `tz3`
    ///
    /// TODO: implement mutation tests to verify the correct ciphertext prefix
    pub const P256_BASE58_PREFIX: [u8; 3] = [6, 161, 164];

    #[must_use]
    #[inline]
    /// Converts a borrowed [`PublicKeyHashV0`] into a reference to its constituent bytes,
    /// discarding any distinction as to what cryptographic algorithm it corresponds to.
    pub const fn as_fixed_bytes(&self) -> &FixedBytes<20> {
        match self {
            Self::Ed25519(bytes) | Self::Secp256k1(bytes) | Self::P256(bytes) => bytes,
        }
    }

    #[must_use]
    #[inline]
    /// Converts a borrowed [`PublicKeyHashV0`] into a reference to its constituent byte-array,
    /// discarding any distinction as to what cryptographic algorithm it corresponds to.
    pub const fn as_array_ref(&self) -> &[u8; 20] {
        match self {
            Self::Ed25519(bytes) => bytes.bytes(),
            Self::Secp256k1(bytes) => bytes.bytes(),
            Self::P256(bytes) => bytes.bytes(),
        }
    }
}

impl DynamicPrefix for PublicKeyHashV0 {
    fn get_prefix(&self) -> &'static [u8] {
        match self {
            PublicKeyHashV0::Ed25519(_) => &Self::ED25519_BASE58_PREFIX,
            PublicKeyHashV0::Secp256k1(_) => &Self::SECP256K1_BASE58_PREFIX,
            PublicKeyHashV0::P256(_) => &Self::P256_BASE58_PREFIX,
        }
    }
}

impl Crypto for PublicKeyHashV0 {}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Timestamp(i64);

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: implement RFC3339 (chrono) display
        self.0.fmt(f)
    }
}

impl Timestamp {
    #[inline]
    #[must_use]
    /// Upcasts an [`i64`] to a representationally equivalent [`Timestamp`].
    pub const fn from_i64(i: i64) -> Self {
        Self(i)
    }

}

impl From<Timestamp> for i64 {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

impl From<i64> for Timestamp {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl AsRef<i64> for Timestamp {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl std::borrow::Borrow<i64> for Timestamp {
    fn borrow(&self) -> &i64 {
        &self.0
    }
}