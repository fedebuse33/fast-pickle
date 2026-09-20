#[derive(Debug, PartialEq)]
pub enum Value {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Dict(Vec<(Value, Value)>),
    Set(Vec<Value>),
    Object {
        class_name: String,
        state: Vec<(Value, Value)>,
    },
}

pub const MAX_DEPTH: usize = 256;
