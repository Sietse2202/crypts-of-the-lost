// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ModToml {
    pub data: Data,
    pub dependencies: Option<HashMap<String, DependencyData>>,
}

#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    Default,
)]
#[serde(rename_all = "kebab-case")]
pub struct Data {
    pub name: String,
    pub version: String,
    pub client_checksum: String,
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub readme: Option<String>,
}

#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Hash,
    Default,
)]
#[serde(rename_all = "kebab-case")]
pub struct DependencyData {
    pub version: String,
    pub optional: bool,
    pub checksum: String,
}
