pub mod math;
pub mod sound;

use sound::Sound;
use rustfft::{FftPlanner, num_complex::Complex};

fn detect_pitch(sound: Sound) -> f32 {
    todo!()    
}

mod tests {
    #![allow(unused)]
    use super::*;

}