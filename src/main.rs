#[macro_use]
extern crate clap;
extern crate byteorder;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

use byteorder::{ReadBytesExt, BigEndian};


use std::path::Path;
use clap::{App, Arg, ArgMatches, ArgGroup};

pub fn build_cli() -> App<'static, 'static> {
    App::new("mstore-info")
        .about("mstore information utility")
        .arg(Arg::with_name("index")
            .value_name("idx")
            .help("Index File")
            .required(true)
            .index(1))
        .arg(Arg::with_name("count").long("count").short("c"))
        .arg(Arg::with_name("offset").long("offset").short("o"))
        .arg(Arg::with_name("ppf").long("points-per-file").short("p"))
        .arg(Arg::with_name("human").long("text").short("t"))
        .group(ArgGroup::with_name("field")
            .args(&["count", "offset", "ppf", "human"])
            .required(true))

}
struct MFileIdx {
    offset: u64,
    ppf: u64,
    count: u64,
}

fn read_idx(idx: &Path) -> Result<MFileIdx, std::io::Error> {
    // Why does try not work?!?
    let mut count = 0;
    let file = try!(File::open(idx));
    let mut buffer = BufReader::new(file);
    let offset = try!(buffer.read_u64::<BigEndian>());
    let ppf = try!(buffer.read_u64::<BigEndian>());
    while let Ok(size) = buffer.by_ref().read_u16::<BigEndian>() {
        count = count + 1;
        try!(buffer.by_ref().seek(SeekFrom::Current(size as i64)));
    }
    return Ok(MFileIdx {
        offset: offset,
        ppf: ppf,
        count: count,
    });
}


fn print_index(idx: &MFileIdx, matches: &ArgMatches) {
    if matches.is_present("count") {
        println!("{}", idx.count)
    } else if matches.is_present("offset") {
        println!("{}", idx.offset)
    } else if matches.is_present("ppf") {
        println!("{}", idx.ppf)
    } else if matches.is_present("human") {
        let name = value_t!(matches, "index", String).unwrap();
        println!("statistics on: {}", name);
        println!("  offset:          {}", idx.offset);
        println!("  points per file: {}", idx.ppf);
        println!("  metric count:    {}", idx.count);
    }
}

fn main() {
    let matches = build_cli().get_matches();
    let base = value_t!(matches, "index", String).unwrap();
    let idx = Path::new(&base);
    //let idx = path.with_extension("idx");
    //let mstore = path.with_extension("mstore");
    match read_idx(idx) {
        Ok(idx) => print_index(&idx, &matches),
        Err(_) => println!("illegal index"),
    }
}