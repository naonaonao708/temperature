use std::io;

fn main() {
    loop {
        println!("Which temperature unit Celsius or Fahrenheit? (C/F)");
        let mut select: String = String::new();
        io::stdin()
            .read_line(&mut select)
            .expect("Failed to read line");

        match select.as_str() {
            "C" => println!("C to F"),
            "F" => println!("F to C"),
            &_ => println!("&str"),
        }
    }
}