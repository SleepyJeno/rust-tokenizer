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

/*
    The Cipher struct is used to encrypt and decrypt data. It contains a key, an IV, and a cipher.
    The Cipher makes use of AES algorithm and the corresponding crate
    Users can add salt to the plaintext to make it harder for an attacker to decrypt the data

    get_cipher_data returns a tuple of (key, iv) to be saved in the DB
*/
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

    //TODO
    
    // fn generte_salt() -> Vec<u8> {
        //     let mut bytes = [0; 8];
        //     rand::thread_rng().fill_bytes(&mut bytes);
        //     bytes.to_vec()
        // }
        
        // pub fn add_salt(&self, plaintext: &[u8]) -> Vec<u8> {
            //     let salt = Self::generte_salt();
            //     let mut buffer = [0u8; 136];
            //     buffer[..plaintext.len()].copy_from_slice(plaintext);
            //     buffer[plaintext.len()..].copy_from_slice(&salt);
            //     buffer.to_vec()
            // }
    
    //TODO: make private
    pub fn get_cipher_data(&self) -> (Vec<u8>, Vec<u8>) {
        (self.key.clone(), self.iv.clone())
    }

    //TODO: pad ciphertext with zeros to reach 128 bytes
    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let pos = plaintext.len();
        let mut buffer = [0u8; 128];
        buffer[..pos].copy_from_slice(plaintext);
        let ciphertext = self.cipher.clone().encrypt(&mut buffer, pos).unwrap();
        ciphertext.to_vec()
    }
    
    //TODO: add check for salted data and decrypt accordingly
    pub fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        let mut buffer = ciphertext.to_vec();
        let decrypted = self.cipher.clone().decrypt(&mut buffer).unwrap();
        decrypted.to_vec()
      }
}