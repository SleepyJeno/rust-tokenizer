use rand::RngCore;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub struct Cipher {
    key: Vec<u8>,
    iv: Vec<u8>,
    pub cipher: Aes128Cbc
}

impl Cipher {
    pub fn new() -> Self{ 
        let key = Self::generate_key();
        let iv = Self::generate_iv();
        let cipher: Aes128Cbc = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
        Self {
            key,
            iv,
            cipher
          }
    }

    fn generate_key() -> Vec<u8> {
        let mut bytes = [0; 16];
        rand::thread_rng().fill_bytes(&mut bytes);
        bytes.to_vec()
    }

    fn generate_iv() -> Vec<u8> {
        let mut bytes = [0; 16];
        rand::thread_rng().fill_bytes(&mut bytes);
        bytes.to_vec()
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let pos = plaintext.len();
        let mut buffer = [0u8; 128];
        buffer[..pos].copy_from_slice(plaintext);
        let ciphertext = self.cipher.clone().encrypt(&mut buffer, pos).unwrap();
        ciphertext.to_vec()
      }
    
    pub fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        let mut buffer = ciphertext.to_vec();
        let decrypted = self.cipher.clone().decrypt(&mut buffer).unwrap();
        decrypted.to_vec()
      }
}