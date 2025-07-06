use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rand::{rng, Rng};

type BytebeatFormula = (fn(u32) -> f32, u32);

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");
    let config = device.default_output_config().unwrap();

    let mut t: u32 = 0;
    let mut current_beat_index = 0;
    let mut beat_start_time = std::time::Instant::now();
    let mut reverse_audio = false;

    let bytebeat_formulas: Vec<BytebeatFormula> = vec![
        (|t| {
            let v = (10 * (t >> 7 | t | t >> 6) + 4 * (t & t >> 13 | t >> 6)) & 255;
            (v as f32 / 128.0) - 1.0
        }, 7000),
        (|t| {
            let v = (t * (t >> 8 & t >> 9 & 46 & t >> 8)) & 255;
            (v as f32 / 128.0) - 1.0
        }, 7000),
        (|t| {
            let cond1 = if t & 4096 != 0 { 6 } else { 16 };
            let cond2 = if t & 4096 != 0 { 3 } else { 4 };
            let v = ((t * (cond1 + (1 & (t >> 14)))) >> (3 & (t >> 8))) | (t >> cond2);
            (v & 255) as f32 / 128.0 - 1.0
        }, 3000),
        (|t| {
            let first_cond = if t & 4096 != 0 {
                if t % 65536 < 59392 { 7 } else { t & 7 }
            } else {
                16
            };
            let shift_amount = 3 & ((!t) >> (if t & 2048 != 0 { 2 } else { 10 }));
            let final_shift = if t & 16384 != 0 {
                if t & 4096 != 0 { 10 } else { 3 }
            } else {
                2
            };
            let v = ((t * (first_cond + (1 & (t >> 14)))) >> shift_amount) | (t >> final_shift);
            (v & 255) as f32 / 128.0 - 1.0
        }, 4000),
    ];

    let sample_rate = config.sample_rate().0 as u32;

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                if beat_start_time.elapsed().as_secs() >= 4 {
                    // 1 in 10 chance to reverse audio
                    let mut rng = rng();
                    if rng.random_range(1..=5) == 1 {
                        reverse_audio = !reverse_audio;
                        println!("Audio reversed! Now playing {}", if reverse_audio { "backwards" } else { "forwards" });
                    }
                    
                    // Move to next formula
                    current_beat_index = (current_beat_index + 1) % bytebeat_formulas.len();
                    
                    beat_start_time = std::time::Instant::now();
                    t = 0;
                    println!("Switching to bytebeat formula {}", current_beat_index + 1);
                }

                let (current_bytebeat, bytebeat_rate) = &bytebeat_formulas[current_beat_index];

                for sample in data.iter_mut() {
                    let scaled_t = (t * bytebeat_rate) / sample_rate;
                    let audio_sample = current_bytebeat(scaled_t);
                    *sample = if reverse_audio { -audio_sample } else { audio_sample };
                    t = t.wrapping_add(1);
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )
        .unwrap();

    stream.play().unwrap();

    println!("Playing bytebeat formulas, switching every 4 seconds...");
    println!("Press Ctrl+C to stop");

    // Log first one
    println!("Starting with bytebeat formula 1");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
