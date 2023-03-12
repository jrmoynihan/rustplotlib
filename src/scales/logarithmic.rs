/// A logaritmic scale implementation
use crate::scales::{Scale, ScaleType};
use std::cmp::{max, Ordering};

/// The scale to represent logarithmic data.
#[derive(Debug, PartialEq, Clone)]
pub struct ScaleLogarithmic {
    /// The domain limits of the dataset that the scale is going to represent.
    domain: Vec<f32>,
    /// The range limits of the drawable area on the chart.
    range: Vec<isize>,
    /// The amount of ticks to display.
    tick_count: usize,
}

impl Default for ScaleLogarithmic {
    fn default() -> Self {
        Self {
            domain: vec![1., 1_000.],
            range: vec![0, 1],
            tick_count: 10,
        }
    }
}

impl ScaleLogarithmic {
    /// Create a new logarithmic scale with default values.
    pub fn new() -> Self {
        ScaleLogarithmic::default()
    }

    /// Set the domain limits for the scale band.
    /// The domain must be positive.
    /// The domain must not contain NaN.
    /// The domain must not contain infinite values.
    /// The domain must not contain duplicate values.
    /// The domain must not contain values that are not finite.
    /// The domain must not contain values that are not normal.
    /// The domain must not contain values that are not subnormal.
    pub fn set_domain(mut self, range: Vec<f32>) -> Self {
        self.domain = range;
        self
    }

    /// Get the domain limits of the scale.
    pub fn domain(&self) -> &Vec<f32> {
        &self.domain
    }

    /// Set the range limits for the scale band.
    pub fn set_range(mut self, range: Vec<isize>) -> Self {
        self.range = range;
        self
    }

    /// Get the range limits of the scale.
    pub fn range(&self) -> &Vec<isize> {
        &self.range
    }

    /// Takes a value x in [a, b] and returns the corresponding value in [0, 1].
    fn normalize(&self, domain_min: f32, domain_max: f32, x: f32) -> f32 {
        // If a == b then return 0.5
        if (domain_min - domain_max).abs() < f32::EPSILON {
            0.5
        } else {
            let domain_distance = domain_max - domain_min;
            (x - domain_min) / domain_distance
        }
    }

    /// Takes a value x in [0, 1] and returns the corresponding value in [a, b].
    fn denormalize(&self, a: f32, b: f32, x: f32) -> f32 {
        // If a == b then return 0.5
        if (a - b).abs() < f32::EPSILON {
            0.5
        } else {
            let b = b - a;
            x * b + a
        }
    }

    /// Takes a value t in [0, 1] and returns the corresponding range in [a, b].
    fn interpolate(&self, a: f32, b: f32, t: f32) -> f32 {
        (b - a) * t + a
    }

    /// Compute the distance between the ticks.
    fn compute_tick_distance(&self) -> f32 {
        let domain = self.domain();

        let domain_min = domain[0];
        let domain_max = domain[1];

        let domain_distance = domain_max - domain_min;

        let tick_count = self.tick_count;
        let tick_distance = domain_distance / (tick_count as f32);

        let tick_distance: f32 = tick_distance.log10().floor();
        let mut tick_distance = 10_f32.powf(tick_distance);

        let tick_count: f32 = domain_distance / tick_distance;
        if tick_count < 2. {
            tick_distance /= 2.;
        } else if tick_count > 5. {
            tick_distance *= 2.;
        }

        tick_distance
    }
}

impl Scale<f32> for ScaleLogarithmic {
    /// Get the type of the scale
    fn get_type(&self) -> ScaleType {
        ScaleType::Logarithmic(self.clone())
    }
    /// Get the domain of the scale
    fn get_domain(&self) -> Vec<f32> {
        self.domain().clone()
    }

    /// Get the domain max of the scale.
    fn domain_max(&self) -> f32 {
        self.domain[1]
    }

    // Get the range value for the given domain entry
    fn scale(&self, x: &f32) -> f32 {
        let domain = self.domain();
        let range = self.range();

        let domain_min = domain[0];
        let domain_max = domain[1];
        let range_min = range[0];
        let _range_max = range[1];

        let normalized = self.normalize(domain_min, domain_max, *x);
        self.interpolate(range_min as f32, _range_max as f32, normalized)
    }

    /// Get the bandwidth (if present)
    fn bandwidth(&self) -> Option<f32> {
        Some(0_f32)
    }

    /// Get the start range value.
    fn range_start(&self) -> f32 {
        self.range[0] as f32
    }

    /// Get the end range value.
    fn range_end(&self) -> f32 {
        self.range[1] as f32
    }

    /// Get the ticks for the scale.
    fn get_ticks(&self) -> Vec<f32> {
        let domain = self.domain();
        let range = self.range();

        let domain_min = domain[0];
        let domain_max = domain[1];
        let range_min = range[0];
        let range_max = range[1];

        let domain_distance = domain_max - domain_min;
        let range_distance = range_max - range_min;

        let tick_distance = self.compute_tick_distance();

        let mut ticks = vec![];
        let mut tick = domain_min;
        while tick <= domain_max {
            ticks.push(tick);
            tick += tick_distance;
        }

        ticks
    }
}
