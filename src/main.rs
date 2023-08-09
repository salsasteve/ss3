use std::error::Error;

use rusty_audio::Audio;
fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "assets/sounds/explode.wav");
    audio.add("lose", "assets/sounds/lose.wav");
    audio.add("move", "assets/sounds/move.wav");
    audio.add("pew", "assets/sounds/pew.wav");
    audio.add("startup", "assets/sounds/startup.wav");
    audio.add("win", "assets/sounds/win.wav");
    audio.play("startup");

    audio.wait();
    Ok(())
}
