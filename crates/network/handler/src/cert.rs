// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Cert
//! This module has some helper functions for working with certificates

use quinn::ServerConfig;
use quinn::rustls::pki_types::pem::PemObject;
use quinn::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pki_types::pem;
use std::path::Path;

use crate::error::CertsError;

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
    const fn new(certs: Vec<CertificateDer<'static>>, key: PrivateKeyDer<'static>) -> Self {
        Self { certs, key }
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
    /// Returns an `pem::Error` if the path is wrong or it can't read or parse the
    /// certificates and keys
    pub fn read_from_file<P, Q>(cert_path: P, key_path: Q) -> Result<Self, pem::Error>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let certs_result: Result<Vec<_>, _> = CertificateDer::pem_file_iter(cert_path)?.collect();

        let certs = certs_result?;

        let key = PrivateKeyDer::from_pem_file(key_path)?;

        Ok(Self::new(certs, key))
    }

    /// Creates a [`ServerConfig`] to be used by the `NetworkHandler`
    ///
    /// # Errors
    /// Returns an `CertsError` when `ServerConfig` creation fails.
    pub fn create_server_config(self) -> Result<ServerConfig, CertsError> {
        Ok(ServerConfig::with_single_cert(self.certs, self.key)?)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore = "CI will complain if this fails, so I made it be ignored. Please test it locally."]
    fn read_success() {
        // the files need to be in the root of this repository
        let cert = Certs::read_from_file("../../certs.pem", "../../key.pem");
        assert!(cert.is_ok());
    }
}
