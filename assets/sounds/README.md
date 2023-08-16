# What is MP3?

MP3 (MPEG Audio Layer III) is a common audio format that uses lossy compression to encode music and other audio content.

## MP3 File Structure

An MP3 file consists of a series of frames, each containing header information, audio data, and sometimes additional metadata. The general structure is as follows:

```
+----------------+---------------------+------------------------+
| Frame Header   | Audio Data          | Optional Metadata      |
+----------------+---------------------+------------------------+
| 4 Bytes        | Variable Length     | Variable Length        |
+----------------+---------------------+------------------------+
```


### Frame Header (4 Bytes)

The frame header is a 4-byte structure that contains vital information about the frame:

- **Byte 0:** Sync word (high 7 bits), MPEG version and layer information (low 1 bit).
- **Byte 1:** Protection, bitrate index, sampling rate frequency, padding, and private bit.
- **Byte 2:** Channel mode, mode extension, copyright, originality, and home.
- **Byte 3:** Emphasis.

### Example:

Here's a simple representation of an MP3 frame header:

```
Byte 0: 11111101
Byte 1: 10110110
Byte 2: 11010001
Byte 3: 00000011
```

### Read an MP3 File and play it with Rodio

```rust
use std::io::BufReader;

fn main() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("assets/sounds/Juicy.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}
```

### Wiki credits:
- [Mp3filestructure.svg](https://en.wikipedia.org/wiki/MP3#/media/File:Mp3filestructure.svg)
- [Wiki](https://en.wikipedia.org/wiki/MP3)


This example shows how to load and decode an MP3 file with Rodio in Rust. From here, you can apply various analysis techniques like Fourier Transform to study the audio spectrum.


