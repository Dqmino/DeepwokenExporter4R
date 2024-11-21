extern crate curl;
use crate::character_info::CharacterInfo;
use std::fs::File;
use std::io;
use std::io::Read;

use crate::data_handler::send_http_request;

mod character_info;
mod data_handler;

fn main() {
    println!("Please insert your deepwoken export file path");
    let mut text = String::new();

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Invalid file path.");

    File::open(file_name)
        .unwrap()
        .read_to_string(&mut text)
        .unwrap();

    let character_info = CharacterInfo::from_str(&text);
    //println!("{:?}", character_info);
    send_http_request(character_info);
}

