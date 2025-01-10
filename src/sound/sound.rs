use super::LittleEndian;
use std::{cell::RefCell, fmt, fs};

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
    pub data: RefCell<Vec<u8>>,
}

impl Sound {
    pub fn from_wav(path: &str) -> Self {
        let contents = fs::read(path).expect("error: could not read file");
    
        // The "RIFF" chunk descriptor
        let _riff = &contents[0..4];                // big endian
        let _chunksize = &contents[4..8];           // little endian
        let _format = &contents[8..12];             // big endian

        // The "fmt" sub-chunk
        let _subchunk1id = &contents[12..16];        // bid endian
        let _subchunk1size = &contents[16..20];     // little endian
        let _audioformat = &contents[20..22];       // little endian
        let _numchannels = &contents[22..24];       // little endian
        let _samplerate = &contents[24..28];        // little endian
        let _byterate = &contents[28..32];          // little endian
        let _blockalign = &contents[32..34];        // little endian
        let _bitspersample = &contents[34..36];     // little endian

        // The "data" sub-chunk
        let _subchunk2id = &contents[36..40];       // big endian
        let _subchunk2size = &contents[40..44];     // little endian
        let _data = &contents[44..];                // little endian

        Self {
            sample_rate: LittleEndian::read_u32(_samplerate),
            data: if LittleEndian::read_u16(_numchannels) == 2 {
                RefCell::new(stero_to_mono(_data))
            } else if LittleEndian::read_u16(_numchannels) == 1 {
                RefCell::new(_data.to_vec())
            } else {
                RefCell::new(Vec::new())
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
            self.data.borrow().len()
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