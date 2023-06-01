pub mod aes;
pub mod buf;
pub mod encoding;
pub mod error;
mod file;

use self::aes::{
    raw::{CipherRawAes128, CipherRawAes256},
    CipherAes,
};
use error::RfcError;

#[derive(Clone, PartialEq, Debug)]
pub enum Mode {
    Aes128,
    Aes256,
}

pub trait Cipher {
    type Output;

    fn crypt<T, U>(bytes: T, key: U, decrypt: bool) -> Result<Self::Output, RfcError>
    where
        T: AsRef<[u8]>,
        U: AsRef<[u8]>,
    {
        if decrypt {
            return Self::decrypt(bytes, key);
        }

        Self::encrypt(bytes, key)
    }

    fn encrypt<T, U>(bytes: T, key: U) -> Result<Self::Output, RfcError>
    where
        T: AsRef<[u8]>,
        U: AsRef<[u8]>;

    fn decrypt<T, U>(bytes: T, key: U) -> Result<Self::Output, RfcError>
    where
        T: AsRef<[u8]>,
        U: AsRef<[u8]>;
}

pub fn pre_process(
    bytes: Vec<u8>,
    decrypt: bool,
    codec: encoding::Encoding,
) -> Result<Vec<u8>, RfcError> {
    Ok(bytes)
}

pub fn crypt<T>(bytes: T, decrypt: bool, key: T, cipher: Mode) -> Result<Vec<u8>, RfcError>
where
    T: AsRef<[u8]>,
{
    match cipher {
        Mode::Aes128 => CipherAes::<CipherRawAes128>::crypt(bytes, key, decrypt),
        Mode::Aes256 => CipherAes::<CipherRawAes256>::crypt(bytes, key, decrypt),
    }
}

pub fn post_process(
    bytes: Vec<u8>,
    decrypt: bool,
    codec: encoding::Encoding,
) -> Result<Vec<u8>, RfcError> {
    Ok(bytes)
}

#[cfg(test)]
pub mod tests {
    use super::Cipher;

    pub fn test_encryption<C: Cipher<Output = Vec<u8>>>() {
        let tests = vec![include_str!("../../Cargo.toml"), "foo", "./mod.rs"];
        tests.into_iter().for_each(|plaintext| {
            let plaintext = plaintext.as_bytes();
            let key = "this_is_my_key".as_bytes();
            let ciphertext = C::encrypt(plaintext.clone(), &key).expect("encryption failed");
            assert!(ciphertext.len() != 0);

            let plaintext_result =
                C::decrypt::<Vec<u8>, &[u8]>(ciphertext, key.into()).expect("decryption failed");
            assert_eq!(plaintext, plaintext_result);
        });
    }
}
