use rsa::{Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use std::fs;
use std::str;
use std::path::PathBuf;

// todo: return pathbuf or file?
pub fn rsa_encrypt(data: PathBuf) -> (PathBuf, RsaPublicKey) {
    let file = fs::read_to_string(data).unwrap();
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key.");
    let pub_key = RsaPublicKey::from(&priv_key);

    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &file.as_bytes());
    todo!()
}

pub fn rsa_decrypt(data: PathBuf, key: String) {
    todo!()
}
