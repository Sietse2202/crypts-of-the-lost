// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Cert
//! This module has some helper functions for working with certificates

use quinn::rustls::pki_types::pem::PemObject;
use quinn::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pki_types::pem;

pub(crate) type Result<T> = std::result::Result<T, pem::Error>;

/// A helper struct that just cleans the function signatures up.
#[derive(Debug, PartialEq, Eq)]
pub struct Certs {
    certs: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
}

impl Certs {
    /// Creates a new instance
    #[inline]
    #[must_use]
    pub const fn new(certs: Vec<CertificateDer<'static>>, key: PrivateKeyDer<'static>) -> Self {
        Self { certs, key }
    }

    /// Gets the certificates from the instance
    #[inline]
    #[must_use]
    pub const fn certs(&self) -> &Vec<CertificateDer<'static>> {
        &self.certs
    }

    /// Gets the key from the instance
    #[inline]
    #[must_use]
    pub const fn key(&self) -> &PrivateKeyDer<'static> {
        &self.key
    }

    /// Get the certificates from disk, this requires you to use something like
    /// [certbot](https://certbot.eff.org/)
    ///
    /// # Errors
    /// This function errors due to any of the following
    /// - IO error
    /// - File not found
    /// - Parsing error in the files
    #[expect(clippy::unwrap_used)]
    #[expect(clippy::missing_panics_doc)]
    pub fn read_from_file() -> Result<Self> {
        let mut certs = CertificateDer::pem_file_iter("./fullchain.pem")?;
        if certs.any(|cert| cert.is_err()) {
            // Panics: we know there is an error
            certs.find(Result::is_err).unwrap()?;
        }

        // Panics: we just confirmed that all certs are not Err.
        let certs = certs.map(|cert| cert.unwrap()).collect();

        let key = PrivateKeyDer::from_pem_file("./privkey.pem")?;

        Ok(Self::new(certs, key))
    }
}

impl From<(Vec<CertificateDer<'static>>, PrivateKeyDer<'static>)> for Certs {
    fn from(value: (Vec<CertificateDer<'static>>, PrivateKeyDer<'static>)) -> Self {
        Self::new(value.0, value.1)
    }
}
