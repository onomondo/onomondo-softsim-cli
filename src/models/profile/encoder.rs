#![allow(unused)]

use super::Profile;

enum Tags {
    IMSI = 1,
    ICCID = 2,
    OPC = 3,
    KI = 4,
    KIC = 5,
    KID = 6,
    END = 0xff,
}

pub fn profile_to_hex(p: Profile) -> String {
    let mut ret = String::new();
    if let Some(imsi) = p.imsi {
        let encoded_imsi = encode_imsi(imsi);
        ret.push_str(
            format!(
                "{:02x}{:02x}{}",
                encoded_imsi.len(),
                Tags::IMSI as u8,
                encoded_imsi
            )
            .as_str(),
        );
    }
    ret
}

fn encode_imsi(imsi: String) -> String {
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

fn swap_nibbles(s: &String) -> String {
    let mut data: Vec<_> = s.chars().collect();
    for idx in (0..s.len() - 1).step_by(2) {
        data.swap(idx, idx + 1);
    }

    data.into_iter().collect()
}

fn half_round_up(n: usize) -> usize {
    (n + 1) / 2
}

fn rpad(s: String, l: usize, b: Option<u8>) -> String {
    let padding_len = l - s.len();
    let pad = String::from_utf8(vec![b.unwrap_or_else(|| b'f'); padding_len]).unwrap();
    format!("{}{}", s, pad)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpad_default() {
        assert_eq!(rpad(String::from("abc"), 5, None), "abcff");

        assert_eq!(rpad(String::from("abc"), 5, Some(b'e')), "abcee");
    }

    #[test]
    fn test_swap_nibbles() {
        assert_eq!(swap_nibbles(&String::from("1234")), "2143");
        assert_eq!(swap_nibbles(&String::from("1")), "1");
        assert_eq!(
            swap_nibbles(&String::from("1234567890abcdef")),
            "2143658709badcfe"
        );
    }

    #[test]
    fn test_imsi_encoder() {
        assert_eq!(
            encode_imsi(String::from("234602102350049")),
            "082943061220530094"
        );

        assert_eq!(
            encode_imsi(String::from("234602102349958")),
            "082943061220439985"
        );
    }

    #[test]
    fn test_encode_iccid() {
        assert_eq!(
            swap_nibbles(&String::from("89457300000013500452")),
            "98543700000031054025"
        )
    }
}
