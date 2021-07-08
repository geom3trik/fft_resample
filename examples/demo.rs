
use std::{error::Error, fs::File, io::BufReader, usize};

use fft_resample::fft_upsample;
use hound::{WavReader, WavSpec, SampleFormat, WavWriter};

fn main() -> Result<(), hound::Error> {
    // Replace with path to test file
    let path = "C:/Users/Setup/Music/file_example_WAV_5MG.wav";
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();
    let mut data = Vec::with_capacity((spec.channels as usize) * (reader.duration() as usize));
    match (spec.bits_per_sample, spec.sample_format) {
        (16, SampleFormat::Int) => {
            for sample in reader.samples::<i16>() {
                data.push((sample? as f32) / (0x7fffi32 as f32));
            }
        }
        (24, SampleFormat::Int) => {
            for sample in reader.samples::<i32>() {
                let val = (sample? as f32) / (0x00ff_ffffi32 as f32);
                data.push(val);
            }
        }
        (32, SampleFormat::Int) => {
            for sample in reader.samples::<i32>() {
                data.push((sample? as f32) / (0x7fff_ffffi32 as f32));
            }
        }
        (32, SampleFormat::Float) => {
            for sample in reader.samples::<f32>() {
                data.push(sample?);
            }
        }
        _ => return Err(hound::Error::Unsupported),
    }

    let resampled_buffer = fft_upsample(&data, (data.len()/44100)*48000, spec.channels as usize);

    let mut writer = WavWriter::create("test2.wav", spec)?;

    for t in 0..resampled_buffer.len() {
        let sample = resampled_buffer[t];
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16)?;
    }
    writer.finalize()?;

    Ok(())
}