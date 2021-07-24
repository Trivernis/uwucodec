# UWU-Codec

I'm sorry.

## Motivation

Do you also think hexadecimal byte encoding is boring? Do you want more uwu in your life?
This program solves both of your problems. Bytes are encoded into two words according to
the following mapping:

| nibble | word |
|--------|------|
| 0000 | uwu
| 0001 | owo
| 0010 | umu
| 0011 | nya
| 0100 | omo
| 0101 | o_o
| 0110 | q_p
| 0111 | u_u
| 1000 | o~o
| 1001 | UwU
| 1010 | OwO
| 1011 | UmU
| 1100 | OmO
| 1101 | O_O
| 1110 | U_U
| 1111 | Nya


## Usage

Use the `uwuencode` and `uwudecode` binaries like this:

```
uwuencode path/to/raw/input path/to/encoded/output
uwudecode path/to/encoded/input path/to/raw/output
```

You can also use the functions in your project to spice things up:

```rust
use uwucodec::encode;
fn main() {
    let encoded = encode(&"Hello World".as_bytes());
    let decoded = decode(&encoded);
}
```

## License

Apache-2.0