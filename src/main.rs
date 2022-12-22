use std::io;

fn main() {
    println!("Which temperature unit Celsius or Fahrenheit? (C/F)");
    let mut select = String::new();
    io::stdin()
        .read_line(&mut select)
        .expect("Failed to read line");

    let temp: char = match select.trim().parse() {
        Ok(char) => char,
        Err(_) => continue,
    };
    
}