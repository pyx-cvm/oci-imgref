//! OCI registry reference
//!
//! # Examples
//!
//! ```rust
//! use oci_imgref::registry::Registry;
//!
//! // Parse a registry with port
//! let registry: Registry = "localhost:5000".parse().unwrap();
//! assert_eq!(registry.host, "localhost");
//! assert_eq!(registry.port.unwrap().get(), 5000);
//!
//! // Custom registry with port
//! let registry: Registry = "registry.example.com:8080".parse().unwrap();
//! assert_eq!(registry.host, "registry.example.com");
//! assert_eq!(registry.port.unwrap().get(), 8080);
//! ```

// https://github.com/distribution/distribution/blob/a4d9db5a884b70be0c96dd6a7a9dbef4f2798c51/reference/reference.go#L8

use core::num::NonZeroU16;

use alloc::string::{String, ToString};

/// a registry parsing error
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, thiserror::Error, displaydoc::Display)]
pub enum Error {
    /// invalid host
    Host,

    /// invalid Port
    Port,
}

/// a container registry
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use oci_imgref::registry::Registry;
/// use std::num::NonZeroU16;
///
/// // Create a registry with custom host and port
/// let registry = Registry {
///     host: "quay.io".into(),
///     port: NonZeroU16::new(5000u16),
/// };
///
/// assert_eq!(registry.to_string(), "quay.io:5000");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Registry {
    /// the host (i.e. `quay.io` in `quay.io:1234`)
    pub host: String,

    /// the port (i.e. `1234` in `quay.io:1234`)
    pub port: Option<NonZeroU16>,
}

impl core::fmt::Display for Registry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.host)?;

        if let Some(port) = self.port {
            write!(f, ":{}", port)?;
        }

        Ok(())
    }
}

impl From<Registry> for String {
    #[inline(always)]
    fn from(registry: Registry) -> Self {
        registry.to_string()
    }
}

impl core::str::FromStr for Registry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (host, port) = s
            .split_once(':')
            .map(|(h, p)| {
                let port: u16 = p.parse().or(Err(Error::Port))?;
                let port = NonZeroU16::new(port).ok_or(Error::Port)?;
                Ok((h, Some(port)))
            })
            .unwrap_or(Ok((s, None)))?;

        if host.is_empty() {
            return Err(Error::Host);
        }

        for segment in host.split('.') {
            for c in segment.chars() {
                if !c.is_ascii_alphanumeric() && c != '-' {
                    return Err(Error::Host);
                }
            }

            if segment.is_empty() || segment.starts_with('-') || segment.ends_with('-') {
                return Err(Error::Host);
            }
        }

        Ok(Self {
            host: host.into(),
            port,
        })
    }
}

impl TryFrom<String> for Registry {
    type Error = Error;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("quay.io", Ok(("quay.io", None)))]
    #[case("docker.io", Ok(("docker.io", None)))]
    #[case("docker.io.", Err(Error::Host))]
    #[case("foo-bar.io", Ok(("foo-bar.io", None)))]
    #[case("foo-bar-.io", Err(Error::Host))]
    #[case("-foo-bar.io", Err(Error::Host))]
    #[case("0zero.io", Ok(("0zero.io", None)))]
    #[case("quay.io:1234", Ok(("quay.io", Some(1234))))]
    #[case("quay.io:0", Err(Error::Port))]
    #[case("quay.io:", Err(Error::Port))]
    #[case(":1234", Err(Error::Host))]
    #[case(":0", Err(Error::Port))]
    fn registry(#[case] input: &str, #[case] result: Result<(&str, Option<u16>), Error>) {
        let result = result.map(|(host, port)| Registry {
            host: host.into(),
            port: port.map(|p| NonZeroU16::new(p)).flatten(),
        });

        assert_eq!(result, input.parse());

        if let Ok(registry) = result {
            assert_eq!(input, registry.to_string());
        }
    }
}
