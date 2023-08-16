use std::io::BufReader;

fn main() {
    //
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("assets/sounds/Juicy.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();

    // // Open the MP3 file
    // let file = File::open("path/to/yourfile.mp3").unwrap();
    // let reader = BufReader::new(file);

    // // Decode the MP3 file
    // let decoder = Decoder::new(reader).unwrap();

    // // You can now analyze the audio data, e.g., perform a Fourier Transform for spectrum analysis
    // // ...

    // // Play the audio
    // let device = rodio::default_output_device().unwrap();
    // rodio::play_raw(&device, decoder.convert_samples());
}