use crate::errors::DecodeError;
use crate::tags::Tag;
use crate::value::{MAX_DEPTH, Value};

pub struct Decoder<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    pub fn decode(&mut self) -> Result<Value, DecodeError> {
        self.decode_with_depth(0)
    }

    fn decode_with_depth(&mut self, depth: usize) -> Result<Value, DecodeError> {
        if depth > MAX_DEPTH {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        let byte = self.read_u8()?;
        let tag = Tag::from_u8(byte).ok_or(DecodeError::InvalidTag(byte))?;
        match tag {
            Tag::Bool => {
                let value = self.read_u8()?;
                match value {
                    0 => Ok(Value::Bool(false)),
                    1 => Ok(Value::Bool(true)),
                    other => Err(DecodeError::InvalidBool(other)),
                }
            }
            Tag::Bytes => {
                let len = self.read_u64()? as usize;
                let bytes = self.read_bytes(len)?;
                Ok(Value::Bytes(bytes.to_vec()))
            }
            Tag::Dict => {
                let len = self.read_u64()? as usize;
                if len > self.remaining_len() {
                    return Err(DecodeError::UnexpectedEof);
                }
                let mut buffer = Vec::with_capacity(len);
                for _ in 0..len {
                    let key = self.decode_with_depth(depth + 1)?;
                    let value = self.decode_with_depth(depth + 1)?;
                    buffer.push((key, value));
                }
                Ok(Value::Dict(buffer))
            }

            Tag::Float => {
                let bits = self.read_u64()?;
                let value = f64::from_le_bytes(bits.to_le_bytes());
                Ok(Value::Float(value))
            }

            Tag::Int => {
                let value = self.read_u64()? as i64;
                Ok(Value::Int(value))
            }

            Tag::List => {
                let len = self.read_u64()? as usize;
                if len > self.remaining_len() {
                    return Err(DecodeError::UnexpectedEof);
                }
                let mut buffer = Vec::with_capacity(len);
                for _ in 0..len {
                    let value = self.decode_with_depth(depth + 1)?;
                    buffer.push(value);
                }
                Ok(Value::List(buffer))
            }

            Tag::None => Ok(Value::None),
            Tag::Set => {
                let len = self.read_u64()? as usize;
                if len > self.remaining_len() {
                    return Err(DecodeError::UnexpectedEof);
                }
                let mut buffer = Vec::with_capacity(len);
                for _ in 0..len {
                    let value = self.decode_with_depth(depth + 1)?;
                    buffer.push(value);
                }
                Ok(Value::Set(buffer))
            }
            Tag::String => {
                let len = self.read_u64()? as usize;
                let bytes = self.read_bytes(len)?;

                String::from_utf8(bytes.to_vec())
                    .map(Value::String)
                    .map_err(|_| DecodeError::InvalidUtf8)
            }

            Tag::Tuple => {
                let len = self.read_u64()? as usize;
                if len > self.remaining_len() {
                    return Err(DecodeError::UnexpectedEof);
                }
                let mut buffer = Vec::with_capacity(len);
                for _ in 0..len {
                    let value = self.decode_with_depth(depth + 1)?;
                    buffer.push(value);
                }
                Ok(Value::Tuple(buffer))
            }

            Tag::Object => {
                let class_name_len = self.read_u64()? as usize;
                if class_name_len > self.remaining_len() {
                    return Err(DecodeError::UnexpectedEof);
                }
                let class_name_bytes = self.read_bytes(class_name_len)?;
                let class_name = String::from_utf8(class_name_bytes.to_vec())
                    .map_err(|_| DecodeError::InvalidUtf8)?;
                let len = self.read_u64()? as usize;
                if len > self.remaining_len() {
                    return Err(DecodeError::UnexpectedEof);
                }
                let mut buffer = Vec::with_capacity(len);
                for _ in 0..len {
                    let key = self.decode_with_depth(depth + 1)?;
                    let value = self.decode_with_depth(depth + 1)?;
                    buffer.push((key, value));
                }
                Ok(Value::Object {
                    class_name,
                    state: buffer,
                })
            }
        }
    }

    fn read_u8(&mut self) -> Result<u8, DecodeError> {
        let byte = self.data.get(self.position);
        match byte {
            None => Err(DecodeError::UnexpectedEof),
            Some(b) => {
                self.position += 1;
                Ok(*b)
            }
        }
    }

    fn read_u64(&mut self) -> Result<u64, DecodeError> {
        let end = self
            .position
            .checked_add(8)
            .ok_or(DecodeError::UnexpectedEof)?;

        let bytes = self
            .data
            .get(self.position..end)
            .ok_or(DecodeError::UnexpectedEof)?;

        self.position = end;

        let bytes: [u8; 8] = bytes.try_into().unwrap();

        Ok(u64::from_le_bytes(bytes))
    }
    fn read_bytes(&mut self, len: usize) -> Result<&[u8], DecodeError> {
        let end = self
            .position
            .checked_add(len)
            .ok_or(DecodeError::UnexpectedEof)?;

        let bytes = self
            .data
            .get(self.position..end)
            .ok_or(DecodeError::UnexpectedEof)?;

        self.position = end;

        Ok(bytes)
    }

    fn remaining_len(&self) -> usize {
        self.data.len() - self.position
    }
}
