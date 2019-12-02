use std::fs;
use std::io::{prelude::*, BufReader};

mod argparser;
mod diriterater;
mod lineconverter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dirs = argparser::parse_arg()?;
    println!("Got dirs: {:#?}", dirs);
    // for dir in dirs {
    //     diriterater::iterate_dir(&dir, lineconverter::convert_line)?;
    // }
    let results: Vec<i32> = dirs
        .iter()
        .filter_map(|root_dir| fs::read_dir(root_dir).ok())
        .flat_map(|sub_dir| sub_dir.filter_map(|x| x.ok()))
        .filter_map(|sub_dir| {
            println!("file: {}", sub_dir.path().display());
            fs::File::open(sub_dir.path()).ok()
        })
        .flat_map(|file| BufReader::new(file).lines())
        .filter_map(|line_result| {
            let line = line_result.unwrap();
            println!("line: {}", line);
            line.trim().parse::<i32>().ok()
        })
        .collect();

    println!("Results: {:#?}", results);

    Ok(())
}
