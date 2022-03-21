use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Debug)]
struct Wav {
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    bits_per_sample: u16,
    data_size: u32,
    data: Vec<f32>,
}

impl Wav {
    fn load_part<const SIZE: usize, T: Read + Seek>(
        reader: &mut T,
        offset: u64,
    ) -> Result<[u8; SIZE], std::io::Error> {
        let mut buffer = [0; SIZE];
        reader.seek(std::io::SeekFrom::Start(offset))?;
        reader.take(SIZE as u64).read(&mut buffer)?;
        Ok(buffer)
    }

    fn load_data<T: Read + Seek>(
        reader: &mut T,
        data_size: usize,
    ) -> Result<Vec<f32>, std::io::Error> {
        reader.seek(std::io::SeekFrom::Start(44))?;
        let mut result = Vec::<f32>::with_capacity(data_size / 2);
        let mut buf = [0; 2];
        while reader.read(&mut buf)? > 0 {
            result.push(i16::from_le_bytes(buf) as f32 / i16::MAX as f32)
        }
        Ok(result)
    }

    fn load<T: Read + Seek>(reader: &mut T) -> Result<Self, std::io::Error> {
        let data_size = u32::from_le_bytes(Self::load_part(reader, 40)?);
        let wav = Self {
            audio_format: u16::from_le_bytes(Self::load_part(reader, 20)?),
            num_channels: u16::from_le_bytes(Self::load_part(reader, 22)?),
            sample_rate: u32::from_le_bytes(Self::load_part(reader, 24)?),
            byte_rate: u32::from_le_bytes(Self::load_part(reader, 28)?),
            bits_per_sample: u16::from_le_bytes(Self::load_part(reader, 34)?),
            data_size: data_size,
            data: Self::load_data(reader, data_size as usize)?,
        };

        Ok(wav)
    }
}

fn main() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open("/Users/maxmelander/Development/rsdemo/data/test2.wav")?;
    let mut buf_reader = BufReader::new(file);

    let wav = Wav::load(&mut buf_reader)?;

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let c: cpal::StreamConfig = cpal::StreamConfig {
        channels: wav.num_channels,
        sample_rate: cpal::SampleRate(wav.sample_rate),
        buffer_size: cpal::BufferSize::Default,
    };

    run::<f32>(&device, &c, Arc::new(wav))?;

    Ok(())
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig, wav: Arc<Wav>) -> std::io::Result<()>
where
    T: cpal::Sample,
{
    let channels = config.channels as usize;

    let mut sample_clock: usize = 0;
    let mut next_value = move || {
        sample_clock += 1;
        wav.data[sample_clock]
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value)
            },
            err_fn,
        )
        .unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(3000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
