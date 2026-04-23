use std::collections::{BTreeMap, HashMap};
use std::net::Ipv4Addr;
struct Info {
    //number of bytes in each piece the file is split into. fixed length sizesalwaqys power of two.
    pub piece_length: usize,
    //maps to a string whose length is a multiple of 20. strings of length 20.
    pub pieces: String,
    pub length: usize,
    //single file by concat the files in the order they appear in this list. files list is the
    //value files maps to, and is a list of dictionaries containing the following keys:
    //length - length of file in bytes
    //path a list of strings, last of which is the actual file name.
    //in the single file case the name key is the name of the file
    pub files: Vec<HashMap<usize, Vec<String>>>,
}

enum TrackerEventType {
    Started,
    Completed,
    Stopped,
    Empty,
}
struct Tracker {
    info_hash: usize,
    peer_id: String,
    ip: Ipv4Addr,
    port: u16,
    uploaded: char,
    downloaded: char,
    left: usize,
    event: TrackerEventType,
}

struct Metainfo {
    announce: String,
    info: Info,
    tracker: Tracker,
}

impl Metainfo {
    pub fn new(announce: String, info: Info, tracker: Tracker) -> Self {
        Self {
            announce,
            info,
            tracker,
        }
    }
    //bencoding
    // take in a vector of raw bytes, or a slice of raw bytes. and a metainfo struct
    // Create a MetaInfo struct, the metainfo struct is continually added to and returned back to
    // the funciton until nothing in the slidce of raw bytes matches anymore.l that means we are a
    //t the end. and then return simply the MetaINfo struct
    //
}
