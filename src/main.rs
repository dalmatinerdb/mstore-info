#[macro_use]
extern crate clap;
extern crate byteorder;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

use byteorder::{ReadBytesExt, BigEndian};


use std::path::Path;
use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("mstore-info")
        .about("mstore information utility")
        .arg(Arg::with_name("index")
            .value_name("idx")
            .help("Index File")
            //.default_value("example/0")
            .required(true)
            .index(1))
}

pub fn read_idx(idx: &Path) -> Result<u64, std::io::Error> {
    // Why does try not work?!?
    let mut count = 0;
    let file = try!(File::open(idx));
    let mut buffer = BufReader::new(file);
    let offset = try!(buffer.read_u64::<BigEndian>());
    let file_size = try!(buffer.read_u64::<BigEndian>());
    println!("offset: {:?}", offset);
    println!("file_size: {:?}", file_size);
    while let Ok(size) = buffer.by_ref().read_u16::<BigEndian>() {
        count = count + 1;
        try!(buffer.by_ref().seek(SeekFrom::Current(size as i64)));
    }
    

    return Ok(count);
}

fn main() {
    let matches = build_cli().get_matches();
    let base = value_t!(matches, "index", String).unwrap();
    let idx = Path::new(&base);
    //let idx = path.with_extension("idx");
    //let mstore = path.with_extension("mstore");
    match read_idx(idx) {
        Ok(count) => println!("elements: {:?}", count),
        Err(_) => println!("illegal index")
    }
}