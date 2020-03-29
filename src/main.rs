use enigo::*;
use std::error::Error;
use std::io::stdin;
use std::result::Result;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Please input the keys that you want to be repeated");
    let mut raw_key_string = String::new();
    stdin().read_line(&mut raw_key_string)?;

    println!("Please input the number of seconds to wait before repeating key presses");
    let mut raw_time_string = String::new();
    stdin().read_line(&mut raw_time_string)?;

    let key_string = raw_key_string.trim();
    let time = raw_time_string.trim().parse()?;

    let mut enigo = Enigo::new();
    let duration_to_sleep = Duration::from_secs(time);

    println!(
        "Spacemachine will type {} every {} seconds\n\r",
        key_string, time
    );

    println!("Starting in 3 seconds");
    thread::sleep(Duration::from_secs(3));

    loop {
        thread::sleep(duration_to_sleep);
        enigo.key_sequence_parse(&key_string);
    }
}
