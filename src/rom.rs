use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RomEncoding {
    Packed12Le,
    Padded16Le12,
    Padded16Be12,
}

#[derive(Debug)]
pub enum RomDecodeError {
    InvalidLength { len: usize },
    Io(std::io::Error),
}

impl Display for RomDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength { len } => write!(f, "invalid ROM byte length: {len}"),
            Self::Io(err) => write!(f, "ROM I/O error: {err}"),
        }
    }
}

impl std::error::Error for RomDecodeError {}

impl From<std::io::Error> for RomDecodeError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub fn load_rom_words(path: &Path) -> Result<Vec<u16>, RomDecodeError> {
    let bytes = fs::read(path)?;
    decode_rom(&bytes)
}

pub fn decode_rom(bytes: &[u8]) -> Result<Vec<u16>, RomDecodeError> {
    let encoding = detect_encoding(bytes)?;
    Ok(match encoding {
        RomEncoding::Packed12Le => unpack_12bit_le(bytes)?,
        RomEncoding::Padded16Le12 => unpack_16bit_container_le(bytes)?,
        RomEncoding::Padded16Be12 => unpack_16bit_container_be(bytes)?,
    })
}

pub fn detect_encoding(bytes: &[u8]) -> Result<RomEncoding, RomDecodeError> {
    if bytes.is_empty() {
        return Err(RomDecodeError::InvalidLength { len: 0 });
    }

    if bytes.len().is_multiple_of(2) {
        let looks_padded_le = bytes.chunks_exact(2).all(|pair| (pair[0] & 0xF0) == 0);
        if looks_padded_le {
            return Ok(RomEncoding::Padded16Le12);
        }

        let looks_padded_be = bytes.chunks_exact(2).all(|pair| (pair[0] & 0xF0) == 0);
        if looks_padded_be {
            return Ok(RomEncoding::Padded16Be12);
        }
    }

    if bytes.len().is_multiple_of(3) {
        return Ok(RomEncoding::Packed12Le);
    }

    Err(RomDecodeError::InvalidLength { len: bytes.len() })
}

pub fn unpack_12bit_le(bytes: &[u8]) -> Result<Vec<u16>, RomDecodeError> {
    if !bytes.len().is_multiple_of(3) {
        return Err(RomDecodeError::InvalidLength { len: bytes.len() });
    }

    let mut out = Vec::with_capacity((bytes.len() / 3) * 2);
    for triple in bytes.chunks_exact(3) {
        let b0 = triple[0] as u16;
        let b1 = triple[1] as u16;
        let b2 = triple[2] as u16;
        let w0 = b0 | ((b1 & 0x0F) << 8);
        let w1 = (b1 >> 4) | (b2 << 4);
        out.push(w0 & 0x0FFF);
        out.push(w1 & 0x0FFF);
    }
    Ok(out)
}

pub fn pack_12bit_le(words: &[u16]) -> Vec<u8> {
    let mut out = Vec::with_capacity(words.len().div_ceil(2) * 3);

    for pair in words.chunks(2) {
        let w0 = pair[0] & 0x0FFF;
        let w1 = pair.get(1).copied().unwrap_or(0) & 0x0FFF;

        out.push((w0 & 0xFF) as u8);
        out.push((((w0 >> 8) & 0x0F) | ((w1 & 0x0F) << 4)) as u8);
        out.push((w1 >> 4) as u8);
    }

    out
}

pub fn unpack_16bit_container_le(bytes: &[u8]) -> Result<Vec<u16>, RomDecodeError> {
    if !bytes.len().is_multiple_of(2) {
        return Err(RomDecodeError::InvalidLength { len: bytes.len() });
    }

    let mut out = Vec::with_capacity(bytes.len() / 2);
    for pair in bytes.chunks_exact(2) {
        let hi_nibble = (pair[0] as u16 & 0x0F) << 8;
        let lo_byte = pair[1] as u16;
        out.push((hi_nibble | lo_byte) & 0x0FFF);
    }
    Ok(out)
}

pub fn unpack_16bit_container_be(bytes: &[u8]) -> Result<Vec<u16>, RomDecodeError> {
    if !bytes.len().is_multiple_of(2) {
        return Err(RomDecodeError::InvalidLength { len: bytes.len() });
    }

    let mut out = Vec::with_capacity(bytes.len() / 2);
    for pair in bytes.chunks_exact(2) {
        let word = u16::from_be_bytes([pair[0], pair[1]]);
        out.push(word & 0x0FFF);
    }
    Ok(out)
}
