use crate::chunk_type::ChunkType;
use crc::{Algorithm, Crc};
use std::fmt::Display;

const CRC_32_CUSTOM: Algorithm<u32> = Algorithm {
    width: 32,
    poly: 0x04C11DB7,    // The *normal* polynomial, not reflected
    init: 0xFFFFFFFF,    // Start value
    refin: true,         // Because the implementation reflects input bits
    refout: true,        // Because output is reflected as well
    xorout: 0xFFFFFFFF,  // Final XOR (inversion)
    check: 0xCBF43926,   // CRC of "123456789"
    residue: 0xDEBB20E3, // Standard CRC-32 residue
};

#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc32 = Crc::<u32>::new(&CRC_32_CUSTOM);
        let chunk_type_and_chunk: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        let crc = crc32.checksum(&chunk_type_and_chunk);
        Chunk {
            data_length: data.len() as u32,
            chunk_type,
            data,
            crc,
        }
    }

    fn length(&self) -> u32 {
        self.data_length
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    fn data_as_string(&self) -> Result<String, String> {
        String::from_utf8(self.data.to_vec())
            .map_err(|err| format!("Error converting data to string: {err}"))
    }

    fn as_bytes(&self) -> Vec<u8> {
        let chunk_type_bytes = self.chunk_type.bytes();
        self.data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type_bytes.to_vec().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let data_length = <u32>::from_be_bytes(value[0..4].try_into().unwrap());
        let chunk_type_bytes: [u8; 4] = value[4..8]
            .try_into()
            .expect("slice must be exactly 4 bytes");

        let chunk_type =
            ChunkType::try_from(chunk_type_bytes).map_err(|_| "invalid chunk type bytes")?;

        let data_start = 9 - 1;
        let data_end = (9 + data_length - 1) as usize;
        let data = &value[data_start..data_end];

        let crc_bytes: [u8; 4] = value
            .get(data_end..data_end + 4)
            .ok_or("invalid crc bytes")?
            .try_into()
            .map_err(|_| "invalid crc bytes")?;

        let crc = u32::from_be_bytes(crc_bytes);

        // Check crc
        let crc32 = Crc::<u32>::new(&CRC_32_CUSTOM);
        let chunk_type_and_chunk: Vec<u8> = chunk_type
            .clone()
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        if crc32.checksum(&chunk_type_and_chunk) == crc {
            return Ok(Chunk {
                data_length,
                chunk_type,
                data: data.to_vec(),
                crc,
            });
        }
        Err("crc bytes does not match".to_string())
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(
            f,
            "  Data: {}",
            String::from_utf8(self.data().to_vec()).unwrap()
        )?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_crc() {
        let crc = Crc::<u32>::new(&CRC_32_CUSTOM);
        assert_eq!(crc.checksum(b"123456789"), 0xCBF43926);
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        print!("{chunk}");
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        print!("{chunk}");
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
