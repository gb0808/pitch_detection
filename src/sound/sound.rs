use super::LittleEndian;
use std::{fmt, fs};

pub struct Sound {
    sample_rate: u32,
    bits_per_sample: u16,
    number_of_channels: u16,
    data: Vec<u8>,
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
            bits_per_sample: LittleEndian::read_u16(_bitspersample),
            number_of_channels: LittleEndian::read_u16(_numchannels),
            data: _data.to_vec(),
        }
    }
}

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "Sample Rate: {}\nBits per Sample: {}\nChannels: {}\nData Length: {} bytes\n",
            self.sample_rate, 
            self.bits_per_sample,
            self.number_of_channels,
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