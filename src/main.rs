#[macro_use]
extern crate clap;
extern crate byteorder;

use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

use byteorder::{ReadBytesExt, BigEndian};


use std::path::Path;
use clap::{App, Arg, ArgMatches, ArgGroup};

pub fn build_cli() -> App<'static, 'static> {
    App::new("mstore-info")
        .about("mstore information utility")
        .arg(Arg::with_name("files")
            .value_name("files")
            .help("Index File")
            .multiple(true)
            .required(true)
            .index(1))
        .arg(Arg::with_name("sum")
            .help("Show the total sum of metrics.")
            .long("sum")
            .short("s"))
        .arg(Arg::with_name("count")
            .help("Shows the count of metrics in each file.")
            .long("count")
            .short("c"))
        .arg(Arg::with_name("offset")
            .help("Shows the offset of each file.")
            .long("offset")
            .short("o"))
        .arg(Arg::with_name("ppf")
            .help("Show the points per file for each file")
            .long("points-per-file")
            .short("p"))
        .arg(Arg::with_name("list")
            .help("Show the points per file for each file")
            .long("list")
            .short("l"))
        .group(ArgGroup::with_name("field").args(&["count", "offset", "ppf", "sum"]))

}
struct MFileIdx {
    offset: u64,
    ppf: u64,
    count: u64,
}

fn read_metric<T: Read>(buffer: &mut T) -> Result<String, std::io::Error> {
    let mut metric = "(".to_string();
    while let Ok(size) = buffer.by_ref().read_u8() {
        let mut section = buffer.by_ref().take(size as u64);
        let mut part = String::new();
        try!(section.read_to_string(&mut part));
        part = part.replace("\\", "\\\\").replace("'", "\\'");
        if metric.len() != 1 {
            metric += " ";
        }
        metric = metric + "'" + &part + "'";
    }
    metric += ")";
    return Ok(metric);
}

fn read_idx(idx: &Path, matches: &ArgMatches) -> Result<MFileIdx, std::io::Error> {
    // Why does try not work?!?
    let mut count = 0;
    let file = try!(File::open(idx));
    let mut buffer = BufReader::new(file);
    let offset = try!(buffer.read_u64::<BigEndian>());
    let ppf = try!(buffer.read_u64::<BigEndian>());
    let print_list = matches.is_present("list");
    while let Ok(size) = buffer.by_ref().read_u16::<BigEndian>() {
        count = count + 1;
        if print_list {
            let mut part = buffer.by_ref().take(size as u64);
            let metric = try!(read_metric(&mut part));
            println!("{}", metric);
        } else {
            try!(buffer.by_ref().seek(SeekFrom::Current(size as i64)));
        }
    }
    return Ok(MFileIdx {
        offset: offset,
        ppf: ppf,
        count: count,
    });
}


fn print_index(name: &str, idx: &MFileIdx, matches: &ArgMatches) {
    if matches.is_present("count") {
        println!("{}", idx.count);
    } else if matches.is_present("offset") {
        println!("{}", idx.offset);
    } else if matches.is_present("ppf") {
        println!("{}", idx.ppf);
    } else if !matches.is_present("list") {
        println!("statistics on: {}", name);
        println!("  offset:          {}", idx.offset);
        println!("  points per file: {}", idx.ppf);
        println!("  metric count:    {}", idx.count);
    }
}

fn main() {
    let matches = build_cli().get_matches();
    //println!("{:?}", matches);
    let files = matches.values_of("files").unwrap().collect::<Vec<_>>();
    let mut sum = 0;
    let mut count = 0;
    let use_sum = matches.is_present("sum");
    let print_human = !(matches.is_present("count") || matches.is_present("offset") ||
                        matches.is_present("ppf") ||
                        matches.is_present("sum") ||
                        matches.is_present("list"));

    for base in files {
        let idx = Path::new(&base);
        count = count + 1;
        //let idx = path.with_extension("idx");
        //let mstore = path.with_extension("mstore");
        match read_idx(idx, &matches) {
            Ok(idx) => {
                if !use_sum {
                    print_index(&base, &idx, &matches);
                };
                sum = sum + idx.count;
            }
            Err(_) => println!("illegal index: {}", base),
        }
    }
    if use_sum {
        println!("{}", sum);
    }
    if print_human && count > 1 {
        println!("\ntotal:");
        println!("  file count:      {}", count);
        println!("  metric count:    {}", sum);
    }

}
