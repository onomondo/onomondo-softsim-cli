use base64::{engine::general_purpose, Engine as _};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs8::DecodePrivateKey;
use rsa::{Oaep, RsaPrivateKey};
use serde_json;
use std::fs;
use std::path::PathBuf;

use super::Profile;
#[derive(Debug)]
pub struct Key {
    key: rsa::RsaPrivateKey,
    pub os_path: PathBuf,
}

impl Key {
    pub fn new(path: &PathBuf) -> Result<Key, Box<dyn std::error::Error>> {
        let buffer = fs::read(path).map_err(|err| {
            format!(
                "Failed to read private key. Is the path correct? Err: {}",
                err
            )
        })?;

        let mut keys = Vec::new();
          
        if let Ok(string) = String::from_utf8(buffer.clone()) {
            keys.push(RsaPrivateKey::from_pkcs8_pem(&string).map_err(|e|{format!("pkcs8-pem{}",e)}));
            keys.push(RsaPrivateKey::from_pkcs1_pem(&string).map_err(|e|{format!("pkcs1-pem{}",e)}));
        }
        keys.push(RsaPrivateKey::from_pkcs8_der(&buffer).map_err(|e|{format!("pkcs8-der{}",e)}));
        keys.push(RsaPrivateKey::from_pkcs1_der(&buffer).map_err(|e|{format!("pkcs1-der{}",e)}));

        // we're cloning in order to report errors later... 
        if let Some(private_key) = keys.clone().into_iter().find(|key| key.is_ok()){
            Ok(Key {
                key: private_key.unwrap(),
                os_path: path.clone(),
            })

        } else {
            for key in &keys {
                log::trace!("{:?}", key);
            } 

             Err("Failed to decode private key. Is the key corrupted?".into())
        }
 
    }

    pub fn decrypt(&self, data: &String) -> Result<Profile, Box<dyn std::error::Error>> {
        let bytes = general_purpose::STANDARD.decode(data).map_err(|e| {
            format!(
                "Failed to decode base64 string. Is the data corrupted? Err: {}",
                e
            )
        })?;

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
    use fs::read_to_string;
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
MIIEogIBAAKCAQEAhbID29kv1/nlIWEftPuTW3FVCSTtKXYH+jfEC1yliBvBdIi9
8PtKfB7AxejfKNrxXNgHZYoF2U6G1eMiRDo8WUoC6BkljnXyJtARNraDOV2zyGMX
9wL/S+z/5bKP3HELhG00JIxBLXwtGYlG6byNxt/rGFcN3UHlWAS4F2t45nb0DLER
SvHRFGug77+hcoIJMbuBKFymZmfG67InOQrj/3q7aCo48SaRT8nocoTOO2vVKz6o
Tk25ZCOR51esb89M5xHdPrkwR1VhDp5keDrkIs0ehB3YFyNtLP4GeN+PrBaa5mNu
FmpH86CKps6GVHxllhsVvNFqTlj9suA8eJVyAwIDAQABAoIBADlimWlqJbEMYB8d
syBZw3xCfv93zBw5v17VRN6jm6F6WGUOtIiVM/lmsdPBCthccbEqQLKCtdP6csGn
xnOGk5S3wduhv48QbSnfsSPM3zJmV96xOf4aWT5bsdMD+9/g3dKvlOwDD47Yd/s4
AyocdrZgMma2JwtLV4eaLvJOfQxeaAgS+X28PNBXVXUrPT54mHpv/L5DKPjosQeY
T6JTAggfo3lnR6ARAWTgoB4dpan7xINYmiobFW8OCDQjgeNQno8mqTyYBZ1rannR
TxYaV4YZm+ohiI4+VVorW4ygDmHkADOITpKajvf2zouTRB8rsg8NAMaL/bgX0Gpc
qGUyXEECgYEA/FAykxDSb6SvzdHJ8TEWzt3RHQcVP7a33O3qdpVDBSEGeXVZA/gT
jZyQ+CMNuCvuYwZj6COhcV2RRhceMT6wCj0ZGNLufIi8a6PUBVK8i0BIODboP0Dn
PrBJ0vjMJfDcTR1UjR2HM2kBnJAQZv2duHzsM51By8DIdsCzF1QR+FMCgYEAh6Yd
nKvT989AlgwiZ7VKCdSTYOyfqm44cR1PfwkjLsxiIgOcLWzmqbuKSf/M9iOSNXzG
PgowzvyC1PU67+P7E9o/mlQVFqxVEbDLWKdrQe3ZInonNjmIZMuCOSSQUZjXmQ03
jrsaEepvHF78J6c/ftL4roI9sXtCJenr4wmDqZECgYAvt1L5MRM6/eApgmU4cdoo
YRas8Kv6EqoO5AeVSYEVNTuuOJ0O/RlljJh19NshW12H6Kt4OVTMxa81nCWfloVg
SlG2uh1T+9/2U1NDdnQluZBu4SNm4vVYi6pKdbbV7gkbpFJbJZxuAovFehFACPKM
80MQN8s7p0fB/Ytg/Asx9QKBgAjngJ4eMWXc9vJijYedQOTwTVRZdAmt/op+UA+Y
u/SAehidgA0DTwjpRKSi5ANsRla4gBLmfFm5/aSZDGte3cZp1y78Qf4hBdyJbG+/
Xa0QPeajeht3H1ruePfjTI3uqIhlc7Ys1gNDDlQgcBDyS5q5opVh+0bBPpBakJ9w
qnHhAoGAPey0MkyKtczO8e7C42pCHo1jhMlyOMFSUh2qQ4P44WKzuBCN7Ph07rdU
BmOUvg4mjS21Y16t9NvdqOrxklCXC5ltB7x4MCQoyABJi4Jumx/+nQEX30/Sq+Fd
qGcyiTg4DOfzxrCux64nSS4+AdkWSt8eCZPvhCbWkq+V+L0Aswg=
-----END RSA PRIVATE KEY-----"#;

        let base64_string = r#"BSkPnnhkcqzIiiepLHv/SuLyclSzEA92sKXqK3ap8ub/kxbHVkMztYV1NtECzltY44LIScDYUYb/HzrfIR1FdrvLllIW2QWTT7h+rb0BXzyuPNaDJhQB/ijFHqQcgYKL/NSD8qQ+5/WSEHl/U5VsGGyw0q/U6SKkYhxCd8g2PbUUnIvj/06NsKaUSQj/iiFDkJfGvTRkLGL5VlBiqTAuvvUx2iZ0MU163wSjfBjh0Zgz9v7u93PDTC+jl+vGJYuyVG0QsdxYwbErU15dI78f73PirNJBcKk6gJyJu+lRK34h3hU7hwYbeby3qKoR936Lv7fVkLl4AinKB3Eilz/pGg=="#;

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
