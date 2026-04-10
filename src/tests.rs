#![cfg(test)]

use crate::RingBuf;
use std::{
    assert_matches,
    io::{Read, Write},
};

#[test]
fn it_works() {
    let mut buf: RingBuf<u8, 5> = Default::default();

    assert_matches!(buf.write(&42u32.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 4] = [0; 4];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u32::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u8.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 1] = [0; 1];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u8::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u32.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 4] = [0; 4];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u32::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u8.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 1] = [0; 1];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u8::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u32.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 4] = [0; 4];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u32::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u8.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 1] = [0; 1];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u8::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u32.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 4] = [0; 4];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u32::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u8.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 1] = [0; 1];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u8::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u32.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 4] = [0; 4];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u32::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u8.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 1] = [0; 1];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u8::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u32.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 4] = [0; 4];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u32::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u8.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 1] = [0; 1];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u8::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u32.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 4] = [0; 4];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u32::from_le_bytes(bytes), 42);

    assert_matches!(buf.write(&42u8.to_le_bytes()), Ok(_));
    let mut bytes: [u8; 1] = [0; 1];
    assert_matches!(buf.read_exact(&mut bytes), Ok(_));
    assert_eq!(u8::from_le_bytes(bytes), 42);
}
