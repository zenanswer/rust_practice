

mod argparser;
mod diriterater;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dirs = argparser::parse_arg()?;
    println!("Got dirs: {:#?}", dirs);
    for dir in dirs {
        diriterater::iterate_dir(&dir)?;
    }
    Ok(())
}
