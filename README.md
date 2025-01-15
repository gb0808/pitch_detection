# pitch_detection
This is a rust crate that identifies pitches in a piece of music

Under heavy development - Updates forthcoming

## Updates
### v 1.0.2
* rustfft is being used for the fft
* got rid of math module

### v 1.0.1
* fft is now done in-place

### v 1.0.0
* Harmonic Product Spectrum algorithm for detecting the fundamental frequency of a sound.
* The program is able to accurately detect the pitch of a sine wave.
* This program has a custom fft implementation that needs some work.