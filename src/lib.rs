pub mod math;
pub mod sound;

use sound::Sound;
use rustfft::{FftPlanner, num_complex::Complex};

fn run_transform(data: &[u8]) -> Vec<Complex<f32>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(data.len());
    let mut buffer = Vec::new();
    for sample in data {
        buffer.push(Complex::new(*sample as f32, 0.0));
    }
    fft.process(&mut buffer);
    buffer
}

fn harmonic_product_spectrum(data: &[Complex<f32>], sample_rate: u32) -> f32 {
    const R: u8 = 2;
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

fn detect_pitch(sound: Sound) -> f32 {
    let data = run_transform(&sound.data);
    let mut norms = Vec::new();
    for slice in &data {
        norms.push(slice.norm());
    }
    harmonic_product_spectrum(&data, sound.sample_rate)
}

mod tests {
    #![allow(unused)]
    use super::*;

    macro_rules! approx {
        ($x:expr, $y:expr) => {
            if !($x - $y < 0.0005 || $y - $x < 0.0005) {
                panic!();
            }
        };
    }

    #[test]
    fn test_hps() {
        let sound1 = Sound::from_wav("./sounds/sine_waves/220.wav");
        let freq = detect_pitch(sound1);
        approx!(220.0, freq);

        let sound2 = Sound::from_wav("./sounds/sine_waves/440.wav");
        let freq = detect_pitch(sound2);
        approx!(440.0, freq);

        let sound3 = Sound::from_wav("./sounds/sine_waves/660.wav");
        let freq = detect_pitch(sound3);
        approx!(660.0, freq);
    }
}