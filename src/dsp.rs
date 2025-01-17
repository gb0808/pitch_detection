use rustfft::{FftPlanner, num_complex::Complex};

fn divide_signal(signal: &[u8], frame_size: usize) -> Vec<Vec<u8>> {
    let mut data = Vec::new();
    let mut i = 0;
    while i < signal.len() {
        let remaining_slots = signal.len() - i;
        if remaining_slots >= frame_size {
            data.push(signal[i..i+frame_size].to_vec());
        } else {
            data.push([&signal[i..], &vec![0u8; frame_size - remaining_slots]].concat().to_vec());
        }
        i += frame_size;
    }
    data
}

fn run_transform(data: &[u8]) -> Vec<Complex<f32>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(data.len());
    let mut buffer: Vec<Complex<f32>> = data.into_iter()
                                            .map(|sample| Complex::new(*sample as f32, 0.0))
                                            .collect();
    fft.process(&mut buffer);
    buffer
}

fn harmonic_product_spectrum(data: &[Complex<f32>], sample_rate: u32) -> f32 {
    const R: u8 = 1;
    let low = (20.0 * data.len() as f32 / sample_rate as f32) as usize;
    let high = (20000.0 * data.len() as f32 / sample_rate as f32) as usize;
    let (mut max_frequency, mut max_product) = (0.0, 0.0);
    for i in low..high {
        let mut product = 1.0;
        let freq = i as f32 / data.len() as f32 * sample_rate as f32;
        for r in 1..=R {
            let index = (freq * r as f32 * data.len() as f32 / sample_rate as f32) as usize;
            product *= data[index].norm();
        }
        if product > max_product {
            max_product = product;
            max_frequency = freq;
        }
    }
    max_frequency * 2.0
}

pub fn detect_pitch(signal: &[u8], sample_rate: u32) -> Vec<f32> {
    let frame_size = (sample_rate / 10) as usize;
    let raw_frames = divide_signal(signal, frame_size);
    println!("Signal Length: {}", raw_frames[1].len());
    let frames: Vec<Vec<Complex<f32>>> = raw_frames.into_iter()
                                                   .map(|rf| run_transform(&rf))
                                                   .collect();
    frames.into_iter().map(|frame| harmonic_product_spectrum(&frame, sample_rate)).collect()
}