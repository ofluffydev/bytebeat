# Bytebeat Player

A simple Rust implementation of a bytebeat player, created out of curiosity and boredom to explore the fascinating world of algorithmic music generation.

## What are Bytebeats?

Bytebeats are a form of algorithmic music generation where simple mathematical formulas operate on a time counter to produce audio waveforms. The concept was popularized by Viznut in 2011, demonstrating how complex and interesting musical patterns can emerge from surprisingly simple mathematical expressions.

The basic idea is:
1. Start with a time counter `t` that increments continuously
2. Apply mathematical operations (bitwise operations, arithmetic, etc.) to `t`
3. The result becomes the amplitude of the audio sample
4. As `t` increases, the formula generates a sequence of audio samples that form music

Example bytebeat formula:
```rust
t * (t >> 8 & t >> 9 & 46 & t >> 8)
```

This simple expression can create complex rhythmic and melodic patterns!

## How This Implementation Works

This project uses the [cpal](https://crates.io/crates/cpal) (Cross-Platform Audio Library) crate to play bytebeat formulas in real-time.

### Audio Pipeline

1. **Device Setup**: cpal connects to the system's default audio output device
2. **Stream Creation**: An audio stream is created with the device's default configuration
3. **Real-time Generation**: For each audio buffer requested by the system:
   - The current bytebeat formula is applied to the time counter `t`
   - The result is normalized to the range [-1.0, 1.0] for audio output
   - The time counter is incremented for the next sample

## Building and Running

Make sure you have Rust installed, then:

```bash
# Play all formulas, cycling every 4 seconds
cargo run

# Play a specific formula forever (1, 2, 3, or 4)
cargo run 1
cargo run 2
cargo run 3
cargo run 4
```

The program will start playing bytebeat formulas immediately. Press Ctrl+C to stop.

## Dependencies

- `cpal`: Cross-platform audio library for real-time audio I/O
- `rand`: Random number generation for formula switching logic

## Technical Notes

- Sample rate is determined by your audio device's default configuration
- Each formula has its own playback rate multiplier for different tempos
- The time counter `t` wraps around on overflow to prevent arithmetic issues
- Audio samples are normalized from the typical 0-255 bytebeat range to [-1.0, 1.0] for audio output
