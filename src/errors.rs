#[derive(Debug, PartialEq)]
pub enum DecodeError {
    UnexpectedEof,
    InvalidTag(u8),
    InvalidBool(u8),
    InvalidUtf8,
    RecursionLimitExceeded,
}
