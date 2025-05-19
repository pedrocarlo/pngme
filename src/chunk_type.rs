use std::{fmt::Display, str::FromStr};

use anyhow::bail;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ChunkType([u8; 4]);

const FIFTH_BIT: u8 = 0b00100000;

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        (self.0[2] & FIFTH_BIT) >> 5 == 0
    }

    pub fn is_critical(&self) -> bool {
        (self.0[0] & FIFTH_BIT) >> 5 == 0
    }

    pub fn is_public(&self) -> bool {
        (self.0[1] & FIFTH_BIT) >> 5 == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.is_valid()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        (self.0[3] & FIFTH_BIT) >> 5 == 1
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for (idx, val) in value.iter().enumerate() {
            if !val.is_ascii_alphabetic() {
                bail!("not an ascii alphabetic byte")
            }
            // Third byte
            if idx == 2 {
                if (*val & FIFTH_BIT) == 1 {
                    bail!("reserved bit not valid")
                }
            }
        }
        Ok(Self(value))
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            bail!("string is not composed of 4 bytes");
        }
        if !s.is_ascii() {
            bail!("string should be ascii");
        }
        let value: [u8; 4] = s
            .chars()
            .map(|c| {
                if !c.is_ascii_alphabetic() {
                    bail!("not an ascii alphabetic byte");
                }
                Ok(c as u8)
            })
            .collect::<Result<Vec<_>, Self::Err>>()?
            .try_into()
            .unwrap();

        value.try_into()
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter() {
            write!(f, "{}", *c as char)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
