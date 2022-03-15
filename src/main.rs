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
    fn load_part<const SIZE: usize, T: Read + Seek>(reader: &mut T, offset: u64) -> Result<[u8; SIZE], std::io::Error> {
        let mut buffer = [0; SIZE];
        reader.seek(std::io::SeekFrom::Start(offset))?;
        reader.take(SIZE as u64).read(&mut buffer)?;
        Ok(buffer)
    }
    fn load<T: Read + Seek>(reader: &mut T) -> Result<Self, std::io::Error> {
        let header = Self {
            audio_format: u16::from_le_bytes(Self::load_part(reader, 20)?),
            num_channels: u16::from_le_bytes(Self::load_part(reader, 22)?),
            sample_rate: u32::from_le_bytes(Self::load_part(reader, 24)?),
            byte_rate: u32::from_le_bytes(Self::load_part(reader, 28)?),
            bits_per_sample: u16::from_le_bytes(Self::load_part(reader, 34)?)
        };

        Ok(dbg!(header))
    }
}

fn main() -> std::io::Result<()> {
    let file = OpenOptions::new().read(true).open("/home/maxel/Development/rsdemo/data/test.wav")?;
    let mut buf_reader = BufReader::new(file);

    _ = Header::load(&mut buf_reader);

    Ok(())
}
