use std::{
    error::Error,
    io,
    thread,
    sync::mpsc,
    time::Duration};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, 
    ExecutableCommand
};
use rusty_audio::Audio;
use ss3::{
    frame,
    render
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "assets/sounds/explode.wav");
    audio.add("lose", "assets/sounds/lose.wav");
    audio.add("move", "assets/sounds/move.wav");
    audio.add("pew", "assets/sounds/pew.wav");
    audio.add("startup", "assets/sounds/startup.wav");
    audio.add("win", "assets/sounds/win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout: io::Stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread

    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Gameloop
    'gameloop: loop {
        let curr_frame = frame::new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    // KeyCode::Char('p') => {
                    //     audio.play("pew");
                    // }
                    // KeyCode::Char('m') => {
                    //     audio.play("move");
                    // }
                    // KeyCode::Char('e') => {
                    //     audio.play("explode");
                    // }
                    // KeyCode::Char('w') => {
                    //     audio.play("win");
                    // }
                    _ => {}
                }
            }            
        }

        // Draw & render
        let _ = render_tx.send(curr_frame)?;
        thread::sleep(Duration::from_millis(1));
    }

    // Clean up
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
