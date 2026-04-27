use std::collections::HashMap;
use std::ops::Range;

use sha1::{Digest, Sha1};

#[derive(Debug, PartialEq, Eq)]
pub struct DecodedValue {
    pub value: BencodeValue,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BencodeValue {
    Str(Vec<u8>),
    Int(i64),
    List(Vec<DecodedValue>),
    Dict(HashMap<Vec<u8>, DecodedValue>), // key is Vec<u8>, not BencodeValue
    End,
}

pub fn info_hash(raw: &[u8]) -> Result<[u8; 20], String> {
    let (_, decoded) = decode(raw)?;
    let info_span = match decoded.value {
        BencodeValue::Dict(map) => map
            .get(&b"info".to_vec())
            .map(|value| value.span.clone())
            .ok_or_else(|| "top-level dictionary does not contain info".to_string())?,
        _ => return Err("torrent must decode to a top-level dictionary".to_string()),
    };

    let digest = Sha1::digest(&raw[info_span]);
    Ok(digest.into())
}

pub fn decode(raw: &[u8]) -> Result<(&[u8], DecodedValue), String> {
    decode_at(raw, 0)
}

fn decode_at(raw: &[u8], offset: usize) -> Result<(&[u8], DecodedValue), String> {
    //String - 3:spam
    //Integer - i3e
    //List l4:spam3eggse
    //Dictionary - d3:cow3:moo4:spame
    //
    if raw.is_empty() {
        return Err("raw is empty".to_string());
    }
    match raw[0] {
        // first, iterate through row and find the e at the end of integer. grab that index.
        // from 1->end pull the actual number , unwrap it.
        // return the rest of the slice  and the new bencodevalue.
        b'i' => {
            let end = raw.iter().position(|&b| b == b'e').unwrap();
            let n: i64 = std::str::from_utf8(&raw[1..end]).unwrap().parse().unwrap();
            let span_end = offset + end + 1;
            Ok((
                &raw[end + 1..],
                DecodedValue {
                    value: BencodeValue::Int(n),
                    span: offset..span_end,
                },
            ))
        }
        b'0'..=b'9' => {
            let start = raw.iter().position(|&b| b == b':').unwrap();
            let length: usize = std::str::from_utf8(&raw[0..start])
                .unwrap()
                .parse()
                .unwrap();
            let string = raw[(start + 1)..(start + 1 + length)].to_vec();
            let span_end = offset + start + 1 + length;
            Ok((
                &raw[(start + 1 + length)..],
                DecodedValue {
                    value: BencodeValue::Str(string),
                    span: offset..span_end,
                },
            ))
        }
        b'l' => {
            let mut items = Vec::new();
            let mut remaining = &raw[1..];
            let mut remaining_offset = offset + 1;
            loop {
                let (rest, val) = decode_at(remaining, remaining_offset)?;
                if val.value == BencodeValue::End {
                    return Ok((
                        rest,
                        DecodedValue {
                            value: BencodeValue::List(items),
                            span: offset..val.span.end,
                        },
                    ));
                }
                items.push(val);
                remaining = rest;
                remaining_offset = offset + (raw.len() - remaining.len());
            }
        }
        b'd' => {
            let mut k = Vec::new();
            let mut map = HashMap::new();
            let mut remaining = &raw[1..];
            let mut remaining_offset = offset + 1;
            let mut count = 0;
            loop {
                let (rest, val) = decode_at(remaining, remaining_offset)?;
                if val.value == BencodeValue::End {
                    return Ok((
                        rest,
                        DecodedValue {
                            value: BencodeValue::Dict(map),
                            span: offset..val.span.end,
                        },
                    ));
                }
                if count % 2 == 0 {
                    if let BencodeValue::Str(bytes) = val.value {
                        k = bytes;
                    }
                } else {
                    map.insert(k.clone(), val);
                }
                count += 1;
                remaining = rest;
                remaining_offset = offset + (raw.len() - remaining.len());
            }
        }
        b'e' => Ok((
            &raw[1..],
            DecodedValue {
                value: BencodeValue::End,
                span: offset..(offset + 1),
            },
        )),
        _ => Ok((
            &[],
            DecodedValue {
                value: BencodeValue::Int(0),
                span: offset..offset,
            },
        )), // catch-all placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::{BencodeValue, decode, info_hash};

    #[test]
    fn records_the_info_span_in_the_decoded_dictionary() {
        let raw = b"d8:announce11:tracker-url4:infod4:name4:spamee";
        let (_, decoded) = decode(raw).unwrap();

        let info_span = match decoded.value {
            BencodeValue::Dict(map) => map.get(&b"info".to_vec()).unwrap().span.clone(),
            _ => panic!("expected top-level dictionary"),
        };

        assert_eq!(&raw[info_span], b"d4:name4:spame");
    }

    #[test]
    fn info_hash_uses_the_recorded_info_span() {
        let raw = b"d8:announce11:tracker-url4:infod4:name4:spamee";
        let expected = [
            0x33, 0x1b, 0xae, 0x21, 0x6f, 0x9b, 0x2a, 0xac, 0x99, 0x7d, 0xf5, 0x4f, 0x9b, 0x27,
            0x0f, 0x6a, 0x76, 0x33, 0xf6, 0xb1,
        ];

        assert_eq!(info_hash(raw).unwrap(), expected);
    }
}
