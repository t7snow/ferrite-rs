use std::collections::BTreeMap;

enum BencodeValue {
    Str(Vec<u8>),
    Int(u8),
    List(Vec<BencodeValue>),
    Dict(BTreeMap<BencodeValue, BencodeValue>),
}
pub fn decode(raw: &[u8]) -> (&[u8], BencodeValue) {}
