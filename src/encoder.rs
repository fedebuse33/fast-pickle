use crate::tags::Tag;
use crate::value::Value;
pub struct Encoder {
    buffer: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }
    pub fn encode(&mut self, value: &Value) {
        match value {
            Value::None => self.buffer.push(Tag::None as u8),
            Value::Bool(b) => {
                self.buffer.push(Tag::Bool as u8);
                self.buffer.push(*b as u8);
            }
            Value::Int(i) => {
                self.buffer.push(Tag::Int as u8);
                self.buffer.extend_from_slice(&i.to_le_bytes());
            }
            Value::Float(f) => {
                self.buffer.push(Tag::Float as u8);
                self.buffer.extend_from_slice(&f.to_le_bytes());
            }
            Value::String(s) => {
                self.buffer.push(Tag::String as u8);
                self.buffer
                    .extend_from_slice(&(s.len() as u64).to_le_bytes());
                self.buffer.extend_from_slice(s.as_bytes());
            }
            Value::Bytes(b) => {
                self.buffer.push(Tag::Bytes as u8);
                self.buffer
                    .extend_from_slice(&(b.len() as u64).to_le_bytes());
                self.buffer.extend_from_slice(b);
            }
            Value::List(l) => {
                self.buffer.push(Tag::List as u8);
                self.buffer
                    .extend_from_slice(&(l.len() as u64).to_le_bytes());
                for element in l {
                    self.encode(element);
                }
            }
            Value::Tuple(t) => {
                self.buffer.push(Tag::Tuple as u8);
                self.buffer
                    .extend_from_slice(&(t.len() as u64).to_le_bytes());
                for element in t {
                    self.encode(element);
                }
            }
            Value::Set(s) => {
                self.buffer.push(Tag::Set as u8);
                self.buffer
                    .extend_from_slice(&(s.len() as u64).to_le_bytes());
                for element in s {
                    self.encode(element);
                }
            }
            Value::Dict(d) => {
                self.buffer.push(Tag::Dict as u8);
                self.buffer
                    .extend_from_slice(&(d.len() as u64).to_le_bytes());
                for (key, value) in d {
                    self.encode(key);
                    self.encode(value);
                }
            }
            Value::Object { class_name, state } => {
                self.buffer.push(Tag::Object as u8);
                self.buffer
                    .extend_from_slice(&(class_name.len() as u64).to_le_bytes());
                self.buffer.extend_from_slice(class_name.as_bytes());
                self.buffer
                    .extend_from_slice(&(state.len() as u64).to_le_bytes());
                for (key, value) in state {
                    self.encode(key);
                    self.encode(value);
                }
            }
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }
}
