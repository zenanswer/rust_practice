
pub fn convert_line(string: &String) -> Result<i32, Box<dyn std::error::Error>>{
    match string.trim().parse::<i32>() {
        Err(err) => {
            println!("Err: \"{}\" {}", string.trim(), err);
            return Err("Bad line.")?
        },
        Ok(num) => {
            println!("Int: {}", num);
            return Ok(num)
        }
    }
}
