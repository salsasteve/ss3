use rodio::{Decoder, OutputStream, Sample, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::thread;

struct MyIterator<S>
where
    S: Source,
    S::Item: Sample + std::fmt::Debug + Clone,
{
    source: S,
    sender: mpsc::Sender<S::Item>,
}

impl<S> Iterator for MyIterator<S>
where
    S: Source,
    S::Item: Sample + std::fmt::Debug + Clone,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.source.next();
        if let Some(ref s) = sample {
            // Here you can send the sample to your visualization logic
            self.sender.send(s.clone()).unwrap();
        }
        sample
    }
}

impl<T> Source for MyIterator<T>
where
    T: Source,
    T::Item: Sample + std::fmt::Debug + Clone,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.source.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.source.total_duration()
    }
}

fn main() {
    // This is grabbing the default output device on your system
    let (_stream, handle) = OutputStream::try_default().unwrap();

    // This is creating a new sink to play audio on
    // A sink represents a "streaming end point" for audio data
    let sink = Sink::try_new(&handle).unwrap();

    // This is loading a sound from a file
    let file = File::open("assets/sounds/Juicy.mp3").expect("Unable to open file"); //.unwrap();

    // Creating a buffered reader from the file
    let br = BufReader::new(file);

    // Creating a decoder from the buffered reader
    let dc = Decoder::new(br).unwrap();

    // Create a channel for sending samples to another thread
    let (sender, receiver) = mpsc::channel();

    // Spawn a thread to print the samples
    thread::spawn(move || {
        for sample in receiver {
            println!("Sample: {:#?}", sample);
        }
    });

    // Appending the decoded audio to the sink
    // Wrap the decoder in your custom iterator
    let my_iterator = MyIterator {
        source: dc,
        sender,
    };

    sink.append(my_iterator);

    // Sleeping until the sound finishes playing
    sink.sleep_until_end();
}
