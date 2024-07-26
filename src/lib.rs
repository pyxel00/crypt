use std::{cmp, isize, usize};

const HEX_TABLE: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];
const BASE64_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

const E_ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub struct Crypt;

impl Crypt {
    pub fn encode_16(input: &String) -> String {
        let mut encoded = String::new();

        let vec = Self::split(&Self::str_to_binary(input), 4);

        for binary in vec {
            match usize::from_str_radix(&binary, 2) {
                Ok(decimal) => encoded.push(HEX_TABLE[decimal]),
                Err(why) => eprintln!("Couldn't parse {binary} to usize : {why}"),
            }
        }

        encoded
    }

    pub fn decode_16(input: &String) -> String {
        let mut decoded = String::new();
        let mut binary = String::new();

        for ch in input.chars() {
            let decimal = match HEX_TABLE.iter().position(|c| *c == ch) {
                Some(d) => d,
                None => {
                    println!("Cannot find the given input in the hex table.");
                    continue;
                }
            };
            binary.push_str(&format!("{:04b}", decimal));
        }

        let vec = Self::split(&binary, 8);

        for item in vec {
            let decimal = u8::from_str_radix(&item, 2).unwrap();
            decoded.push(decimal as char);
        }

        decoded
    }

    pub fn encode_64(input: &String) -> String {
        let mut encoded = String::new();
        let mut padding = String::new();

        let vec = Self::split(&Self::str_to_binary(input), 6);
        for mut binary in vec {
            while binary.len() % 6 != 0 {
                binary.push_str("00");
                padding.push('=');
            }

            match usize::from_str_radix(&binary, 2) {
                Ok(decimal) => encoded.push(BASE64_TABLE[decimal]),
                Err(why) => eprintln!("Couldn't parse {binary} to usize : {why}"),
            };
        }

        encoded + &padding
    }
    pub fn decode_64(input: &String) -> String {
        let un_padded: String = input.chars().filter(|c| *c != '=').collect();
        let mut decoded = String::new();
        let mut binary = String::new();

        for ch in un_padded.chars() {
            match BASE64_TABLE.iter().position(|c| *c == ch) {
                Some(pos) => binary.push_str(&format!("{:06b}", pos)),
                None => eprintln!("Couldn't find {ch} in base64 table."),
            };
        }

        let vec = Self::split(&binary, 8);

        for item in vec {
            match u8::from_str_radix(&item, 2) {
                Ok(decimal) => decoded.push(decimal as char),
                Err(why) => eprintln!("Couldn't parse {binary} to usize : {why}"),
            }
        }

        decoded
    }

    pub fn encode_rot(input: &str) -> String {
        let mut encoded = String::new();

        for char in input.chars() {
            let mut decimal = match E_ALPHABET
                .iter()
                .position(|c| *c == char.to_ascii_lowercase())
            {
                Some(d) => d,
                None => {
                    eprintln!("Couldn' find {char} in the english alphabet");
                    continue;
                }
            };

            decimal += 13;

            if decimal >= E_ALPHABET.len() {
                decimal = decimal % E_ALPHABET.len();
            }

            if char.is_uppercase() {
                encoded.push(E_ALPHABET[decimal].to_ascii_uppercase());
            } else {
                encoded.push(E_ALPHABET[decimal]);
            }
        }

        encoded
    }

    pub fn decode_rot(input: &str) -> String {
        let mut encoded = String::new();

        for char in input.chars() {
            let mut decimal: isize = match E_ALPHABET
                .iter()
                .position(|c| *c == char.to_ascii_lowercase())
            {
                Some(d) => d as isize,
                None => {
                    eprintln!("Couldn' find {char} in the english alphabet");
                    continue;
                }
            };

            match decimal - 13 < 0 {
                true => decimal += 13,
                false => decimal -= 13,
            }

            if decimal >= E_ALPHABET.len() as isize {
                decimal = decimal % E_ALPHABET.len() as isize;
            }

            if char.is_uppercase() {
                encoded.push(E_ALPHABET[decimal as usize].to_ascii_uppercase());
            } else {
                encoded.push(E_ALPHABET[decimal as usize]);
            }
        }

        encoded
    }

    fn str_to_binary(input: &String) -> String {
        let mut binary = String::new();

        for char in input.chars() {
            binary.push_str(&format!("{:08b}", char as u8))
        }

        binary
    }

    fn split(input: &String, at: usize) -> Vec<String> {
        let mut last = Vec::new();
        let mut i: usize = 0;

        while i != input.len() {
            let n_point = cmp::min(i + at, input.len());
            last.push(input[i..n_point].to_string());
            i = n_point;
        }

        last
    }
}
