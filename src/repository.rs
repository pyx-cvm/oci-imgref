//! OCI repository reference
//!
//! # Examples
//!
//! ```
//! use oci_imgref::repository::Repository;
//!
//! // Parse a full repository reference
//! let repo: Repository = "quay.io/organization/container".parse()?;
//! assert_eq!(repo.registry.unwrap().host, "quay.io");
//! assert_eq!(repo.organization.unwrap(), "organization");
//! assert_eq!(repo.container, "container");
//!
//! // The registry (docker.io) is optional
//! let repo: Repository = "library/ubuntu".parse()?;
//! assert!(repo.registry.is_none());
//! # Ok::<(), oci_imgref::repository::Error>(())
//! ```

use alloc::string::{String, ToString};
use core::hash::Hash;
use core::{fmt::Display, str::FromStr};

use crate::registry::Registry;

/// an image parsing error
#[derive(Copy, Clone, Debug, PartialEq, Eq, thiserror::Error, displaydoc::Display)]
pub enum Error {
    /// invalid registry: {0}
    Registry(#[from] super::registry::Error),

    /// invalid organization
    Organization,

    /// invalid container
    Container,
}

/// a container repository reference
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Repository {
    /// the registry (i.e. `quay.io:1234` in `quay.io:1234/foo/bar:latest`)
    pub registry: Option<Registry>,

    /// the organization (i.e. `foo` in `foo/bar:latest`)
    pub organization: Option<String>,

    /// the container (i.e. `bar` in `foo/bar:latest`)
    pub container: String,
}

impl Display for Repository {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(registry) = &self.registry {
            write!(f, "{}/", registry)?;
        }

        if let Some(org) = &self.organization {
            write!(f, "{}/", &org)?;
        }

        write!(f, "{}", self.container)
    }
}

impl From<Repository> for String {
    #[inline]
    fn from(value: Repository) -> Self {
        value.to_string()
    }
}

impl FromStr for Repository {
    type Err = Error;

    fn from_str(from: &str) -> Result<Self, Self::Err> {
        match from.rsplit_once('/') {
            // `ubuntu`
            None => Ok(Self {
                registry: None,
                organization: None,
                container: path(from, Error::Container)?.into(),
            }),

            Some((pfx, con)) => match pfx.rsplit_once('/') {
                None => {
                    // `quay.io/ubuntu`
                    if pfx == "localhost" || pfx.contains('.') || pfx.contains(':') {
                        Ok(Self {
                            registry: Some(pfx.parse()?),
                            organization: None,
                            container: path(con, Error::Container)?,
                        })

                    // `library/ubuntu`
                    } else {
                        Ok(Self {
                            registry: None,
                            organization: Some(path(pfx, Error::Organization)?),
                            container: path(con, Error::Container)?,
                        })
                    }
                }

                // `docker.io/library/ubuntu`
                Some((reg, org)) => Ok(Self {
                    registry: Some(reg.parse()?),
                    organization: Some(path(org, Error::Organization)?),
                    container: path(con, Error::Container)?,
                }),
            },
        }
    }
}

impl TryFrom<String> for Repository {
    type Error = Error;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[inline(always)]
fn path(from: &str, err: Error) -> Result<String, Error> {
    for c in from.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.' | '-' => (),
            _ => return Err(err),
        }
    }

    match (from.chars().next(), from.chars().rev().next()) {
        (Some(l), Some(r)) if l.is_ascii_alphanumeric() && r.is_ascii_alphanumeric() => (),
        _ => return Err(err),
    }

    Ok(from.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_validation() {
        assert!(path("valid", Error::Container).is_ok());
        assert!(path("valid-name", Error::Container).is_ok());
        assert!(path("valid_name.123", Error::Container).is_ok());
        assert!(path("UPPERCASE", Error::Container).is_ok());
        assert!(path("", Error::Container).is_err()); // Empty string
        assert!(path("invalid!", Error::Container).is_err()); // Invalid symbol
        assert!(path("invalid/name", Error::Container).is_err()); // Invalid slash
        assert!(path("invalid space", Error::Container).is_err()); // Invalid space
        assert!(path("-invalid", Error::Container).is_err()); // Bad start char
        assert!(path("invalid-", Error::Container).is_err()); // Bad end char
        assert!(path("bad@chars", Error::Container).is_err()); // Invalid chars
        assert!(path(".invalid", Error::Container).is_err()); // Bad start with dot
        assert!(path("_invalid", Error::Container).is_err()); // Bad start with underscore
    }
}
