/// simple argument parsing
/// https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html
/// 
/// Clap basic
/// https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html#parse-command-line-arguments
/// https://stackoverflow.com/questions/48068334/taking-multiple-values-in-an-argument-in-clap


extern crate clap;

use clap::{Arg, App};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn parse_arg() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let matches = App::new("Arg parser for file parser program")
        //.version("0.1.0")
        .version(VERSION)
        .author("zenanswer <zenanswer@hotmail.com>")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("dir-list")
                 .short("d")
                 .long("dirs")
                 .takes_value(true)
                 .required(true)
                 .min_values(1)
                 .help("A list of dir"))
        .get_matches();

        if let Some(dir_list) = matches.values_of("dir-list") {
            for dir in dir_list {
                println!("list: {}", dir);
            }
        }
    Err("Bad request")?
}
