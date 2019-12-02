use std::fs;
use std::io::{prelude::*, BufReader};

pub fn iterate_dir(dir: &String)-> Result<(), Box<dyn std::error::Error>> {

    for path_result in fs::read_dir(dir)? {
        let path = path_result?;
        println!("Dir name: {}", path.path().display());
        let file = fs::File::open(&path.path())?;
        let reader = BufReader::new(file);

        for line_result in reader.lines() {
            let line = line_result?;
            println!("RAW: {}", line);
            match line.trim().parse::<i32>() {
                Ok(num) =>
                    println!("Int: {}", num),
                Err(err) =>
                    println!("Err: \"{}\" {}", line.trim(), err),
            }
        }
    }

    Ok(())
}
