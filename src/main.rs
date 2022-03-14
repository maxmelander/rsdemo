use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::prelude::*;

#[allow(dead_code)]
#[derive(Debug)]
struct Header {
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    bits_per_sample: u16,
}

impl Header {
    fn load<T: Read + Seek>(reader: &mut T) -> Result<Self, std::io::Error> {
        macro_rules! load_part{
            ($s: literal, $o: literal) => {
                {
                    let mut buffer = [0; $s];
                    reader.seek(std::io::SeekFrom::Start($o))?;
                    reader.take($s).read(&mut buffer)?;
                    buffer
                }
            }
        }

        let mut header = Self {
            audio_format: u16::from_le_bytes(load_part!(2, 20)),
            num_channels: u16::from_le_bytes(load_part!(2, 22)),
            sample_rate: u32::from_le_bytes(load_part!(4, 24)),
            byte_rate: u32::from_le_bytes(load_part!(4, 28)),
            bits_per_sample: u16::from_le_bytes(load_part!(2, 34))
        };

        Ok(dbg!(header))
    }
}

fn main() -> std::io::Result<()> {
    let file = OpenOptions::new().read(true).open("/Users/maxmelander/Development/rsdemo/data/test.wav")?;
    let mut buf_reader = BufReader::new(file);

    _ = Header::load(&mut buf_reader);

    Ok(())
}
