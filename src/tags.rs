#[repr(u8)]
pub enum Tag {
    None = 0x00,
    Bool = 0x01,
    Int = 0x02,
    Float = 0x03,
    String = 0x04,
    Bytes = 0x05,
    List = 0x06,
    Tuple = 0x07,
    Dict = 0x08,
    Set = 0x09,
    Object = 0x0A,
}

impl Tag {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Tag::None),
            0x01 => Some(Tag::Bool),
            0x02 => Some(Tag::Int),
            0x03 => Some(Tag::Float),
            0x04 => Some(Tag::String),
            0x05 => Some(Tag::Bytes),
            0x06 => Some(Tag::List),
            0x07 => Some(Tag::Tuple),
            0x08 => Some(Tag::Dict),
            0x09 => Some(Tag::Set),
            0x0A => Some(Tag::Object),
            _ => None,
        }
    }
}
