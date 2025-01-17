use super::LittleEndian;
use std::{fmt, fs};

fn stero_to_mono(data: &[u8]) -> Vec<u8> {
    let mut avg = Vec::new();
    let mut temp = 0;
    for i in 0..data.len() {
        if i % 2 == 0 {
            temp = data[i];
        } else {
            avg.push(temp / 2 + data[i] / 2);
        }
    }
    avg
}

pub struct Sound {
    pub sample_rate: u32,
    pub data: Vec<u8>,
}

impl Sound {
    pub fn from_wav(path: &str) -> Self {
        let contents = fs::read(path).expect("error: could not read file");
        let numchannels = &contents[22..24];
        let samplerate = &contents[24..28];
        let _data = &contents[44..];
    
        Self {
            sample_rate: LittleEndian::read_u32(samplerate),
            data: if LittleEndian::read_u16(numchannels) == 2 {
                stero_to_mono(_data)
            } else if LittleEndian::read_u16(numchannels) == 1 {
                _data.to_vec()
            } else {
                Vec::new()
            },
        }
    }
}

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "Sample Rate: {}\nData Length: {} bytes\n", 
            self.sample_rate, 
            self.data.len()
        )
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    #[test]
    fn display() {
        let sound = Sound::from_wav("./sounds/sine_waves/440.wav");
        println!("{sound}");
    }
}