use base64::{engine::general_purpose, Engine as _};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use serde_json;
use std::fs::read_to_string;
use std::path::PathBuf;

use super::Profile;
#[derive(Debug)]
pub struct Key {
    key: rsa::RsaPrivateKey,
    pub os_path: PathBuf,
}

impl Key {
    pub fn new(path: &PathBuf) -> Result<Key, Box<dyn std::error::Error>> {
        let buffer = read_to_string(path)?;

        // let private_key = rsa::pkcs8::DecodePrivateKey::from_pkcs8_pem(&buffer)?;
        let private_key = RsaPrivateKey::from_pkcs1_pem(&buffer)?;
        Ok(Key {
            key: private_key,
            os_path: path.clone(),
        })
    }

    pub fn decrypt(&self, data: &String) -> Result<Profile, Box<dyn std::error::Error>> {
        let bytes = general_purpose::STANDARD.decode(data).unwrap();

        // padding: crypto.constants.RSA_PKCS1_PADDING -> onomondo-sims
        // TODO: get this to match the padding originally intended
        let dec_date = String::from_utf8(self.key.decrypt(Pkcs1v15Encrypt, &bytes)?)?;
        let profile: Profile = serde_json::from_str(&dec_date)?;

        Ok(profile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::profile::api;
    #[test]
    fn import_key() {
        let mut sample_key = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        sample_key.push("resources/test/key");
        let key = Key::new(&sample_key).unwrap();

        let mut sample_profile = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        sample_profile.push("resources/test/response.json");

        let s = read_to_string(sample_profile).unwrap();
        // content is same format as the api response
        let api_response: api::Response = serde_json::from_str(&s).unwrap();

        let decrypted_profile: Vec<_> = api_response
            .profiles
            .into_iter()
            .map(|p| {
                let res = key.decrypt(&p.profile);
                match res {
                    Ok(p) => p,
                    Err(e) => {
                        println!("Error: {:?}", e);
                        println!("Context: {:?}", p);
                        panic!("Failed to decrypt profile")
                    }
                }
            })
            .collect();

        assert_eq!(decrypted_profile[0].opc.as_ref().unwrap(), "abcdef");
        assert_eq!(decrypted_profile[1].k.as_ref().unwrap(), "1234567890");
    }

    #[test]
    fn base64_decode() {
        let b64 = "SGVsbG8gV29ybGQ=".to_string();

        let decoded = general_purpose::STANDARD.decode(b64).unwrap();
        let asstring = String::from_utf8(decoded).unwrap();

        assert_eq!(asstring, "Hello World");
    }
}
