//! PKCS#8 `EncryptedPrivateKeyInfo`

use crate::{Error, Result};
use core::convert::TryFrom;
use der::{Decodable, Encodable, Message};
use pkcs5::EncryptionScheme;

/// PKCS#8 `EncryptedPrivateKeyInfo`.
///
/// ASN.1 structure containing a PKCS#5 [`EncryptionScheme`] identifier for a
/// password-based symmetric encryption scheme and encrypted private key data.
///
/// ## Encryption algorithm support
///
/// tl;dr: none yet!
///
/// This crate does not (yet) support decrypting/encrypting private key data.
/// However, support for the following may be added in future releases.
/// Please see the following GitHub issue for tracking information:
///
/// <https://github.com/RustCrypto/utils/issues/263>
///
/// ## Schema
/// Structure described in [RFC 5208 Section 6]:
///
/// ```text
/// EncryptedPrivateKeyInfo ::= SEQUENCE {
///   encryptionAlgorithm  EncryptionAlgorithmIdentifier,
///   encryptedData        EncryptedData }
///
/// EncryptionAlgorithmIdentifier ::= AlgorithmIdentifier
///
/// EncryptedData ::= OCTET STRING
/// ```
///
/// [RFC 5208 Section 6]: https://tools.ietf.org/html/rfc5208#section-6
#[cfg_attr(docsrs, doc(cfg(feature = "pkcs5")))]
#[derive(Clone)]
pub struct EncryptedPrivateKeyInfo<'a> {
    /// Algorithm identifier describing a password-based symmetric encryption
    /// scheme used to encrypt the `encrypted_data` field.
    pub encryption_algorithm: EncryptionScheme<'a>,

    /// Private key data
    pub encrypted_data: &'a [u8],
}

impl<'a> TryFrom<&'a [u8]> for EncryptedPrivateKeyInfo<'a> {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self> {
        Ok(Self::from_bytes(bytes)?)
    }
}

impl<'a> TryFrom<der::Any<'a>> for EncryptedPrivateKeyInfo<'a> {
    type Error = der::Error;

    fn try_from(any: der::Any<'a>) -> der::Result<EncryptedPrivateKeyInfo<'a>> {
        any.sequence(|decoder| {
            Ok(Self {
                encryption_algorithm: decoder.decode()?,
                encrypted_data: decoder.octet_string()?.as_bytes(),
            })
        })
    }
}

impl<'a> Message<'a> for EncryptedPrivateKeyInfo<'a> {
    fn fields<F, T>(&self, f: F) -> der::Result<T>
    where
        F: FnOnce(&[&dyn Encodable]) -> der::Result<T>,
    {
        f(&[
            &self.encryption_algorithm,
            &der::OctetString::new(self.encrypted_data)?,
        ])
    }
}