use std::io::{BufWriter, Write};
use std::fs::{File};
extern crate clap;
use clap::{Arg, App, ArgMatches};

extern crate blobber;

fn main() {
    let matches = get_matches();
    let size_str = matches.value_of("size").unwrap_or("1m");
    let size = parse_file_size(size_str);
    let path = matches.value_of("outfile").unwrap_or("out.txt");
    println!("output: {}", path);
    let file = File::create(path).expect("Unable to create file");
    let mut w_buf = BufWriter::new(file);
    let lorem = blobber::get_lorem(size, true);
    println!("actual output size: {}", get_file_size(lorem.as_bytes().len()));
    w_buf.write_all(lorem.as_bytes()).expect("write failed");
}

fn get_file_size(size: usize) -> String {
    let mut counter = 0;
    let mut updated_size = size as f32;
    while updated_size >= 1024.0 {
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
                .takes_value(true)
                .required(false))
            .arg(Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .required(false))
            .get_matches()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_to_text() {
        let bytes = get_file_size(10);
        assert_eq!(bytes, "10.00 bytes".to_string());
        let kilobytes = get_file_size(1024);
        assert_eq!(kilobytes, "1.00 kilobytes".to_string());
        let mega = get_file_size(1024 * 1024);
        assert_eq!(mega, "1.00 megabytes".to_string());
        let gig = get_file_size(1024 * 1024 * 1024);
        assert_eq!(gig, "1.00 gigabytes".to_string());
        let tera = get_file_size(1024 * 1024 * 1024 * 1024);
        assert_eq!(tera, "1.00 terabytes".to_string());
        let peta = get_file_size(1024 * 1024 * 1024 * 1024 * 1024);
        assert_eq!(peta, "1.00 petabytes".to_string());
    }

    #[test]
    fn text_to_number() {
        let bytes = parse_file_size("10");
        assert_eq!(bytes, 10);
        let kilo = parse_file_size("1k");
        assert_eq!(kilo, 1024);
        let meg = parse_file_size("1m");
        assert_eq!(meg, 1024 * 1024);
        let gig = parse_file_size("1g");
        assert_eq!(gig, 1024 * 1024 * 1024);
    }
}