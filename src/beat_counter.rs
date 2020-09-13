pub struct BeatCounter {
    sampling_frequency: u64,
    beats_per_sample: u64,
    remainder_per_sample: u64,
    beat_counter: u64,
    remainder_counter: u64,
}

impl BeatCounter {
    pub fn new(bpm: u64, sampling_frequency: u64) -> Self {
        let bps = bpm / 60;

        let beats_per_sample = bps / sampling_frequency;
        let remainder_per_sample = bps % sampling_frequency;

        Self {
            sampling_frequency,
            beats_per_sample,
            remainder_per_sample,
            beat_counter: 0,
            remainder_counter: 0,
        }
    }

    // Advance counters by one sample. Return counter.
    pub fn process(&mut self) -> u64 {
        // Force the first output to be zero
        let current_sample = self.beat_counter;

        self.beat_counter += self.beats_per_sample;
        self.remainder_counter += self.remainder_per_sample;
        if self.remainder_counter >= self.sampling_frequency {
            self.beat_counter += 1;
            self.remainder_counter -= self.sampling_frequency;
        }

        current_sample
    }
}

pub struct BeatCounterF64 {
    beat: f64,
    bpm: f64,
    sampling_frequency: f64,
}

impl BeatCounterF64 {
    pub fn new(bpm: f64, sampling_frequency: f64) -> Self {
        Self {
            beat: 0.0,
            bpm,
            sampling_frequency
        }
    }

    pub fn process(&mut self) -> f64 {
        // Force first result to be zero
        let current_beat = self.beat;

        let beats_per_sample = self.bpm / 60.0 / self.sampling_frequency;
        self.beat += beats_per_sample;

        current_beat
    }
}