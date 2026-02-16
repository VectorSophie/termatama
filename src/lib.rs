pub mod rom;
pub mod state;
pub mod sys;
pub mod tui;

use std::path::Path;

pub use rom::{decode_rom, load_rom_words, RomDecodeError, RomEncoding};
pub use sys::{Button, CpuStateView, EngineError, TamaEngine};

pub fn load_engine_from_file(path: &Path) -> Result<TamaEngine, EngineError> {
    let rom = load_rom_words(path).map_err(EngineError::Rom)?;
    TamaEngine::new(rom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_roundtrip() {
        let words = vec![0x000, 0x123, 0xABC, 0xFFF, 0x456, 0x789];
        let bytes = rom::pack_12bit_le(&words);
        let restored = rom::unpack_12bit_le(&bytes).expect("unpack");
        assert_eq!(restored[..words.len()], words);
    }

    #[test]
    fn engine_can_tick_with_zero_rom() {
        let words = vec![0u16; 4096];
        let mut engine = TamaEngine::new(words).expect("engine init");
        engine.tick_many(16);
    }

    #[test]
    fn optional_real_tama_b_smoke() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let path = root.join("roms\tama.b");
        if !path.exists() {
            return;
        }

        let mut engine = load_engine_from_file(&path).expect("load tama.b");
        engine.tick_many(32);
    }
}
