use std::io::{BufWriter, Write};
use std::fs::{File};
extern crate clap;
use clap::{Arg, App, ArgMatches};

fn main() {
    let matches = get_matches();
    let size_str = matches.value_of("size").expect("size is required");
    let size = parse_file_size(size_str);
    let path = matches.value_of("outfile").expect("outfile is require");
    println!("output: {}", path);
    let file = File::create(path).expect("Unable to create file");
    let mut w_buf = BufWriter::new(file);
    let lorem = "0. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let whole_blocks = size / lorem.as_bytes().len();
    println!("actual output size: {}", get_file_size((whole_blocks + 1) * lorem.as_bytes().len()));
    for i in 0..(whole_blocks + 1) {
        lorem.replace("0. ", format!("{}. ", i).as_str());
        w_buf.write_all(lorem.as_bytes()).expect("write failed");
    }
}

fn get_file_size(size: usize) -> String {
    let mut counter = 0;
    let mut updated_size = size as f32;
    while updated_size > 1024.0 {
        updated_size /= 1024.0;
        counter += 1;
    }
    match counter {
        0 => format!("{:.2} bytes", updated_size),
        1 => format!("{:.2} kilobytes", updated_size),
        2 => format!("{:.2} megabytes", updated_size),
        3 => format!("{:.2} gigabytes", updated_size),
        4 => format!("{:.2} terabytes", updated_size),
        5 => format!("{:.2} petabytes", updated_size),
        _ => panic!("error parsing size")
    }
}

fn parse_file_size(arg: &str) -> usize {
    let lowered = arg.to_lowercase();
    if lowered.ends_with('k') {
        let trimmed = arg.get(0..arg.len() - 1).expect("unknown size");
        trimmed.parse::<usize>().expect("unknow size") * 1024
    } else if lowered.ends_with('m') {
        let trimmed = arg.get(0..arg.len() - 1).expect("unknown size");
        trimmed.parse::<usize>().expect("unknown size") * 1024 * 1024
    } else if lowered.ends_with('g') {
        let trimmed = arg.get(0..arg.len() - 1).expect("unknown size");
        trimmed.parse::<usize>().expect("unknown size") * 1024 * 1024 * 1024
    } else {
        arg.parse::<usize>().expect("unknown size")
    }
}

fn get_matches() ->  ArgMatches<'static> {
    App::new("dd_stat")
            .version("0.1.0")
            .arg(Arg::with_name("size")
                .short("s")
                .long("size")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .required(true)
                .takes_value(true))
            .get_matches()
}