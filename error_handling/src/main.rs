use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    // panic!("◢▆▅▄▃ ╰(〒皿〒)╯▃▄▅▆◣");
    // Try typing `RUST_BACKTRACE=1 cargo run` in the terminal

    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // unwrap() vs except()
    // let f1 = File::open("world.txt").unwrap();
    // let f2 = File::open("cool.txt").expect("cool.txt should be included in this project");

    println!("{:?}", read_username_from_file());

    println!("{:?}", last_char_of_first_line("hello world"));
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;

//     Ok(())
// }

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();

    // File::open("hello.txt")?.read_to_string(&mut s)?;

    // Ok(s)

    // Or
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
