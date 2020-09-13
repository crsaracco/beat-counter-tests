mod beat_counter;
use beat_counter::BeatCounter;
use beat_counter::BeatCounterF64;

const MILLIBEAT_MULITPLIER: u64 = 1000;
const MICROBEAT_MULTIPLIER: u64 = 1000000;
const NANOBEAT_MULTIPLIER: u64 =  1000000000;
const PICOBEAT_MULTIPLIER: u64 =  1000000000000;

const MULTIPLIER: u64 = PICOBEAT_MULTIPLIER;
const BPM: u64 = 1;
const SAMPLING_FREQUENCY: u64 = 48000;

fn single_test_f64(beat_counter_f64: &mut BeatCounterF64, sample: u64, sampling_frequency_f64: f64, bpm_f64: f64) -> f64 {
    let expected_sample = sample as f64;
    let current_beat = beat_counter_f64.process();

    let bps_f64 = bpm_f64 / 60.0;
    let measured_sample = current_beat / bps_f64 * sampling_frequency_f64;

    measured_sample - expected_sample
}

fn single_test_bc(beat_counter: &mut BeatCounter, multiplier: f64, sample: u64, sampling_frequency_f64: f64, bpm_f64: f64) -> f64 {
    let expected_sample = sample as f64;
    let current_beat = beat_counter.process();

    let bps_f64 = bpm_f64 / 60.0;
    let measured_sample = (current_beat as f64) / multiplier / bps_f64 * sampling_frequency_f64;

    measured_sample - expected_sample
}

fn main() {
    let bpm_f64 = BPM as f64;
    let sampling_frequency_f64 = SAMPLING_FREQUENCY as f64;

    let mut beat_counter_f64 = BeatCounterF64::new(1.0, sampling_frequency_f64);
    let mut beat_counter_bc = BeatCounter::new(BPM * MULTIPLIER, SAMPLING_FREQUENCY);

    let mut s = 0;
    loop {
        let error_f64 = single_test_f64(&mut beat_counter_f64, s, sampling_frequency_f64, bpm_f64);
        let error_bc = single_test_bc(&mut beat_counter_bc, MULTIPLIER as f64, s, sampling_frequency_f64, bpm_f64);

        if s % 10000 == 0 {
            println!("sample: {:>15}, error_f64: {:0.8}, error_u64: {:0.8}", s, error_f64, error_bc);
        }
        if error_f64.abs() >= 0.5 || error_bc.abs() >= 0.5 {
            println!("sample: {:>15}, error_f64: {:0.8}, error_u64: {:0.8}", s, error_f64, error_bc);
            break;
        }
        s += 1;
    }
}