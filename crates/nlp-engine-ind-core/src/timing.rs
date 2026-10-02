/// Cross-platform timing for conversion latency measurement.
#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::time::Instant;

    pub struct TimerStart(Instant);

    impl TimerStart {
        pub fn now() -> Self {
            Self(Instant::now())
        }

        pub fn elapsed_us(&self) -> u64 {
            self.0.elapsed().as_micros() as u64
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod native {
    pub struct TimerStart;

    impl TimerStart {
        pub fn now() -> Self {
            Self
        }

        pub fn elapsed_us(&self) -> u64 {
            0
        }
    }
}

pub use native::TimerStart;
