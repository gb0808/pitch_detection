pub mod sound;
pub mod dsp;

mod tests {
    #![allow(unused)]
    use super::*;
    use dsp::*;
    use sound::Sound;
    use std::{fs::File, io::prelude::*};

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

    #[test]
    fn plot_frequencies() -> std::io::Result<()> {
        let sound = Sound::from_wav("./sounds/passage/cello_passage.wav");
        let frequencies = dsp::detect_pitch(&sound.data, sound.sample_rate);
        
        let mut file = File::create("test_out/frequencies.csv")?;
        for freq in frequencies {
            let text = format!("{freq},\n");
            file.write(text.as_bytes());
        }
        Ok(())
    }
}