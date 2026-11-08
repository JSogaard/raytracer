#[derive(Debug, Copy, Clone)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn bounding(first: Self, second: Self) -> Self {
        let min = f64::min(first.min, second.min);
        let max = f64::max(first.max, second.max);

        Self { min, max }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn span(&self) -> f64 {
        (self.max - self.min).abs()
    }

    pub fn center(&self) -> f64 {
        (self.min() + self.max()) / 2.0
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }

    pub fn overlap(&self, second: &Self, third: &Self) -> bool {
        let min = self.min.max(second.min.max(third.min));
        let max = self.max.min(second.max.min(third.max));
        min <= max
    }

    pub fn expand(&self, x: f64) -> Self {
        let padding = x / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }

    pub fn invert(&self) -> Self {
        Self::new(self.max(), self.min())
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
}
