use crate::{
    normalizer::{Normalizer, NUM_OF_SUPPORTED_Z},
    MANTISSA, RANGE_LOWER_L, RANGE_LOWER_U, RANGE_MIDDLE_L, RANGE_MIDDLE_U, RANGE_UPPER_L,
    RANGE_UPPER_U,
};
use std::ops::RangeInclusive as Range;

pub struct LMU(pub Range<i32>, pub Range<i32>, pub Range<i32>);

impl std::default::Default for LMU {
    fn default() -> Self {
        Self(
            RANGE_LOWER_L..=RANGE_LOWER_U,
            RANGE_MIDDLE_L..=RANGE_MIDDLE_U,
            RANGE_UPPER_L..=RANGE_UPPER_U,
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
    pub div_lm: i32,
    pub div_u: i32,
    pub offset_1: i32,
    pub offset_2: i32,
    pub first: i32,
    pub last: i32,
}

impl std::default::Default for Ranges {
    fn default() -> Self {
        Self {
            ranges: LMU::default(),
            div_lm: MANTISSA,
            div_u: MANTISSA,
            offset_1: MANTISSA + 1,
            offset_2: MANTISSA * 2 + 1,
            first: RANGE_LOWER_L,
            last: RANGE_UPPER_U,
        }
    }
}

impl From<Ranges> for Normalizer {
    fn from(ranges: Ranges) -> Self {
        Self {
            ranges: ranges.ranges,
            div_lm: ranges.div_lm,
            div_u: ranges.div_u,
            offset_1: ranges.offset_1,
            offset_2: ranges.offset_2,
            first: ranges.first,
            last: ranges.last,
        }
    }
}

pub struct RangesBuilder {
    lower: Range<i32>,
    middle: Range<i32>,
    upper: Range<i32>,
}

impl std::default::Default for RangesBuilder {
    fn default() -> Self {
        Self {
            lower: 0..=0,
            middle: 0..=0,
            upper: 0..=0,
        }
    }
}

impl From<RangesBuilder> for Ranges {
    fn from(builder: RangesBuilder) -> Self {
        let len_ = builder.iter().map(len).sum::<i32>();
        let ranges = builder.lmu();
        let div = len_ / 3;
        let (first, last) = set_first_last(&ranges);

        Self {
            div_lm: div,
            div_u: div + len_ % div,
            offset_1: len(&ranges.0),
            offset_2: len(&ranges.0) + len(&ranges.1),
            first,
            last,
            ranges,
        }
    }
}

impl RangesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_lower(mut self, r: Range<i32>) -> Self {
        self.lower = r;
        self
    }

    pub fn with_middle(mut self, r: Range<i32>) -> Self {
        self.middle = r;
        self
    }

    pub fn with_upper(mut self, r: Range<i32>) -> Self {
        self.upper = r;
        self
    }

    pub fn build(mut self) -> Option<Ranges> {
        if self.iter().all(|r| *r.start() == 0 && *r.end() == 0) {
            return Some(Ranges::default());
        }

        let l = self.lower;
        let m = self.middle;
        let u = self.upper;

        let mut lmu = [l.start(), l.end(), m.start(), m.end(), u.start(), u.end()];
        lmu.sort();

        self.lower = lmu[0].clone()..=lmu[1].clone();
        self.middle = lmu[2].clone()..=lmu[3].clone();
        self.upper = lmu[4].clone()..=lmu[5].clone();
        let len: i32 = self.iter().map(len).sum();

        if len > NUM_OF_SUPPORTED_Z {
            panic!(
                "OUT OF RANGE: The maximum number of supported z-indexes is {}; your current counter is {}.",
                NUM_OF_SUPPORTED_Z,
                len,
            )
        }

        Some(self.into())
    }

    fn iter(&self) -> impl Iterator<Item = &Range<i32>> {
        vec![&self.lower, &self.middle, &self.upper].into_iter()
    }

    fn lmu(self) -> LMU {
        LMU(self.lower, self.middle, self.upper)
    }
}

pub fn len(r: &Range<i32>) -> i32 {
    r.end() - r.start() + 1 // becuase Range is inclusive
}

fn set_first_last(ranges: &LMU) -> (i32, i32) {
    let mut first = i32::MAX;
    let mut last = -i32::MAX;
    let mut iter = ranges.iter();
    while let Some(r) = iter.next() {
        let (start, end) = (r.start().clone(), r.end().clone());
        if start < first {
            first = start;
        }
        if end > last {
            last = end;
        }
    }
    (first, last)
}
