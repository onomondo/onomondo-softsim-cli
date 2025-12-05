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
    Smsp = 7,
    Pin = 8,
    Adm = 10,
    Puk = 11,
    Smsc = 12,
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

impl Profile {
    pub fn to_json(&self, include_smsp: bool, include_smsc: bool) -> Result<String, Box<dyn std::error::Error>> {
        to_json(self, include_smsp, include_smsc)
    }
    pub fn to_hex(&self, include_smsp: bool, include_smsc: bool) -> String {
        to_hex(self, include_smsp, include_smsc)
    }
}

fn to_json(p: &Profile, include_smsp: bool, include_smsc: bool) -> Result<String, Box<dyn std::error::Error>> {
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

    profile.additional_fields.push(AdditionField {
        name: String::from("Hex encoded profile"),
        file: String::from("n/a"),
        content: to_hex(p, include_smsp, include_smsc),
    });

    let t = serde_json::to_string(&profile)?;
    Ok(t)
}

pub fn to_hex(p: &Profile, include_smsp: bool, include_smsc: bool) -> String {
    let mut ret = String::new();

    if let Some(imsi) = &p.imsi {
        let encoded_imsi = encode_imsi(imsi);
        ret.push_str(&encoded_imsi.encode_tlv(Tags::Imsi));
    }

    if let Some(iccid) = &p.iccid {
        let encoded_iccid = swap_nibbles(iccid);
        ret.push_str(&encoded_iccid.encode_tlv(Tags::Iccid));
    }

    if let Some(opc) = &p.opc {
        ret.push_str(&opc.encode_tlv(Tags::Opc));
    }

    if let Some(ki) = &p.k {
        ret.push_str(&ki.encode_tlv(Tags::Ki));
    }

    if let Some(kic) = &p.kic {
        ret.push_str(&kic.encode_tlv(Tags::Kic));
    }

    if let Some(kid) = &p.kid {
        ret.push_str(&kid.encode_tlv(Tags::Kid));
    }

    if include_smsp {
        if let Some(smsp) = &p.smsp {
            ret.push_str(&smsp.encode_tlv(Tags::Smsp));
        }
    }
    if include_smsc {
        if let Some(smsc) = &p.smsc {
            let encoded = encode_smsc(smsc);
            ret.push_str(&encoded.encode_tlv(Tags::Smsc));
        }
    }

    if let Some(pin) = &p.pin {
        let encoded_pin = hex::encode(pin.as_bytes());
        ret.push_str(&encoded_pin.encode_tlv(Tags::Pin));
    }

    if let Some(puk) = &p.puk {
        let encoded_puk = hex::encode(puk.as_bytes());
        ret.push_str(&encoded_puk.encode_tlv(Tags::Puk));
    }
    if let Some(adm) = &p.adm {
        let encoded_adm = hex::encode(adm.as_bytes());
        ret.push_str(&encoded_adm.encode_tlv(Tags::Adm));
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
    if s.len() <= 1 {
        return s.to_string();
    }
    let mut data: Vec<_> = s.chars().collect();
    for idx in (0..s.len() - 1).step_by(2) {
        data.swap(idx, idx + 1);
    }

    data.into_iter().collect()
}

fn half_round_up(n: usize) -> usize {
    n.div_ceil(2)
}

fn rpad(s: &str, l: usize, b: Option<u8>) -> String {
    let padding_len = l - s.len();
    let pad = String::from_utf8(vec![b.unwrap_or(b'f'); padding_len]).unwrap();
    format!("{}{}", s, pad)
}

fn encode_smsc(smsc: &str) -> String {
    // strip out non-digits
    let mut digits: String = smsc.chars().filter(|c| c.is_ascii_digit()).collect();

    // semi-octet length in bytes
    let digits_octets = half_round_up(digits.len());
    let length_octet = digits_octets + 1; // +1 for TON

    // pad with 'f' if odd number of digits
    if digits.len() % 2 == 1 {
        digits.push('f');
    }

    // swap nibbles representation
    let swapped = swap_nibbles(&digits);

    // build content string: length, TON 0x91 (international), swapped digits
    let mut content = format!("{:02x}91{}", length_octet, swapped);

    // pad to 12 octets (24 hex chars) with 'ff' bytes. Ensures that it match onomondo-uicc expectations
    const SMSC_CONTENT_BYTES: usize = 12;
    let current_octets = 1 + 1 + digits_octets; // length_octet included as one octet in the data
    if current_octets < SMSC_CONTENT_BYTES {
        let padding_octets = SMSC_CONTENT_BYTES - current_octets;
        for _ in 0..padding_octets {
            content.push_str("ff");
        }
    }
    content
}

trait Tlv {
    fn encode_tlv(&self, tag: Tags) -> String;
}

impl Tlv for String {
    fn encode_tlv(&self, tag: Tags) -> String {
        format!("{:02x}{:02x}{}", tag as u8, self.len(), self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpad_default() {
        assert_eq!(rpad(&String::from("abc"), 5, None), "abcff");
        assert_eq!(rpad(&String::from("abc"), 5, Some(b'e')), "abcee");
    }
    #[test]
    fn test_pin_to_ascii() {
        let pin = "1234";
        let pin_ascii = hex::encode(pin.as_bytes());

        println!("Resulting string: {}", pin_ascii);

        assert_eq!(pin_ascii, "31323334");
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
            pin: None,
            puk: None,
            adm: None,
            smsp: None,
            smsc: None,
        };

        assert_eq!(
            "080910101032540636",
            encode_imsi(p.imsi.as_deref().unwrap())
        );
        assert_eq!(
            "98001032547698103214",
            swap_nibbles(p.iccid.as_deref().unwrap())
        );
    assert_eq!(p.to_hex(true, false), "01120809101010325406360214980010325476981032140320000000000000000000000000000000000420000102030405060708090A0B0C0D0E0F0520000102030405060708090A0B0C0D0E0F0620000102030405060708090A0B0C0D0E0F")
    }

    #[test]
    fn test_smsp_flag() {
    let p = Profile {
            iccid: None,
            imsi: None,
            opc: None,
            k: None,
            kic: None,
            kid: None,
            pin: None,
            puk: None,
            adm: None,
            smsp: Some(String::from("abcd")),
            smsc: None,
        };

        // when enabled, default tag 7 should be present at start of tlv for smsp: 07 04 abcd
    let encoded_default = p.to_hex(true, false);
        assert!(encoded_default.contains("0704abcd"));

        // when disabled, smsp should not be included
    let encoded_custom = p.to_hex(false, false);
        assert!(!encoded_custom.contains("abcd"));
    }

    #[test]
    fn test_smsc_flag() {
        let p = Profile {
            iccid: None,
            imsi: None,
            opc: None,
            k: None,
            kic: None,
            kid: None,
            pin: None,
            puk: None,
            adm: None,
            smsp: None,
            smsc: Some(String::from("+447797704848")),
        };

        // when enabled, expected SMSC TLV: tag 0c length 18 hex (24) then content starting with 07 91 <swapped digits>
        let encoded_default = p.to_hex(false, true);
        assert!(encoded_default.contains("0c18"));
        assert!(encoded_default.contains("0791447779078484ffffffff"));
    }
    
    #[test]
    fn test_smsc_flag_odd_number() {
        let p = Profile {
            iccid: None,
            imsi: None,
            opc: None,
            k: None,
            kic: None,
            kid: None,
            pin: None,
            puk: None,
            adm: None,
            smsp: None,
            smsc: Some(String::from("+44779770484")),
        };

        // when enabled, expected SMSC TLV: tag 0c length 18 hex (24) then content starting with 07 91 <swapped digits>
        let encoded_default = p.to_hex(false, true);
        assert!(encoded_default.contains("0c18"));
        assert!(encoded_default.contains("07914477790784f4ffffffff"));
    }
}
