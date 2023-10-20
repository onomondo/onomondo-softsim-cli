use super::Profile;
use serde::Serialize;

#[allow(dead_code)]
enum Tags {
    Imsi = 1,
    Iccid = 2,
    Opc = 3,
    Ki = 4,
    Kic = 5,
    Kid = 6,
    Msisdn = 7,
    End = 0xff,
}
#[derive(Serialize)]
struct AdditionField {
    name: String,
    file: String,
    content: String,
}
#[derive(Serialize)]
struct ExtendedProfile {
    profile: Profile,
    additional_fields: Vec<AdditionField>,
}

pub fn to_json(p: &Profile) -> Result<String, Box<dyn std::error::Error>> {
    let mut profile = ExtendedProfile {
        profile: p.clone(),
        additional_fields: Vec::new(),
    };

    if let Some(i) = &p.imsi {
        let imsi = AdditionField {
            name: String::from("encoded imsi"),
            file: String::from("/3f00/7ff0/6f07"),
            content: encode_imsi(i),
        };

        profile.additional_fields.push(imsi);
    };

    if let Some(i) = &p.iccid {
        let iccid = AdditionField {
            name: String::from("encoded iccid"),
            file: String::from("/3f00/2fe2"),
            content: swap_nibbles(i),
        };

        profile.additional_fields.push(iccid);
    };

    if let (Some(o), Some(k)) = (&p.opc, &p.k) {
        let a001 = AdditionField {
            name: String::from("Key material for attaching to network"),
            file: String::from("/3f00/a001"),
            content: format!("{}{}00", k, o),
        };

        profile.additional_fields.push(a001);
    };

    if let (Some(kid), Some(kic)) = (&p.kid, &p.kic) {
        let a004 = AdditionField {
            name: String::from("Key material for OTA related functions"),
            file: String::from("/3f00/a004"),
            content: format!("b00011060101{}{}{}", kic, kid, rpad("", 2 * 76, None)),
        };

        profile.additional_fields.push(a004);
    };

    let t = serde_json::to_string(&profile)?;
    Ok(t)
}

pub fn to_hex(p: &Profile) -> String {
    let mut ret = String::new();
    if let Some(imsi) = &p.imsi {
        let encoded_imsi = encode_imsi(imsi);
        ret.push_str(
            format!(
                "{:02x}{:02x}{}",
                Tags::Imsi as u8,
                encoded_imsi.len(),
                encoded_imsi
            )
            .as_str(),
        );
    }

    if let Some(iccid) = &p.iccid {
        let encoded_iccid = swap_nibbles(iccid);
        ret.push_str(
            format!(
                "{:02x}{:02x}{}",
                Tags::Iccid as u8,
                encoded_iccid.len(),
                encoded_iccid
            )
            .as_str(),
        );
    }

    if let Some(opc) = &p.opc {
        ret.push_str(format!("{:02x}{:02x}{}", Tags::Opc as u8, opc.len(), opc).as_str());
    }

    if let Some(ki) = &p.k {
        ret.push_str(format!("{:02x}{:02x}{}", Tags::Ki as u8, ki.len(), ki).as_str());
    }
    if let Some(kic) = &p.kic {
        ret.push_str(format!("{:02x}{:02x}{}", Tags::Kic as u8, kic.len(), kic).as_str());
    }

    if let Some(kid) = &p.kid {
        ret.push_str(format!("{:02x}{:02x}{}", Tags::Kid as u8, kid.len(), kid).as_str());
    }

    if let Some(msisdn) = &p.msisdn {
        ret.push_str(format!("{:02x}{:02x}{}", Tags::Msisdn as u8, msisdn.len(), msisdn).as_str());
    }
    ret
}

fn encode_imsi(imsi: &str) -> String {
    let l = half_round_up(imsi.len() + 1);
    let oe = imsi.len() & 1;

    // this is the worst.
    format!(
        "{:02x}{}",
        l,
        swap_nibbles(&format!(
            "{:x}{}",
            (oe << 3) | 1,
            rpad(imsi, 15, Some(b'f'))
        ))
    )
}

fn swap_nibbles(s: &str) -> String {
    let mut data: Vec<_> = s.chars().collect();
    for idx in (0..s.len() - 1).step_by(2) {
        data.swap(idx, idx + 1);
    }

    data.into_iter().collect()
}

fn half_round_up(n: usize) -> usize {
    (n + 1) / 2
}

fn rpad(s: &str, l: usize, b: Option<u8>) -> String {
    let padding_len = l - s.len();
    let pad = String::from_utf8(vec![b.unwrap_or(b'f'); padding_len]).unwrap();
    format!("{}{}", s, pad)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpad_default() {
        assert_eq!(rpad(&String::from("abc"), 5, None), "abcff");
        assert_eq!(rpad(&String::from("abc"), 5, Some(b'e')), "abcee");
    }

    #[test]
    fn test_swap_nibbles() {
        assert_eq!(swap_nibbles("1234"), "2143");
        assert_eq!(swap_nibbles("1"), "1");
        assert_eq!(swap_nibbles("1234567890abcdef"), "2143658709badcfe");
    }

    #[test]
    fn test_imsi_encoder() {
        assert_eq!(encode_imsi("234602102350049"), "082943061220530094");
        assert_eq!(encode_imsi("234602102349958"), "082943061220439985");
    }

    #[test]
    fn test_encode_iccid() {
        assert_eq!(swap_nibbles("89457300000013500452"), "98543700000031054025")
    }

    #[test]
    fn encodes_testprofile() {
        let p = Profile {
            iccid: Some(String::from("89000123456789012341")),
            imsi: Some(String::from("001010123456063")),
            opc: Some(String::from("00000000000000000000000000000000")),
            k: Some(String::from("000102030405060708090A0B0C0D0E0F")),
            kic: Some(String::from("000102030405060708090A0B0C0D0E0F")),
            kid: Some(String::from("000102030405060708090A0B0C0D0E0F")),
            msisdn: None,
        };

        assert_eq!(
            "080910101032540636",
            encode_imsi(p.imsi.as_deref().unwrap())
        );
        assert_eq!(
            "98001032547698103214",
            swap_nibbles(p.iccid.as_deref().unwrap())
        );
        assert_eq!(to_hex(&p), "01120809101010325406360214980010325476981032140320000000000000000000000000000000000420000102030405060708090A0B0C0D0E0F0520000102030405060708090A0B0C0D0E0F0620000102030405060708090A0B0C0D0E0F")
    }
}
