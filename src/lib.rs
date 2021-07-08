use core::num;

use rustfft::{FftPlanner, num_complex::Complex};

pub fn fft_upsample(input: &[f32], output_length: usize, num_channels: usize) -> Vec<f32> {
    let input_length = input.len();

    assert!(output_length >= input_length);

    // Setup FFT stuff
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(input_length / num_channels);
    let ifft = planner.plan_fft_inverse(output_length / num_channels);

    // Allocate buffers for deinterleaving and complex output
    let mut deinterleaved = vec![0.0; input_length];
    let mut output = Vec::with_capacity(output_length);


    deinterleave(input, &mut deinterleaved, num_channels);

    deinterleaved.chunks(input_length / num_channels).for_each(|chunk| {
        // Convert chunk to complex
        let mut buffer = chunk.iter().map(|sample| Complex {re: *sample, im: 0.0}).collect::<Vec<_>>();
        
        // Forward FFT
        fft.process(&mut buffer);

        let mut padded_buffer = vec![Complex{re: 0.0f32, im: 0.0f32}; output_length/num_channels];

        // Pad with zeros
        zero_pad(&buffer, &mut padded_buffer, (output_length - input_length)/num_channels);
        
        // Inverse FFT
        ifft.process(&mut padded_buffer);

        // Push into output
        output.extend(padded_buffer.drain(0..));

    });    

    // Convert output to real
    let deinterleaved_output = output.iter().map(|complex_sample| complex_sample.re/output.len() as f32).collect::<Vec<_>>();

    // Allocate vec for interleaving
    let mut output = vec![0.0; output_length];

    interleave(&deinterleaved_output, &mut output, num_channels);

    output
}

pub fn deinterleave<T: Copy>(input: &[T], output: &mut [T], num_channels: usize) {
    debug_assert_eq!(input.len(), output.len());
    let num_samples = input.len() / num_channels;
    for sm in 0..num_samples {
        for ch in 0..num_channels {
            output[ch * num_samples + sm] = input[sm * num_channels + ch];
        }
    }
}

/// Interleave a buffer of samples into an output buffer.
pub fn interleave<T: Copy>(input: &[T], output: &mut [T], num_channels: usize) {
    debug_assert_eq!(input.len(), output.len());
    let num_samples = input.len() / num_channels;
    for sm in 0..num_samples {
        for ch in 0..num_channels {
            output[sm * num_channels + ch] = input[ch * num_samples + sm];
        }
    }
}

pub fn zero_pad(input: &[Complex<f32>], output: &mut [Complex<f32>], num_zeros: usize) {
    assert!(input.len() + num_zeros == output.len());
    for (index, sample) in input[0..input.len()/2].iter().enumerate() {
        output[index] = *sample;
    }

    for (index, sample) in input[(input.len()/2)..(input.len())].iter().enumerate() {
        output[input.len()/2 + num_zeros + index] = *sample;
    }

}