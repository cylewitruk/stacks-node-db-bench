use fake::Dummy;
use rkyv::{with::AsBox, Archive, Deserialize, Serialize};

pub mod rkyv_tests;

// This wrapper type serializes the contained value out-of-line so that newer
// versions can be viewed as the older version.
//
// In a complete message format, sending a version number along with the buffer
// would allow clients to reject incompatible messages before validating the
// buffer.
#[derive(Archive, Deserialize, Serialize)]
#[repr(transparent)]
#[archive(check_bytes)]
#[archive_attr(repr(transparent))]
struct Versioned<T>(#[with(AsBox)] pub T);

/// Original struct
#[derive(Debug, Archive, Serialize, Deserialize, Dummy, PartialEq)]
#[archive(check_bytes, compare(PartialEq))]
#[archive_attr(check_bytes, derive(Debug, PartialEq))]
pub struct PersonV1 {
    pub age: u32,
    pub name: String,
    pub address: String,
}

/// Swap order of name and address
#[derive(Debug, Archive, Serialize, Deserialize, Dummy, PartialEq)]
#[archive(check_bytes, compare(PartialEq))]
#[archive_attr(check_bytes, derive(Debug, PartialEq))]
pub struct PersonV2 {
    pub age: u32,
    pub address: String,
    pub name: String,
}

/// Swap order of name and age
#[derive(Debug, Archive, Serialize, Deserialize, Dummy, PartialEq)]
#[archive(check_bytes, compare(PartialEq))]
#[archive_attr(check_bytes, derive(Debug, PartialEq))]
pub struct PersonV3 {
    pub name: String,
    pub age: u32,
    pub address: String,
}

/// Rename a single column (age -> current_age)
#[derive(Debug, Archive, Serialize, Deserialize, Dummy, PartialEq)]
#[archive(check_bytes, compare(PartialEq))]
#[archive_attr(check_bytes, derive(Debug, PartialEq))]
pub struct PersonV4 {
    pub current_age: u32,
    pub name: String,
    pub address: String,
}

/// Remove any field (age)
#[derive(Debug, Archive, Serialize, Deserialize, Dummy, PartialEq)]
#[archive(check_bytes, compare(PartialEq))]
#[archive_attr(check_bytes, derive(Debug, PartialEq))]
pub struct PersonV5 {
    pub name: String,
    pub address: String,
}

/// Remove any field (name)
#[derive(Debug, Archive, Serialize, Deserialize, Dummy, PartialEq)]
#[archive(check_bytes, compare(PartialEq))]
#[archive_attr(check_bytes, derive(Debug, PartialEq))]
pub struct PersonV6 {
    pub age: u32,
    pub address: String,
}

/// Remove any field (address)
#[derive(Debug, Archive, Serialize, Deserialize, Dummy, PartialEq)]
#[archive(check_bytes, compare(PartialEq))]
#[archive_attr(check_bytes, derive(Debug, PartialEq))]
pub struct PersonV7 {
    pub age: u32,
    pub name: String,
}

/// Add a new field (email)
#[derive(Debug, Archive, Serialize, Deserialize, Dummy, PartialEq)]
#[archive(check_bytes, compare(PartialEq))]
#[archive_attr(check_bytes, derive(Debug, PartialEq))]
pub struct PersonV8 {
    pub age: u32,
    pub name: String,
    pub address: String,
    pub email: String,
}