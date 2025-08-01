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

#[cfg(test)]
#[expect(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_parsing() {
        let toml = r#"[data]
                            name = "great-name"
                            version = "0.1.0"
                            client-checksum = "1234567890"
                            authors = ["me", "you"]
                            description = "test"
                            license = "MIT OR Apache-2.0"
                            license-file = "LICENSE.md"
                            "#;

        let toml: ModToml = toml::from_str(toml).unwrap();

        assert_eq!(toml.data.name, "great-name");
        assert_eq!(toml.data.version, "0.1.0");
        assert_eq!(toml.data.client_checksum, "1234567890");
        assert!(
            toml.data
                .authors
                .is_some_and(|auths| auths == vec!["me", "you"])
        );
        assert!(toml.data.description.is_some_and(|des| des == "test"));
        assert!(
            toml.data
                .license
                .is_some_and(|lic| lic == "MIT OR Apache-2.0")
        );
        assert!(
            toml.data
                .license_file
                .is_some_and(|lic| lic == "LICENSE.md")
        );
    }

    #[test]
    fn test_optionals() {
        let toml = r#"[data]
                            name = "minimal"
                            version = "0.1.0"
                            client-checksum = "1234567890"
                            "#;

        let toml: ModToml = toml::from_str(toml).unwrap();

        assert!(toml.data.authors.is_none());
        assert!(toml.data.description.is_none());
        assert!(toml.data.license.is_none());
        assert!(toml.data.license_file.is_none());
    }

    #[test]
    fn test_dependencies() {
        let toml = r#"[data]
        name = "minimal"
        version = "0.1.0"
        client-checksum = "1234567890"

        [dependencies]
        dep1 = { version = "3.14.1", optional = true, checksum = "0987654321" }
        "#;

        let toml: ModToml = toml::from_str(toml).unwrap();

        assert!(toml.dependencies.is_some());

        let deps = toml.dependencies.unwrap();

        assert!(deps.contains_key("dep1"));
        assert_eq!(deps["dep1"].version, "3.14.1");
        assert!(deps["dep1"].optional);
        assert_eq!(deps["dep1"].checksum, "0987654321");
    }
}
