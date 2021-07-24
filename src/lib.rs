//! This crate provides an encoding from byte to uwu and back.
//! Encode:
//! ```
//! use uwucodec::encode;
//!
//! fn main() {
//!     println!("Hello World in uwu is: {}", encode("Hello World".as_bytes()));
//! }
//! ```
//!
//! Decode:
//! ```
//! use uwucodec::decode;
//! fn main() {
//!     let text = "omo o~o q_p o_o q_p OmO q_p OmO q_p Nya umu uwu o_o u_u q_p Nya u_u umu q_p OmO q_p omo";
//!     println!("{} is {:?}", text, decode(text));
//! }
//! ```

use std::cell::RefCell;
use std::collections::HashMap;

static VALUE_MAPPINGS: [&str; 16] = [
    "uwu", "owo", "umu", "nya", "omo", "o_o", "q_p", "u_u", "o~o", "UwU", "OwO", "UmU", "OmO",
    "O_O", "U_U", "Nya",
];
thread_local! { static WORD_MAP: RefCell<HashMap<&'static str, u8>> = RefCell::new(get_word_map()); }
static SEPARATOR: &str = " ";

/// Encodes into the best encoding in existence
pub fn encode<'a, I: IntoIterator<Item = &'a u8> + 'a>(data: I) -> String {
    data.into_iter()
        .map(|b| encode_byte(*b))
        .flatten()
        .collect::<Vec<&'static str>>()
        .join(SEPARATOR)
}

fn encode_byte(byte: u8) -> [&'static str; 2] {
    [
        VALUE_MAPPINGS[(byte >> 4) as usize],
        VALUE_MAPPINGS[(byte & 15) as usize],
    ]
}

/// Decodes the best encoding in existence back into bytes
pub fn decode<S: AsRef<str>>(encoded_data: S) -> Vec<u8> {
    let mut data = Vec::new();
    let mut words = encoded_data.as_ref().split(SEPARATOR);
    loop {
        let byte = if let (Some(head), Some(tail)) = (words.next(), words.next()) {
            [head, tail]
        } else {
            break;
        };
        data.push(decode_byte(byte))
    }

    data
}

fn decode_byte(enc_byte: [&str; 2]) -> u8 {
    (decode_word(enc_byte[0]) << 4) | decode_word(enc_byte[1])
}

fn decode_word(word: &str) -> u8 {
    WORD_MAP.with(|m| *m.borrow().get(word).unwrap_or(&0u8))
}

fn get_word_map() -> HashMap<&'static str, u8> {
    let mut value_map = HashMap::with_capacity(VALUE_MAPPINGS.len());
    for i in 0..VALUE_MAPPINGS.len() {
        value_map.insert(VALUE_MAPPINGS[i], i as u8);
    }

    value_map
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    #[test]
    fn it_encodes() {
        let data = vec![0u8, 16u8, 12u8];
        let encoded = encode(&data);
        assert_eq!(encoded, String::from("uwu uwu owo uwu uwu OmO"))
    }

    #[test]
    fn it_decodes() {
        let encoded_data = String::from("uwu uwu owo uwu uwu OmO");
        let decoded = decode(encoded_data);
        assert_eq!(decoded, vec![0u8, 16u8, 12u8])
    }

    #[test]
    fn it_encodes_100000() {
        let data = vec![0u8, 16u8, 12u8];
        for _ in 0..100000 {
            let encoded = encode(&data);
            assert_eq!(encoded, String::from("uwu uwu owo uwu uwu OmO"))
        }
    }

    #[test]
    fn it_decodes_100000() {
        let encoded_data = String::from("uwu uwu owo uwu uwu OmO");
        for _ in 0..100000 {
            let decoded = decode(&encoded_data);
            assert_eq!(decoded, vec![0u8, 16u8, 12u8])
        }
    }
}
