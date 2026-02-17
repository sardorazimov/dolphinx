use std::sync::atomic::{AtomicU64, Ordering};


pub struct Metrics {

    pub peak_speed: AtomicU64,
    pub total_speed: AtomicU64,
    pub seconds: AtomicU64,

}


impl Metrics {

    pub fn new() -> Self {

        Self {

            peak_speed: AtomicU64::new(0),
            total_speed: AtomicU64::new(0),
            seconds: AtomicU64::new(0),

        }

    }


    pub fn update(&self, current_speed: u64) {

        // update peak
        let peak = self.peak_speed.load(Ordering::Relaxed);

        if current_speed > peak {

            self.peak_speed
                .store(current_speed, Ordering::Relaxed);

        }

        // update average tracking
        self.total_speed
            .fetch_add(current_speed, Ordering::Relaxed);

        self.seconds
            .fetch_add(1, Ordering::Relaxed);

    }


    pub fn avg_speed(&self) -> u64 {

        let total =
            self.total_speed.load(Ordering::Relaxed);

        let seconds =
            self.seconds.load(Ordering::Relaxed);

        if seconds == 0 {
            0
        } else {
            total / seconds
        }

    }


    pub fn peak(&self) -> u64 {

        self.peak_speed.load(Ordering::Relaxed)

    }

}
