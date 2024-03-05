use fake::{Fake, Faker};
use super::*;

#[test]
fn test_v1() {
    let person_v1: PersonV1 = Faker.fake();
    let serialized = rkyv::to_bytes::<_, 2048>(&person_v1)
        .expect("Failed to serialize");
    let deserialized = rkyv::check_archived_root::<PersonV1>(&serialized)
        .expect("Failed to deserialize");
    assert_eq!(person_v1, *deserialized);
}

#[test]
fn test_swap_name_and_address_ordering() {
    let person_v1: PersonV1 = Faker.fake();
    let serialized = rkyv::to_bytes::<_, 2048>(&person_v1)
        .expect("Failed to serialize");
    let deserialized = rkyv::check_archived_root::<PersonV2>(&serialized)
        .expect("Failed to deserialize");
    
    assert_eq!(person_v1.age, deserialized.age);
    assert_eq!(person_v1.name, deserialized.name);
    assert_eq!(person_v1.address, deserialized.address);
}

#[test]
fn test_swap_name_and_age_ordering() {
    let person_v1: PersonV1 = Faker.fake();
    let serialized = rkyv::to_bytes::<_, 2048>(&person_v1)
        .expect("Failed to serialize");
    let deserialized = rkyv::check_archived_root::<PersonV3>(&serialized)
        .expect("Failed to deserialize");
    
    assert_eq!(person_v1.name, deserialized.name);
    assert_eq!(person_v1.age, deserialized.age);
    assert_eq!(person_v1.address, deserialized.address);
}

#[test]
fn test_rename_age_to_current_age() {
    let person_v1: PersonV1 = Faker.fake();
    let serialized = rkyv::to_bytes::<_, 2048>(&person_v1)
        .expect("Failed to serialize");
    let deserialized = rkyv::check_archived_root::<PersonV4>(&serialized)
        .expect("Failed to deserialize");
    
    assert_eq!(person_v1.name, deserialized.name);
    assert_eq!(person_v1.age, deserialized.current_age);
    assert_eq!(person_v1.address, deserialized.address);
}

#[test]
fn test_remove_age() {
    let person_v1: PersonV1 = Faker.fake();
    let serialized = rkyv::to_bytes::<_, 2048>(&person_v1)
        .expect("Failed to serialize");
    let deserialized = rkyv::check_archived_root::<PersonV5>(&serialized)
        .expect("Failed to deserialize");
    
    assert_eq!(person_v1.name, deserialized.name);
    assert_eq!(person_v1.address, deserialized.address);
}

#[test]
fn test_remove_name() {
    let person_v1: PersonV1 = Faker.fake();
    let serialized = rkyv::to_bytes::<_, 2048>(&person_v1)
        .expect("Failed to serialize");
    let deserialized = rkyv::check_archived_root::<PersonV6>(&serialized)
        .expect("Failed to deserialize");
    
    assert_eq!(person_v1.age, deserialized.age);
    assert_eq!(person_v1.address, deserialized.address);
}

#[test]
fn test_remove_address() {
    let person_v1: PersonV1 = Faker.fake();
    let serialized = rkyv::to_bytes::<_, 2048>(&person_v1)
        .expect("Failed to serialize");
    let deserialized = rkyv::check_archived_root::<PersonV7>(&serialized)
        .expect("Failed to deserialize");
    
    assert_eq!(person_v1.age, deserialized.age);
    assert_eq!(person_v1.name, deserialized.name);
}

#[test]
fn test_add_email() {
    let person_v1: PersonV1 = Faker.fake();
    let serialized = rkyv::to_bytes::<_, 2048>(&person_v1)
        .expect("Failed to serialize");
    let deserialized = rkyv::check_archived_root::<PersonV8>(&serialized)
        .expect("Failed to deserialize");
    
    assert_eq!(person_v1.age, deserialized.age);
    assert_eq!(person_v1.name, deserialized.name);
    assert_eq!(person_v1.address, deserialized.address);
    assert_eq!(String::default(), deserialized.email);
}