//! Encoding-tolerant text reading.
//!
//! Windows learners produce files in at least three shapes:
//!   * UTF-8 (modern editors, PowerShell 7, `wb erfasse`)
//!   * UTF-16LE (PowerShell 5.1 `>` and `Out-File`, usually with BOM)
//!   * CP850 (`cmd.exe` redirection on a German/Austrian console)
//!
//! The order is fixed and deterministic (SPEC §3) rather than statistical: the
//! same file must always produce the same check result, on every machine.

use std::io;
use std::path::Path;

use super::cp850;

/// Upper bound for a single learner file. `wb status` reads every checked file
/// of every exercise, so an accidental runaway `Out-File` loop in `abgabe/`
/// must not be able to exhaust the VM's memory. Every documented check works on
/// a prefix: regexes match within it, and a file this large clears any
/// `datei_zeilen_min` many times over.
pub const MAX_READ_BYTES: usize = 8 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEncoding {
    Utf8,
    Utf8Bom,
    Utf16Le,
    Utf16Be,
    Cp850,
}

/// Decode a byte buffer into text, trying UTF-8 -> UTF-16 -> CP850.
pub fn decode(bytes: &[u8]) -> (String, TextEncoding) {
    if let Some(rest) = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
        return (
            String::from_utf8_lossy(rest).into_owned(),
            TextEncoding::Utf8Bom,
        );
    }
    if let Some(rest) = bytes.strip_prefix(&[0xFF, 0xFE]) {
        return (decode_utf16(rest, true), TextEncoding::Utf16Le);
    }
    if let Some(rest) = bytes.strip_prefix(&[0xFE, 0xFF]) {
        return (decode_utf16(rest, false), TextEncoding::Utf16Be);
    }
    // Order matters: BOM-less UTF-16LE that holds plain ASCII is *also* valid
    // UTF-8 (every second byte is a NUL, which UTF-8 allows). So the UTF-16
    // shape has to be ruled out before UTF-8 is accepted, or PowerShell 5.1
    // output would decode into text full of NUL characters.
    if looks_like_utf16le(bytes) {
        return (decode_utf16(bytes, true), TextEncoding::Utf16Le);
    }
    if let Ok(text) = std::str::from_utf8(bytes) {
        return (text.to_string(), TextEncoding::Utf8);
    }
    (cp850::decode(bytes), TextEncoding::Cp850)
}

/// Read a file as text, tolerating the encodings above. Never reads more than
/// [`MAX_READ_BYTES`], so a runaway file in `abgabe/` cannot exhaust memory.
pub fn read(path: &Path) -> io::Result<(String, TextEncoding)> {
    use std::io::Read;

    let file = std::fs::File::open(path)?;
    let mut bytes = Vec::new();
    file.take(MAX_READ_BYTES as u64).read_to_end(&mut bytes)?;
    Ok(decode(&bytes))
}

fn decode_utf16(bytes: &[u8], little_endian: bool) -> String {
    let encoding = if little_endian {
        encoding_rs::UTF_16LE
    } else {
        encoding_rs::UTF_16BE
    };
    let (text, _had_errors) = encoding.decode_without_bom_handling(bytes);
    text.into_owned()
}

/// BOM-less UTF-16LE heuristic: Latin text in UTF-16LE has a NUL in every
/// second byte. Requires an even length and mostly-empty high bytes.
fn looks_like_utf16le(bytes: &[u8]) -> bool {
    if bytes.len() < 4 || bytes.len() % 2 != 0 {
        return false;
    }
    let pairs = bytes.len() / 2;
    let sample = pairs.min(2048);
    let mut high_nulls = 0usize;
    let mut low_nulls = 0usize;
    for i in 0..sample {
        if bytes[2 * i + 1] == 0 {
            high_nulls += 1;
        }
        if bytes[2 * i] == 0 {
            low_nulls += 1;
        }
    }
    high_nulls * 10 >= sample * 7 && low_nulls * 10 < sample * 3
}

