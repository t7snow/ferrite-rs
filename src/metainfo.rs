use std::collections::{BTreeMap, HashMap};
use std::error;
use std::net::Ipv4Addr;

use crate::bencoding::BencodeValue;
pub struct Info {
    pub name: String,
    //number of bytes in each piece the file is split into. fixed length sizesalwaqys power of two.
    pub piece_length: i64,
    //maps to a string whose length is a multiple of 20. strings of length 20.
    pub pieces: String,
    pub length: i64,
    pub path: Vec<String>,
    //single file by concat the files in the order they appear in this list. files list is the
    //value files maps to, and is a list of dictionaries containing the following keys:
    //length - length of file in bytes
    //path a list of strings, last of which is the actual file name.
    //in the single file case the name key is the name of the file
    pub files: Option<Vec<HashMap<usize, Vec<String>>>>,
}

pub enum TrackerEventType {
    Started,
    Completed,
    Stopped,
    Empty,
}
pub struct Tracker {
    info_hash: usize,
    peer_id: String,
    ip: Ipv4Addr,
    port: u16,
    uploaded: char,
    downloaded: char,
    left: usize,
    event: TrackerEventType,
}

pub struct Metainfo {
    announce: String,
    info: Info,
}

impl Metainfo {
    pub fn new(announce: String, info: Info, tracker: Tracker) -> Self {
        Self { announce, info }
    }
    //bencoding
    // take in a vector of raw bytes, or a slice of raw bytes. and a metainfo struct
    // Create a MetaInfo struct, the metainfo struct is continually added to and returned back to
    // the funciton until nothing in the slidce of raw bytes matches anymore.l that means we are a
    //t the end. and then return simply the MetaINfo struct
    pub fn deocde_benvalue(bencode_value: BencodeValue) -> Result<Metainfo, String> {
        match bencode_value {
            BencodeValue::Dict(map) => {
                let info = map.get(&b"info".to_vec()).unwrap();

                match info {
                    BencodeValue::Dict(info_map) => {
                        let announce = match map.get(&b"announce".to_vec()).unwrap() {
                            BencodeValue::Str(s) => String::from_utf8(s.clone()).unwrap(),
                            _ => return Err("announce not a string".to_string()),
                        };
                        let name = match info_map.get(&b"name".to_vec()).unwrap() {
                            BencodeValue::Str(s) => String::from_utf8(s.clone()).unwrap(),
                            _ => return Err("name not a string".to_string()),
                        };
                        let piece_length = match info_map.get(&b"piece length".to_vec()).unwrap() {
                            BencodeValue::Int(i) => i,
                            _ => return Err("piece_length not an i64".to_string()),
                        };
                        let pieces = match info_map.get(&b"pieces".to_vec()).unwrap() {
                            BencodeValue::Str(s) => String::from_utf8(s.clone()).unwrap(),
                            _ => return Err("pieces not a string".to_string()),
                        };
                        let length = match info_map.get(&b"length".to_vec()).unwrap() {
                            BencodeValue::Int(i) => i,
                            _ => return Err("length not an i64".to_string()).unwrap(),
                        };
                        let path = match info_map.get(&b"path".to_vec()).unwrap() {
                            BencodeValue::List(l) => l
                                .iter()
                                .map(|x| match x {
                                    BencodeValue::Str(s) => String::from_utf8(s.clone()).unwrap(),
                                    _ => panic!("path element not a string"),
                                })
                                .collect::<Vec<String>>(),
                            _ => return Err("path not a list".to_string()),
                        };

                        Ok(Self {
                            announce,
                            info: Info {
                                name,
                                piece_length: *piece_length,
                                pieces,
                                length: *length,
                                path,
                                files: None,
                            },
                        })
                    }
                    _ => Err("string".to_string()),
                }
            }
            _ => Err("string".to_string()),
        }
    }
}
