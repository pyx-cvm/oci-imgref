use oci_imgref::digest::Error as DigestError;
use oci_imgref::image::{Error, Image};
use oci_imgref::registry::Error as RegError;
use oci_imgref::repository::Error as RepoError;

#[rstest::rstest]
#[case(
    "quay.io:443/foo/bar:latest@sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "quay.io",
    443,
    "foo",
    "bar",
    "latest",
    "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
)]
#[case(
    "quay.io:443/foo/bar:latest",
    "quay.io",
    443,
    "foo",
    "bar",
    "latest",
    None
)]
#[case("quay.io:443/foo/bar", "quay.io", 443, "foo", "bar", None, None)]
#[case("quay.io/foo/bar:x", "quay.io", None, "foo", "bar", "x", None)]
#[case("quay.io/foo/bar", "quay.io", None, "foo", "bar", None, None)]
#[case("quay.io/foo", "quay.io", None, None, "foo", None, None)]
#[case("localhost/foo", "localhost", None, None, "foo", None, None)]
#[case("foo/bar", None, None, "foo", "bar", None, None)]
#[case("foo", None, None, None, "foo", None, None)]
#[case("foo:latest", None, None, None, "foo", "latest", None)]
#[case(
    "foo:latest@sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    None,
    None,
    None,
    "foo",
    "latest",
    "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
)]
#[case(
    "foo@sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    None,
    None,
    None,
    "foo",
    None,
    "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
)]
fn image(
    #[case] input: &'static str,
    #[case] host: impl Into<Option<&'static str>>,
    #[case] port: impl Into<Option<u16>>,
    #[case] org: impl Into<Option<&'static str>>,
    #[case] con: &'static str,
    #[case] tag: impl Into<Option<&'static str>>,
    #[case] digest: impl Into<Option<&'static str>>,
) {
    let image: Image = input.parse().unwrap();

    match host.into() {
        None => assert!(image.repository.registry.is_none()),
        Some(host) => {
            let reg = image.repository.registry.unwrap();
            assert_eq!(reg.host, host);

            match port.into() {
                None => assert!(reg.port.is_none()),
                Some(port) => assert_eq!(port, reg.port.unwrap().into()),
            }
        }
    }

    match org.into() {
        None => assert!(image.repository.organization.is_none()),
        Some(org) => assert_eq!(image.repository.organization.unwrap(), org),
    }

    assert_eq!(image.repository.container, con);

    match tag.into() {
        None => assert!(image.tag.is_none()),
        Some(tag) => assert_eq!(image.tag.unwrap(), tag),
    }

    match digest.into() {
        None => assert!(image.digest.is_none()),
        Some(digest) => assert_eq!(image.digest.unwrap().to_string(), digest),
    }
}

#[rstest::rstest]
#[case(
    "foo@sha256:X3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    Error::Digest(DigestError::Character)
)]
#[case(
    "foo@sha257:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    Error::Digest(DigestError::Algorithm)
)]
#[case("foo@sha256:e3", Error::Digest(DigestError::Length))]
#[case("foo@", Error::Digest(DigestError::Length))]
#[case("foo:-", Error::Tag)]
#[case("foo-", Error::Repository(RepoError::Container))]
#[case("foo-/bar", Error::Repository(RepoError::Organization))]
#[case("quay.io/foo-/bar", Error::Repository(RepoError::Organization))]
#[case("quay.io-/foo", Error::Repository(RepoError::Registry(RegError::Host)))]
#[case(
    "quay.io-/foo/bar",
    Error::Repository(RepoError::Registry(RegError::Host))
)]
#[case(
    "quay.io:/foo/bar",
    Error::Repository(RepoError::Registry(RegError::Port))
)]
#[case(
    "quay.io:abcd/foo/bar",
    Error::Repository(RepoError::Registry(RegError::Port))
)]
fn failure(#[case] input: &'static str, #[case] error: Error) {
    assert_eq!(input.parse::<Image>().unwrap_err(), error);
}
