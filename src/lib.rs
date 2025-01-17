pub mod sound;
pub mod dsp;

mod tests {
    #![allow(unused)]
    use super::*;
    use dsp::*;
    use sound::Sound;

    #[ignore]
    #[test]
    fn test_hps_sine() {
        let test_freq = 440;
        let sound = Sound::from_wav(&format!("./sounds/sine_waves/{test_freq}.wav"));
        let frequencies = dsp::detect_pitch(&sound.data, sound.sample_rate);
        for freq in frequencies {
            println!("{freq}");
        }
    }

    #[ignore]
    #[test]
    fn test_hps_scale() {
        let sound = Sound::from_wav("./sounds/scales/c_major_scale.wav");
        let frequencies = dsp::detect_pitch(&sound.data, sound.sample_rate);
        for freq in frequencies {
            println!("{freq}");
        } 
    }
}