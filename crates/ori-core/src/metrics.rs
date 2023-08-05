use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

#[derive(Clone, Debug)]
pub struct InstantMetrics {
    events: VecDeque<Instant>,
    count: usize,
}

impl Default for InstantMetrics {
    fn default() -> Self {
        Self::new(100)
    }
}

impl InstantMetrics {
    pub fn new(count: usize) -> Self {
        Self {
            events: VecDeque::new(),
            count,
        }
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn event(&mut self) {
        self.events.push_back(Instant::now());

        if self.events.len() > self.count {
            self.events.pop_front();
        }
    }

    pub fn duration(&self) -> Option<Duration> {
        if self.events.len() < 2 {
            return None;
        }

        let now = Instant::now();
        let &first = self.events.front().unwrap();
        Some(now - first)
    }

    pub fn seconds_per_event(&self) -> Option<f32> {
        let seconds = self.duration()?.as_secs_f32();
        Some(self.events.len() as f32 / seconds)
    }

    pub fn events_per_second(&self) -> f32 {
        match self.duration() {
            Some(duration) => {
                let seconds = duration.as_secs_f32();
                self.events.len() as f32 / seconds
            }
            None => 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DurationMetrics {
    events: VecDeque<Duration>,
    count: usize,
}

impl Default for DurationMetrics {
    fn default() -> Self {
        Self::new(100)
    }
}

impl DurationMetrics {
    pub fn new(count: usize) -> Self {
        Self {
            events: VecDeque::new(),
            count,
        }
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn event(&mut self, duration: Duration) {
        self.events.push_back(duration);

        if self.events.len() > self.count {
            self.events.pop_front();
        }
    }

    pub fn average(&self) -> Option<Duration> {
        if self.events.is_empty() {
            return None;
        }

        let sum = self.events.iter().sum::<Duration>();
        let average = sum / self.events.len() as u32;
        Some(average)
    }
}

#[derive(Clone, Debug)]
pub struct Metrics {
    /// The time the application was started.
    pub start_time: Instant,
    /// The number of pointer moved events per second.
    pub pointer_moved: InstantMetrics,
    /// The time it took to process the event.
    pub event: DurationMetrics,
    /// The time it took to layout the application.
    pub layout: DurationMetrics,
    /// The time it took to draw the application.
    pub draw: DurationMetrics,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            pointer_moved: InstantMetrics::new(100),
            event: DurationMetrics::new(100),
            layout: DurationMetrics::new(100),
            draw: DurationMetrics::new(100),
        }
    }

    pub fn log(&self) {
        let _scope = tracing::debug_span!("metrics").entered();

        tracing::debug!(
            "Pointer moved events per second: {:.2}",
            self.pointer_moved.events_per_second()
        );
        let event = self.event.average().unwrap_or_default();
        let layout = self.layout.average().unwrap_or_default();
        let draw = self.draw.average().unwrap_or_default();
        tracing::debug!("Average event processing time: {:?}", event);
        tracing::debug!("Average layout time: {:?}", layout);
        tracing::debug!("Average draw time: {:?}", draw);
    }
}
