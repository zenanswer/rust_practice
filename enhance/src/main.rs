use std::thread;
use std::time::Duration;

trait AsSeconds<'a> {
    type Item;
    fn as_seconds(&'a self) -> Result<Duration, std::num::ParseIntError>;
}

impl<'a> AsSeconds<'a> for u64 {
    type Item = u64;
    fn as_seconds(& self) -> Result<Duration, std::num::ParseIntError> {
        Ok(Duration::new(self.clone(), 0))
    }
}

impl<'a> AsSeconds<'a> for &str {
    type Item = &'a str;
    fn as_seconds(&'a self) -> Result<Duration, std::num::ParseIntError>{
        let u64_seconds_result = u64::from_str_radix(self, 10)?;
        Ok(Duration::new(u64_seconds_result, 0))
    }
}

fn main() -> Result<(), std::num::ParseIntError> {
    // let five_seconds = Duration::new(5, 0);
    // thread::sleep(five_seconds);
    println!("Sleep 6 seconds ...");
    thread::sleep(6.as_seconds()?);
    println!("Sleep 5 seconds ...");
    thread::sleep("5".as_seconds()?);
    println!("Exit ...");
    Ok(())
}
