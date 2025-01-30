//! A no_std-compatible library for parsing and validating OCI (Open
//! Container Initiative) image references.
//!
//! This crate provides types and validation for OCI image references
//! according to the [OCI Distribution Specification](https://github.com/opencontainers/distribution-spec).
//!
//! # Features
//!
//! - `serde`: Adds serialization/deserialization support via serde
//!
//! # Safety
//!
//! This crate is `no_std` compatible and forbids unsafe code. It maintains
//! strict linting rules including no panics/unwraps and complete documentation
//! coverage.
//!
//! # Example
//!
//! ```rust
//! use oci_imgref::image::Image;
//!
//! // Parse a full image reference with tag
//! let image: Image = "docker.io/library/ubuntu:latest".parse().unwrap();
//! assert_eq!(image.to_string(), "docker.io/library/ubuntu:latest");
//!
//! // Parse image with digest
//! let image: Image = "registry.example.com/project/app@sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".parse().unwrap();
//! assert_eq!(image.to_string(), "registry.example.com/project/app@sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
//!
//! // Parse custom registry with port
//! let image: Image = "localhost:5000/my-app:1.0".parse().unwrap();
//! assert_eq!(image.to_string(), "localhost:5000/my-app:1.0");
//! ```
#![no_std]
#![forbid(unsafe_code, clippy::expect_used, clippy::panic)]
#![deny(
    clippy::all,
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_code,
    unreachable_patterns,
    unstable_features,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

extern crate alloc;

#[cfg(test)]
extern crate std;

pub use oci_digest as digest;

pub mod image;
pub mod registry;
pub mod repository;
