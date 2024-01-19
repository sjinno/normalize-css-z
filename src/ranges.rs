use crate::{
    normalizer::NUM_OF_SUPPORTED_Z, RANGE_LOWER_L, RANGE_LOWER_U, RANGE_MIDDLE_L, RANGE_MIDDLE_U,
    RANGE_UPPER_L, RANGE_UPPER_U,
};
use std::ops::RangeInclusive as Range;

type Lower = Range<i32>;
type Middle = Range<i32>;
type Upper = Range<i32>;

pub struct LMU(pub Lower, pub Middle, pub Upper);

impl std::default::Default for LMU {
    fn default() -> Self {
        Self(
            range(RANGE_LOWER_L, RANGE_LOWER_U),
            range(RANGE_MIDDLE_L, RANGE_MIDDLE_U),
            range(RANGE_UPPER_L, RANGE_UPPER_U),
        )
    }
}

impl LMU {
    pub fn iter(&self) -> impl Iterator<Item = &Range<i32>> {
        vec![&self.0, &self.1, &self.2].into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Range<i32>> {
        vec![&mut self.0, &mut self.1, &mut self.2].into_iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Range<i32>> {
        vec![self.0, self.1, self.2].into_iter()
    }
}

pub struct Ranges {
    pub ranges: LMU,
}

impl std::default::Default for Ranges {
    fn default() -> Self {
        Self {
            ranges: LMU::default(),
        }
    }
}

#[derive(Default)]
pub struct RangesBuilder {
    ranges: LMU,
    counter: usize,
}

impl RangesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_lower(mut self, r: Range<i32>) -> Self {
        self.ranges.0 = r;
        self
    }

    pub fn with_middle(mut self, r: Range<i32>) -> Self {
        self.ranges.1 = r;
        self
    }

    pub fn with_upper(mut self, r: Range<i32>) -> Self {
        self.ranges.2 = r;
        self
    }

    pub fn build(mut self) -> Option<Ranges> {
        let l = self.ranges.0;
        let m = self.ranges.1;
        let u = self.ranges.2;

        let mut lmu = [l.start(), l.end(), m.start(), m.end(), u.start(), u.end()];
        lmu.sort();

        self.ranges.0 = range(lmu[0].clone(), lmu[1].clone());
        self.ranges.1 = range(lmu[2].clone(), lmu[3].clone());
        self.ranges.2 = range(lmu[4].clone(), lmu[5].clone());
        self.counter = self.ranges.iter().map(|r| r.clone().count()).sum();

        if self.counter > NUM_OF_SUPPORTED_Z {
            panic!(
                "OUT OF RANGE: The maximum number of supported z-indexes is {}; your current counter is {}.",
                NUM_OF_SUPPORTED_Z,
                self.counter,
            )
        }

        Some(self.into())
    }
}

impl From<RangesBuilder> for Ranges {
    fn from(builder: RangesBuilder) -> Self {
        Self {
            ranges: builder.ranges,
        }
    }
}

pub fn range(start: i32, end: i32) -> Range<i32> {
    assert!(start < end);
    Range::new(start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