/// Count lines that contain more than whitespace.
pub fn count_non_empty_lines(text: &str) -> usize {
    text.lines().filter(|line| !line.trim().is_empty()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn utf16le_bytes(text: &str, bom: bool) -> Vec<u8> {
        let mut out = Vec::new();
        if bom {
            out.extend_from_slice(&[0xFF, 0xFE]);
        }
        for unit in text.encode_utf16() {
            out.extend_from_slice(&unit.to_le_bytes());
        }
        out
    }

    #[test]
    fn plain_utf8() {
        let (text, enc) = decode("Größe: 16 GB".as_bytes());
        assert_eq!(text, "Größe: 16 GB");
        assert_eq!(enc, TextEncoding::Utf8);
    }

    #[test]
    fn utf8_with_bom() {
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice("Gesamter physischer Speicher".as_bytes());
        let (text, enc) = decode(&bytes);
        assert_eq!(text, "Gesamter physischer Speicher");
        assert_eq!(enc, TextEncoding::Utf8Bom);
    }

    #[test]
    fn utf16le_with_bom() {
        let (text, enc) = decode(&utf16le_bytes("Systeminfo\r\nGröße: 8 GB\r\n", true));
        assert_eq!(text, "Systeminfo\r\nGröße: 8 GB\r\n");
        assert_eq!(enc, TextEncoding::Utf16Le);
    }

    #[test]
    fn utf16le_without_bom() {
        let (text, enc) = decode(&utf16le_bytes("Hostname: aquinas\r\nRAM: 16 GB\r\n", false));
        assert_eq!(text, "Hostname: aquinas\r\nRAM: 16 GB\r\n");
        assert_eq!(enc, TextEncoding::Utf16Le);
    }

    #[test]
    fn utf16be_with_bom() {
        let mut bytes = vec![0xFE, 0xFF];
        for unit in "Speicher".encode_utf16() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        let (text, enc) = decode(&bytes);
        assert_eq!(text, "Speicher");
        assert_eq!(enc, TextEncoding::Utf16Be);
    }

    #[test]
    fn cp850_fallback() {
        // "Größe: 4 Kerne" as a German cmd.exe would write it
        let mut bytes = b"Gr".to_vec();
        bytes.extend_from_slice(&[0x94, 0xE1]); // ö, ß
        bytes.extend_from_slice(b"e: 4 Kerne");
        let (text, enc) = decode(&bytes);
        assert_eq!(text, "Größe: 4 Kerne");
        assert_eq!(enc, TextEncoding::Cp850);
    }

    #[test]
    fn ascii_is_not_mistaken_for_utf16() {
        let (text, enc) = decode(b"ab");
        assert_eq!(text, "ab");
        assert_eq!(enc, TextEncoding::Utf8);
    }

    #[test]
    fn empty_input() {
        let (text, enc) = decode(b"");
        assert_eq!(text, "");
        assert_eq!(enc, TextEncoding::Utf8);
    }

    #[test]
    fn non_empty_line_counting() {
        let text = "eins\r\n\r\n   \nzwei\ndrei";
        assert_eq!(count_non_empty_lines(text), 3);
    }

    /// `wb status` reads every checked file of every exercise. A runaway
    /// `Out-File` loop in `abgabe/` must not make the runner allocate the whole
    /// file — instant feedback is the product.
    #[test]
    fn read_stops_at_the_size_cap() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("riesig.txt");
        let line = b"Zeile voller Text\n";
        let mut bytes = Vec::with_capacity(MAX_READ_BYTES + line.len() * 64);
        while bytes.len() < MAX_READ_BYTES + line.len() * 32 {
            bytes.extend_from_slice(line);
        }
        let written = bytes.len();
        std::fs::write(&path, &bytes).unwrap();

        let (text, _) = read(&path).unwrap();
        assert!(
            text.len() <= MAX_READ_BYTES,
            "read {} bytes from a {written}-byte file — the cap did not hold",
            text.len()
        );
    }

    /// A file below the cap must still be read whole, byte for byte.
    #[test]
    fn read_returns_small_files_completely() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("klein.txt");
        std::fs::write(&path, "Größe: 4 Kerne\nzwei\n").unwrap();

        let (text, enc) = read(&path).unwrap();
        assert_eq!(text, "Größe: 4 Kerne\nzwei\n");
        assert_eq!(enc, TextEncoding::Utf8);
    }
}
