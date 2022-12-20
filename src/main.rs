use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};
fn main() {

    let args = App::new("grep-lite")
                    .version("0.1")
                    .about("Searches for patterns")
                    .arg(Arg::with_name("pattern")
                        .help("The pattern to search for")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("input")
                        .help("File to search")
                        .takes_value(true)
                        .required(false))
                    .get_matches();


   let pattern = args.value_of("pattern").expect("Issue creating regex expression");
    let re = Regex::new(pattern).expect("Issue creating regex pattern");

   let quote = "Every face, every shop, bedroom window, public-house, and
   dark square is a picture feverishly turned--in search of what?
   It is the same with books. What do we seek through millions of pages?";

   let input = args.value_of("input").unwrap_or("-");

   if input == "-" {
    let stdin = io::stdin();
    let reader = stdin.lock();
    process_lines(reader, re);
   } else {
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    process_lines(reader, re);
   }

}

// process lines using a BufReader with a known size and a regex pattern
fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();

        match re.find(&line) {
            Some(_) => println!("{line} -- line {i}"),
            None => {},
        }
    }
}