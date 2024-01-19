use crate::{
    ranges::{Ranges, LMU},
    MANTISSA,
};

pub const NUM_OF_SUPPORTED_Z: usize = MANTISSA as usize * 3 + 1;

pub struct Normalizer {
    pub ranges: LMU,
}

impl std::default::Default for Normalizer {
    fn default() -> Self {
        let Ranges { ranges } = Ranges::default();
        Self { ranges }
    }
}

impl Normalizer {
    pub fn new(ranges: Option<Ranges>) -> Self {
        if let Some(Ranges { ranges }) = ranges {
            Self { ranges }
        } else {
            Self::default()
        }
    }

    /// Normalizes a CSS z-index to an f32 floating-point number between 0.0 and 1.0.
    ///
    /// ```
    /// # use normalize_css_z::normalize;
    /// # fn main() {
    /// let z_ = 2_147_483_647;
    /// let z = normalize(z_);
    /// assert_eq!(z, Some(1.0));
    /// # }
    /// ```
    pub fn calc(&self, z_: i32) -> Option<f32> {
        fn helper(n: i32, exp_offset: i32) -> Option<f32> {
            let quo = n / MANTISSA;
            let rem = n % MANTISSA;
            let normal = 2f32.powi(-quo - exp_offset);

            // Returns the `n`th subnormal number for the given `normal`.
            Some(f32::from_bits(normal.to_bits() - rem as u32))
        }

        let Normalizer { ranges } = self;

        match &z_ {
            z if z == self.ranges.0.start() => Some(0.0),
            z if z == self.ranges.2.end() => Some(1.0),
            _ => {
                if let Some((i, r)) = ranges.iter().enumerate().find(|(_, r)| r.contains(&z_)) {
                    match i {
                        0 => helper(r.end() - z_, 2),
                        1 => helper(r.end() - z_, 1),
                        2 => helper(r.end() - z_, 0),
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
            }
        }
    }
}
