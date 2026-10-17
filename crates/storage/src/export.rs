//! Encrypted export file for moving to another device (docs/konzept.md, "Gerätewechsel").
//!
//! Format: `HTEX1` | salt (16 bytes) | nonce (24 bytes) | XChaCha20-Poly1305 ciphertext.
//! The key comes from the passphrase via Argon2id with the crate's default parameters.

use argon2::Argon2;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};

const MAGIC: &[u8; 5] = b"HTEX1";
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 24;
pub const MIN_PASSPHRASE_CHARS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportError {
    PassphraseTooShort,
    NotAnExportFile,
    /// Wrong passphrase or a damaged file; AEAD cannot tell the two apart.
    WrongPassphraseOrDamaged,
    Random,
    Crypto,
}

fn key(passphrase: &str, salt: &[u8]) -> Result<[u8; 32], ExportError> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(passphrase.as_bytes(), salt, &mut key)
        .map_err(|_| ExportError::Crypto)?;
    Ok(key)
}

pub fn encrypt(passphrase: &str, plaintext: &[u8]) -> Result<Vec<u8>, ExportError> {
    if passphrase.chars().count() < MIN_PASSPHRASE_CHARS {
        return Err(ExportError::PassphraseTooShort);
    }
    let mut salt = [0u8; SALT_LEN];
    let mut nonce = [0u8; NONCE_LEN];
    getrandom::getrandom(&mut salt).map_err(|_| ExportError::Random)?;
    getrandom::getrandom(&mut nonce).map_err(|_| ExportError::Random)?;
    let cipher = XChaCha20Poly1305::new(&key(passphrase, &salt)?.into());
    let ciphertext = cipher
        .encrypt(XNonce::from_slice(&nonce), plaintext)
        .map_err(|_| ExportError::Crypto)?;
    let mut out = Vec::with_capacity(MAGIC.len() + SALT_LEN + NONCE_LEN + ciphertext.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

pub fn decrypt(passphrase: &str, file: &[u8]) -> Result<Vec<u8>, ExportError> {
    let header = MAGIC.len() + SALT_LEN + NONCE_LEN;
    if file.len() <= header || &file[..MAGIC.len()] != MAGIC {
        return Err(ExportError::NotAnExportFile);
    }
    let salt = &file[MAGIC.len()..MAGIC.len() + SALT_LEN];
    let nonce = &file[MAGIC.len() + SALT_LEN..header];
    let cipher = XChaCha20Poly1305::new(&key(passphrase, salt)?.into());
    cipher
        .decrypt(XNonce::from_slice(nonce), &file[header..])
        .map_err(|_| ExportError::WrongPassphraseOrDamaged)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PASS: &str = "correct horse battery";

    #[test]
    fn round_trip() {
        let file = encrypt(PASS, b"profile data").unwrap();
        assert!(file.starts_with(MAGIC));
        assert!(!file.windows(12).any(|w| w == b"profile data"));
        assert_eq!(decrypt(PASS, &file).unwrap(), b"profile data");
    }

    #[test]
    fn same_input_gives_different_files() {
        assert_ne!(encrypt(PASS, b"x").unwrap(), encrypt(PASS, b"x").unwrap());
    }

    #[test]
    fn wrong_passphrase_and_tampering_are_rejected() {
        let mut file = encrypt(PASS, b"profile data").unwrap();
        assert_eq!(
            decrypt("wrong passphrase", &file),
            Err(ExportError::WrongPassphraseOrDamaged)
        );
        let last = file.len() - 1;
        file[last] ^= 1;
        assert_eq!(
            decrypt(PASS, &file),
            Err(ExportError::WrongPassphraseOrDamaged)
        );
    }

    #[test]
    fn rejects_short_passphrase_and_foreign_files() {
        assert_eq!(encrypt("short", b"x"), Err(ExportError::PassphraseTooShort));
        assert_eq!(
            decrypt(PASS, b"{\"json\": true}"),
            Err(ExportError::NotAnExportFile)
        );
    }
}
