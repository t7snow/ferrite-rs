use std::collections::BTreeMap;

enum BencodeValue {
    Str(Vec<u8>),
    Int(i64),
    List(Vec<BencodeValue>),
    Dict(BTreeMap<BencodeValue, BencodeValue>),
}
pub fn decode(raw: &[u8]) -> Result<(&[u8], BencodeValue), String> {
    //String - 3:spam
    //Integer - i3e
    //List l4:spam3eggse
    //Dictionary - d3:cow3:moo4:spame
    //
    match raw[0] {
        // first, iterate through row and find the e at the end of integer. grab that index.
        // from 1->end pull the actual number , unwrap it.
        // return the rest of the slice  and the new bencodevalue.
        b'i' => {
            let end = raw.iter().position(|&b| b == b'e').unwrap();
            let n: i64 = std::str::from_utf8(&raw[1..end]).unwrap().parse().unwrap();
            Ok((&raw[end + 1..], BencodeValue::Int(n)))
        }
        b'0'..=b'9' => {
            let start = raw.iter().position(|&b| b == b':').unwrap();
            let length = std::str::from_utf8(&raw[0..start])
                .unwrap()
                .parse()
                .unwrap();
            let string = raw[(start + 1)..length].to_vec();
            Ok((
                &raw[(start + 1)..(start + 1 + length)],
                BencodeValue::Str(string),
            ))
        }
        _ => Ok((&[], BencodeValue::Int(0))), // catch-all placeholder
    }
}
