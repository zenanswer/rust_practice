

mod argparser;
mod diriterater;
mod lineconverter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dirs = argparser::parse_arg()?;
    println!("Got dirs: {:#?}", dirs);
    for dir in dirs {
        diriterater::iterate_dir(&dir, lineconverter::convert_line)?;
    }
    Ok(())
}
