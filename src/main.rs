use ferrite_rs::{
    bencoding::{self, BencodeValue, DecodedValue, decode},
    metainfo::Metainfo,
};

fn main() {
    let raw_bytes = std::fs::read("multi-file-test.torrent").unwrap();
    let (remaining, decoded) = decode(&raw_bytes).unwrap();
    let bencode_val = decoded.value;
    let metainfo = Metainfo::decode_benvalue(bencode_val).unwrap();
    println!("{:?}", metainfo);
    println!("{:?}", remaining);
}
