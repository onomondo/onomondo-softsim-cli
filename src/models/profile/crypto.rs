use base64::{engine::general_purpose, Engine as _};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::{Oaep, RsaPrivateKey};
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

        let padding = Oaep::new::<sha1::Sha1>();
        let dec_date = String::from_utf8(self.key.decrypt(padding, &bytes)?)?;
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

        println!("Decrypted: {:?}", decrypted_profile);
        assert_eq!(decrypted_profile[0].opc.as_ref().unwrap(), "abcdef");
        assert_eq!(decrypted_profile[1].opc.as_ref().unwrap(), "helloworld");
    }

    #[test]
    fn base64_decode() {
        let b64 = "SGVsbG8gV29ybGQ=".to_string();

        let decoded = general_purpose::STANDARD.decode(b64).unwrap();
        let asstring = String::from_utf8(decoded).unwrap();

        assert_eq!(asstring, "Hello World");
    }

    #[test]
    fn test_all_decrypt_variantions() {
        use rsa::{sha2, Oaep, Pkcs1v15Encrypt};

        let key = r#"-----BEGIN RSA PRIVATE KEY-----
MIICXAIBAAKBgQCm2R0JxqyPfZSGOWUd9vT2fGLOn5IGl1PfuwC4NuH4h0Ozoe71
8iisX1TBoR9RgqxUYfepiW9tjhjqOK3PDfwhWkMHxXPdM+naHV4ZiiNITU6H30St
mh0+GrMuUXWHHqygg+yzcUch2JtCc9rQLCXESOOwpMO1Ui8WXH3gBWM7lQIDAQAB
AoGAQZLg67+ugDKN1fbmu9EcU2dteeGTBY4iA7M+RCglxYR74jSJcxX6UEyjRfpq
EaH20q8yI+qE5ZzMQ/mErfTEGz4Zf4gpoZxv/GKzJ8MULmxO2X+E7Bm0DwjpITeL
bDV7l9597eAWgJJ1inRJ0rpT3F6IGN+5Mj7t3s4IhnX9NkECQQD60xkYA5VF8gbi
uJ1lnygKbL9fFP/Lh4JVZUH2MY2KAcTYGjJvxSVt2DFJFCX/mxif/yvRN4kUqR2c
IAkK91+FAkEAqkpvnDPiNnKVgr+zFVb6VsRs2WAEK8qr43AcwMG3W5lw/ptJ9A51
Of1uFDi5niYo9Rc2wcadXYQV/jlRnGlA0QJBAIQdnSIhAQeOrEHPrFhStOyIy2Rx
0yqJfgUtCMl84GjI9b4+TkLBPS3Wql8r1bgFIbtk1NemwPW4/ne2CA1Wr2ECQAPZ
zxBPNAxbJvpf72LKJrsTkgqQW0fKO3zXKi9JsiXGIIIBbPix4wC+tGCMr9Xdswtn
zPswzJoyxHSNQ0UwNCECQEeLTz8jDz1gHmDZYUT1Pk05FwyK2P7KhciuO5fmD9Gb
Kthw+VViUazIaTshRIqgZeL4x20slSuESZuTFllZCoA=
-----END RSA PRIVATE KEY-----"#;

        let base64_string = r#"cEFafGQ8OipYXKYL1nHLedFYBUwYPrHaJLWW6s/RgjK5waliUN2pcnfzZTwgsVeUClrWYYM5p9dfct7U1qUavWFn91lxKNErTLUZ5X8AMIpjDm8GFQGBG7srVhtaigAjaSsLK4XENQWzz/NEfEmmd9IiQCW0cud+xi7WLJvDOoo="#;

        let bytes = general_purpose::STANDARD.decode(base64_string).unwrap();
        let key = RsaPrivateKey::from_pkcs1_pem(key).unwrap();
        let mut results = Vec::new();

        // test 1
        let decrypted = key.decrypt(Pkcs1v15Encrypt, &bytes);
        results.push(decrypted);

        // test 2
        let padding = Oaep::new::<sha2::Sha256>();
        let decrypted = key.decrypt(padding, &bytes);
        results.push(decrypted);

        // test 3
        let padding = Oaep::new::<sha1::Sha1>();
        let decrypted = key.decrypt(padding, &bytes);
        results.push(decrypted);

        results.iter().filter(|res| res.is_ok()).for_each(|res| {
            let as_vec = res.as_ref().unwrap();
            let asstring = String::from_utf8(as_vec.clone()).unwrap();
            println!("Decrypted: {}", asstring);
        });

        println!("Results: {:?}", results);
        assert!(results.iter().any(|p| p.is_ok()))
    }
}
