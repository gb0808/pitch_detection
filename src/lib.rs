pub mod math;
pub mod sound;

use sound::Sound;
use math::{transform, complex};

fn harmonic_product_spectrum(data: &[complex::ComplexNumber], sample_rate: u32) -> f32 {
    const R: u8 = 2;
    let low = (20.0 * data.len() as f32 / sample_rate as f32) as usize;
    let high = (20000.0 * data.len() as f32 / sample_rate as f32) as usize;
    let (mut max_frequency, mut max_product) = (0.0, 0.0);
    for i in low..high {
        let mut product = 1.0;
        let freq = i as f32 / data.len() as f32 * sample_rate as f32;
        for r in 1..=R {
            let index = (freq * r as f32 * data.len() as f32 / sample_rate as f32) as usize;
            product *= data[index].get_magnitude();
        }
        if product > max_product {
            max_product = product;
            max_frequency = freq;
        }
    }
    max_frequency * 2.0
}

fn detect_pitch(sound: Sound) -> f32 {
    let data = transform::fast_fourier_transform(&sound.data);
    let mut norms = Vec::new();
    for slice in &data {
        norms.push(slice.get_magnitude());
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