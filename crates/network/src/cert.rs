// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Cert
//! This module has some helper functions for working with certificates

use std::path::Path;

use quinn::rustls::pki_types::pem::PemObject;
use quinn::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pki_types::pem;

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

    /// Loads TLS certificates from disk.
    ///
    /// You can generate these using tools like [Certbot](https://certbot.eff/.org/)
    /// or create them manually (self-signed).
    ///
    /// Note: Self-signed certificates do **not** protect against
    /// man-in-the-middle (MITM) attacks unless the client explicitly trusts them.
    ///
    /// # Errors
    /// This function errors due to any of the following
    /// - IO error
    /// - File not found
    /// - Parsing error in the files
    pub fn read_from_file<P: AsRef<Path>>(cert_path: P, key_path: P) -> Result<Self, pem::Error> {
        let certs_result: Result<Vec<_>, _> = CertificateDer::pem_file_iter(cert_path)?.collect();

        let certs = certs_result?;

        let key = PrivateKeyDer::from_pem_file(key_path)?;

        Ok(Self::new(certs, key))
    }
}

impl From<(Vec<CertificateDer<'static>>, PrivateKeyDer<'static>)> for Certs {
    fn from(value: (Vec<CertificateDer<'static>>, PrivateKeyDer<'static>)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_success() {
        let cert = Certs::read_from_file("./certs.pem", "./key.pem");
        assert!(cert.is_ok());
    }
}
