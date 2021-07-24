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
use std::io::{BufRead, Read, Write};

static VALUE_MAPPINGS: [&str; 16] = [
    "uwu", "owo", "umu", "nya", "omo", "o_o", "q_p", "u_u", "o~o", "UwU", "OwO", "UmU", "OmO",
    "O_O", "U_U", "Nya",
];
thread_local! { static WORD_MAP: RefCell<HashMap<&'static str, u8>> = RefCell::new(get_word_map()); }
static SEPARATOR: &str = " ";

/// Encodes bytes from a reader to a sink writer
pub fn encode_stream<R: Read, W: Write>(src: &mut R, sink: &mut W) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];
    while let Ok(num_bytes) = src.read(&mut buf) {
        if num_bytes == 0 {
            break;
        }
        sink.write_all(encode(&buf[0..num_bytes]).as_bytes())?;
    }

    Ok(())
}

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

/// Decodes a stream of bytes into the raw data
pub fn decode_stream<R: BufRead, W: Write>(src: &mut R, sink: &mut W) -> std::io::Result<()> {
    let mut buf = String::new();
    while let Ok(num_bytes) = src.read_line(&mut buf) {
        if num_bytes == 0 {
            break;
        }
        sink.write_all(&mut decode(&buf))?;
    }

    Ok(())
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
    use crate::{decode, decode_stream, encode, encode_stream};
    use std::io::Cursor;

    #[test]
    fn it_encodes() {
        let data = vec![0u8, 16u8, 12u8];
        let encoded = encode(&data);
        assert_eq!(encoded, String::from("uwu uwu owo uwu uwu OmO"))
    }

    #[test]
    fn it_stream_encodes() {
        let data = vec![0u8, 16u8, 12u8];
        let mut output_buf = Vec::new();
        encode_stream(&mut data.as_slice(), &mut output_buf).unwrap();
        let encoded = String::from_utf8(output_buf).unwrap();
        assert_eq!(encoded, String::from("uwu uwu owo uwu uwu OmO"))
    }

    #[test]
    fn it_decodes() {
        let encoded_data = String::from("uwu uwu owo uwu uwu OmO");
        let decoded = decode(encoded_data);
        assert_eq!(decoded, vec![0u8, 16u8, 12u8])
    }

    #[test]
    fn it_stream_decodes() {
        let mut data = Cursor::new(String::from("uwu uwu owo uwu uwu OmO"));
        let mut output_buf = Vec::new();
        decode_stream(&mut data, &mut output_buf).unwrap();
        assert_eq!(output_buf, vec![0u8, 16u8, 12u8])
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
