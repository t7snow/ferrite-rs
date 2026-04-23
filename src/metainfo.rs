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

enum PeerMessageType {
    Choke,
    Unchoke, 
    Interested,
    Not_Interested,
    Have,
    Bitfield,
    Request,
    Piece,
    Cancel,
}
}
struct Metainfo {
    announce: String,
    info: Info,
    trackers: Tracker,
}
