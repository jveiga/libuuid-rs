#![feature(libc)]
extern crate libc;
use libc::{c_uchar, c_int};

#[link(name="uuid")]
extern "C" {
    pub fn uuid_compare(uu1: &Uuid, uu2: &Uuid) -> c_int;
    pub fn uuid_generate(out: &mut Uuid);
    pub fn uuid_is_null(uu: &Uuid) -> c_int;
    pub fn uuid_clear(uu: &mut Uuid);
    pub fn uuid_copy(dst: &mut Uuid, src: &Uuid);
}

#[repr(C)] //TODO check up on this
#[derive(Debug, PartialEq, Clone)]
pub struct Uuid([c_uchar; 16]);

impl Uuid {
    pub fn new() -> Uuid {
        Uuid([0u8; 16])
    }
    /// Generate a new uuid
    ///```
    /// assert!(Uuid::new() != Uuid::generate());
    ///```
    pub fn generate() -> Option<Uuid> {
        let mut x = Uuid([0u8; 16]);
        unsafe {
            uuid_generate(&mut x);
        }
        match Uuid::is_null(&x) {
            false =>  Some(x),
            _ => None
        }
    }

    /// Compares uuids
    ///```
    /// assert!(Uuid::compare(Uuid::generate() != Uuid::generate()));
    ///```
    pub fn compare(one: &Uuid, other: &Uuid) -> bool {
        one == other
    }

    /// Clear a uuid
    ///```
    /// let clear_uuid = Uuid::new();
    /// let mut uuid_to_clear = Uuid::generate().unwrap();
    /// Uuid::clear(&mut uuid_to_clear);
    /// debug_assert_eq!(clear_uuid, uuid_to_clear);
    ///```
    pub fn clear(v: &mut Uuid) {
        unsafe {
            uuid_clear(v);
        }
    }

    /// Checks if a uuid is clean
    ///```
    /// let clear_uuid = Uuid::new();
    /// let mut uuid_to_clear = Uuid::generate().unwrap();
    /// Uuid::clear(&mut uuid_to_clear);
    /// debug_assert_eq!(clear_uuid, uuid_to_clear);
    ///```
    pub fn is_null(&self) -> bool{
        unsafe{
            uuid_is_null(self) == 1
        }
    }

    /// Copies from one uuid to another
    /// ```
    /// let original = Uuid::generate();
    /// let mut copy = Uuid::new();
    /// Uuid::copy(&original, &mut copy);
    /// assert_eq!(original, copy);
    /// ```
    pub fn copy(src: &Uuid, dst: &mut Uuid) {
        *dst = src.clone();
    }

}

#[test]
fn test_generate() {
    let clear_uuid = Uuid([0u8; 16]);
    let generated_uuid = Uuid::generate().unwrap();
    assert!(clear_uuid != generated_uuid);
}

#[test]
fn test_clear() {
    let clear_uuid = Uuid([0u8; 16]);
    let mut uuid_to_clear = Uuid::generate().unwrap();
    Uuid::clear(&mut uuid_to_clear);
    debug_assert_eq!(clear_uuid, uuid_to_clear);
}

#[test]
fn test_compare() {
    let test_generate = || {
        let generated_uuid = Uuid::generate().unwrap();
        assert!(Uuid::compare(&generated_uuid, &generated_uuid));
    };
    let test_empty = || {
        let empty = Uuid([0u8; 16]);
        assert!(Uuid::compare(&empty, &empty));
    };
    test_generate();
    test_empty();
}

#[test]
fn test_is_null() {
    let is_null_with_clear = ||{
        let mut generated_uuid = Uuid::generate().unwrap();
        Uuid::clear(&mut generated_uuid);
        debug_assert!(Uuid::is_null(&generated_uuid));
    };
    let is_null_with_empty = ||{
        let mut generated_uuid = Uuid::generate().unwrap();
        Uuid::clear(&mut generated_uuid);
        debug_assert_eq!(Uuid([0u8; 16]), generated_uuid);
    };
    is_null_with_empty();
    is_null_with_clear();
}

#[test]
fn test_copy() {
    let src = Uuid::generate().unwrap();
    let mut dst = Uuid::new();
    Uuid::copy(&src, &mut dst);
    debug_assert_eq!(src, dst);
}
