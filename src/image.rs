//! OCI image reference
//!
//! # Examples
//!
//! ```rust
//! use oci_imgref::image::Image;
//!
//! // Parse an image reference with a tag
//! let image = "docker.io/library/ubuntu:latest".parse::<Image>().unwrap();
//! assert_eq!(image.repository.to_string(), "docker.io/library/ubuntu");
//! assert_eq!(image.tag.unwrap(), "latest");
//! assert!(image.digest.is_none());
//!
//! // Parse an image reference with a digest
//! let image = "docker.io/library/ubuntu@sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".parse::<Image>().unwrap();
//! assert_eq!(image.repository.to_string(), "docker.io/library/ubuntu");
//! assert!(image.tag.is_none());
//! assert_eq!(image.digest.unwrap().to_string(), "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
//! ```

use alloc::string::{String, ToString};
use core::{fmt::Display, hash::Hash, str::FromStr};

use crate::{digest::Digest, repository::Repository};

/// an image parsing error
#[derive(Copy, Clone, Debug, PartialEq, Eq, thiserror::Error, displaydoc::Display)]
pub enum Error {
    /// invalid registry: {0}
    Repository(#[from] crate::repository::Error),

    /// invalid tag
    Tag,

    /// invalid digest: {0}
    Digest(#[from] crate::digest::Error),
}

/// a container image reference
///
/// # Fields
///
/// * `repository` - The repository component including registry and path (e.g. `docker.io/library/ubuntu`)
/// * `tag` - Optional tag component (e.g. `latest` in `ubuntu:latest`)
/// * `digest` - Optional content-addressable digest (e.g. `sha256:abc123...` in `ubuntu@sha256:abc123...`)
///
/// # Examples
///
/// ```rust
/// use oci_imgref::image::Image;
///
/// // Create an image reference with a tag
/// let image = Image {
///     repository: "docker.io/library/ubuntu".parse().unwrap(),
///     tag: Some("latest".into()),
///     digest: None,
/// };
///
/// // Parse from a string
/// let image: Image = "docker.io/library/ubuntu:latest".parse().unwrap();
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Image {
    /// the repository (i.e. `quay.io:1234/foo/bar` in `quay.io:1234/foo/bar:latest`)
    pub repository: Repository,

    /// the tag (i.e. `latest` in `foo/bar:latest`)
    pub tag: Option<String>,

    /// the digest (i.e. `sha256:deadbeef` in `foo/bar@sha256:deadbeef`)
    pub digest: Option<Digest>,
}

impl From<Image> for String {
    fn from(value: Image) -> Self {
        value.to_string()
    }
}

impl Display for Image {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.repository)?;

        if let Some(tag) = &self.tag {
            write!(f, ":{}", tag)?;
        }

        if let Some(digest) = &self.digest {
            write!(f, "@{}", digest)?;
        }

        Ok(())
    }
}

impl FromStr for Image {
    type Err = Error;

    fn from_str(mut from: &str) -> Result<Self, Self::Err> {
        let mut digest = None;
        if let Some((prefix, dig)) = from.rsplit_once('@') {
            digest = Some(dig.parse()?);
            from = prefix;
        }

        let mut tag = None;
        if let Some((prefix, lbl)) = from.rsplit_once(':') {
            if !lbl.contains('/') {
                if lbl.is_empty() {
                    return Err(Error::Tag);
                }

                for (i, c) in lbl.chars().enumerate() {
                    match (i, c) {
                        (i, _) if i > 127 => return Err(Error::Tag),
                        (_, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') => (),
                        (0, _) => return Err(Error::Tag),
                        (_, '.' | '-') => (),
                        _ => return Err(Error::Tag),
                    }
                }

                tag = Some(lbl.to_string());
                from = prefix;
            }
        }

        Ok(Self {
            repository: Repository::from_str(from)?,
            tag,
            digest,
        })
    }
}

impl TryFrom<String> for Image {
    type Error = Error;

    #[inline(always)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
