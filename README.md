# README

## Plan

### Read through the [BitTorrent Spec](https://www.bittorrent.org/beps/bep_0003.html).

+ HTTP works by downloading from one server - that one server must upload to all clients. BitTorrent allows for downloaders upload to eachother. modest increase in load.

#### Bit Torrent File Distribution
+ Web Server
+ Metainfo File
+ BitTorrent Tracker
+ Original Downloader ? 
+ End user web browsers, end user downloaders

#### Start Serving: host
+ runs a tracker?
+ runs ordinary web server
+ .torrent with application on web server
+ gereate a metainfo (.torrent) file
+ put the .torrent on the webserver
+ link to the .torrent file from webpage
+ start a download

#### Start dwonloading
+ whatever

#### Bencoding Parsing
+ Strings are length-prefixed base ten followed by a colon and string: **EXAMPLE**: 4:spam. -> 'spam' . so just length 
+ integers are represented by an 'i' followed by the number in base 10 followed by an 'e'. i4e. i03e or any 0x is invalid. i0e is valid as it is just 0. 
+ lists are encoded with an l followed by their elements. followed by an e. l4:spam4:eggse - this is awful
+ Dictionaries are encoded as a 'd' followed by a list of alteraning keys:
+     d3:cow3:moo4:spam4:eggse => {'cow:'moo', 'spam': 'eggs'}
+     d4:spaml1:a1:bee => {'spam': ['a','b']} keys must be strings and appear in sorted order. 


#### Metainfo (.torrent) files
+ bencoded dictionaries with the keys: {anncounce: 'url of the tracker'}
+ key: {info: maps the a dictionary}
++ 
