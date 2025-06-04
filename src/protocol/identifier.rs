use std::fmt::Display;

use bytes::BytesMut;

use crate::{
    network::BufferReader,
    protocol::{Readable, Writeable},
};

#[derive(Debug, Clone)]
pub struct Identifier {
    pub namespace: String,
    pub value: String,
}

impl Identifier {
    pub fn minecraft(value: &str) -> Self {
        Self {
            namespace: String::from("minecraft"),
            value: value.to_string(),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.value)
    }
}

impl Readable for Identifier {
    fn read(buffer: &[u8]) -> Result<(Self, usize), super::ReadError> {
        let mut reader = BufferReader::new(buffer);
        let raw_value = reader.read(String::read)?;

        if let Some((namespace, value)) = raw_value.split_once(':') {
            return Ok((
                Self {
                    namespace: namespace.to_string(),
                    value: value.to_string(),
                },
                reader.consumed(),
            ));
        }

        Ok((
            Self {
                namespace: String::from("minecraft"),
                value: raw_value,
            },
            reader.consumed(),
        ))
    }
}

impl Writeable for Identifier {
    fn write(&self) -> Result<bytes::Bytes, super::WriteError> {
        let mut buffer = BytesMut::new(); // TODO: capacity
        buffer.extend_from_slice(&format!("{}:{}", self.namespace, self.value).write()?);
        Ok(buffer.freeze())
    }
}
