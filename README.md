# oci-imgref

A no_std-compatible library for parsing and validating OCI (Open
Container Initiative) image references.

This crate provides types and validation for OCI image references
according to the [OCI Distribution Specification](https://github.com/opencontainers/distribution-spec).

## Features

- `serde`: Adds serialization/deserialization support via serde

## Safety

This crate is `no_std` compatible and forbids unsafe code. It maintains
strict linting rules including no panics/unwraps and complete documentation
coverage.

## Example

```rust
use oci_imgref::image::Image;

// Parse a full image reference with tag
let image: Image = "docker.io/library/ubuntu:latest".parse().unwrap();
assert_eq!(image.to_string(), "docker.io/library/ubuntu:latest");

// Parse image with digest
let image: Image = "registry.example.com/project/app@sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".parse().unwrap();
assert_eq!(image.to_string(), "registry.example.com/project/app@sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");

// Parse custom registry with port
let image: Image = "localhost:5000/my-app:1.0".parse().unwrap();
assert_eq!(image.to_string(), "localhost:5000/my-app:1.0");
```
