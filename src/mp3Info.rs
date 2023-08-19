
use std::fs::read;
use puremp3::read_mp3;

fn main() {
    let data: Vec<u8> = read("assets/sounds/Juicy.mp3").expect("Could not open file");
    let (header, samples) = read_mp3(&data[..]).expect("Invalid MP3");
    println!("MP3 Header: {:#?}", header);
    for (left, right) in samples {
        // Operate on samples here
        println!("Left: {}, Right: {}", left, right);
    }
}
