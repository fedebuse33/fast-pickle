use crate::decoder::Decoder;
use crate::encoder::Encoder;
use crate::errors::DecodeError;
use crate::tags::Tag;
use crate::value::Value;

#[test]
fn test_none() {
    let mut encoder = Encoder::new();
    let value = Value::None;
    encoder.encode(&value);

    assert_eq!(encoder.into_bytes()[0], Tag::None as u8)
}

#[test]
fn test_int() {
    let mut encoder = Encoder::new();
    let value = Value::Int(42);
    encoder.encode(&value);

    let bytes = encoder.into_bytes();

    assert_eq!(bytes[0], Tag::Int as u8);
    assert_eq!(bytes.len() - 1, 8);
    assert_eq!(&bytes[1..], &42i64.to_le_bytes());
}

#[test]
fn test_round_trip() {
    let original = Value::Dict(vec![
        (
            Value::String("name".into()),
            Value::String("Federico".into()),
        ),
        (Value::String("age".into()), Value::Int(42)),
        (Value::String("active".into()), Value::Bool(true)),
        (
            Value::String("scores".into()),
            Value::List(vec![
                Value::Float(10.5),
                Value::Float(20.75),
                Value::Int(-3),
            ]),
        ),
    ]);

    let mut encoder = Encoder::new();
    encoder.encode(&original);
    let bytes = encoder.into_bytes();

    let mut decoder = Decoder::new(&bytes);
    let decoded = decoder.decode().unwrap();

    assert_eq!(decoded, original);
}

#[test]
fn test_empty_input() {
    let mut decoder = Decoder::new(&[]);
    assert_eq!(decoder.decode(), Err(DecodeError::UnexpectedEof));
}

#[test]
fn test_truncated_string() {
    // Tag::String, poi dichiara lunghezza 10, ma fornisce solo 2 byte di contenuto
    let mut bytes = vec![Tag::String as u8];
    bytes.extend_from_slice(&10u64.to_le_bytes());
    bytes.extend_from_slice(b"ab"); // solo 2 byte invece di 10

    let mut decoder = Decoder::new(&bytes);
    assert_eq!(decoder.decode(), Err(DecodeError::UnexpectedEof));
}

#[test]
fn test_huge_declared_length_list() {
    // Tag::List con lunghezza dichiarata enorme, ma zero byte a seguire
    let mut bytes = vec![Tag::List as u8];
    bytes.extend_from_slice(&u64::MAX.to_le_bytes());

    let mut decoder = Decoder::new(&bytes);
    assert_eq!(decoder.decode(), Err(DecodeError::UnexpectedEof));
}

#[test]
fn test_recursion_limit() {
    // Costruisce 300 Tag::List annidate, ognuna con len=1 e un solo elemento
    // (che è la lista successiva), per superare MAX_DEPTH (256)
    let mut bytes = Vec::new();
    for _ in 0..300 {
        bytes.push(Tag::List as u8);
        bytes.extend_from_slice(&1u64.to_le_bytes()); // len = 1
    }
    bytes.push(Tag::None as u8); // elemento più interno

    let mut decoder = Decoder::new(&bytes);
    assert_eq!(decoder.decode(), Err(DecodeError::RecursionLimitExceeded));
}

#[test]
fn test_invalid_tag() {
    let bytes = vec![0xFFu8]; // nessun tag corrisponde a 0xFF
    let mut decoder = Decoder::new(&bytes);
    assert_eq!(decoder.decode(), Err(DecodeError::InvalidTag(0xFF)));
}
