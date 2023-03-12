use crate::{ScaleBand, ScaleLinear, ScaleLogarithmic};

pub mod band;
pub mod linear;
pub mod logarithmic;

#[derive(PartialEq)]
pub enum ScaleType {
    Band(ScaleBand),
    Linear(ScaleLinear),
    Logarithmic(ScaleLogarithmic),
    // Ordinal(ScaleO),
}

pub enum ScaleDomainValue {
    Band(String),
    Linear(f32),
    Logarithmic(f32),
}

impl ScaleType {
    pub fn scale(&self, domain: &ScaleDomainValue) -> Result<f32, ()> {
        match self {
            ScaleType::Band(s) => match domain {
                ScaleDomainValue::Band(d) => Ok(s.scale(d)),
                _ => Err(()),
            },
            ScaleType::Linear(s) => match domain {
                ScaleDomainValue::Linear(d) => Ok(s.scale(d)),
                _ => Err(()),
            },
            ScaleType::Logarithmic(s) => match domain {
                ScaleDomainValue::Logarithmic(d) => Ok(s.scale(d)),
                _ => Err(()),
            },
        }
    }
    pub fn is_range_reversed(&self) -> bool {
        match self {
            ScaleType::Band(s) => s.is_range_reversed(),
            ScaleType::Linear(s) => s.is_range_reversed(),
            ScaleType::Logarithmic(s) => s.is_range_reversed(),
        }
    }

    pub fn bandwidth(&self) -> Option<f32> {
        match self {
            ScaleType::Band(s) => s.bandwidth(),
            _ => None,
        }
    }
}

/// The Scale trait defines common operations on all scales.
pub trait Scale<T> {
    /// Get the type of the scale.
    fn get_type(&self) -> String;

    /// Get the domain of the scale.
    fn get_domain(&self) -> Vec<T>;

    fn domain_max(&self) -> f32;

    /// Get the range value for the given domain entry.
    fn scale(&self, domain: &T) -> f32;

    /// Get the bandwidth (if present).
    fn bandwidth(&self) -> Option<f32>;

    /// Get the start range value.
    fn range_start(&self) -> f32;

    /// Get the end range value.
    fn range_end(&self) -> f32;

    /// Check whether the range is in reversed order, meaning the start is greater than the end.
    fn is_range_reversed(&self) -> bool {
        self.range_start() > self.range_end()
    }

    /// Get the list of ticks that represent the scale on a chart axis.
    fn get_ticks(&self) -> Vec<T>;
}
