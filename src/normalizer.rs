use crate::{
    ranges::{Ranges, LMU},
    MANTISSA,
};

pub const NUM_OF_SUPPORTED_Z: i32 = MANTISSA * 3 + 1;

pub struct Normalizer {
    pub ranges: LMU,
    pub div_lm: i32,
    pub div_u: i32,
    pub offset_1: i32,
    pub offset_2: i32,
    pub first: i32,
    pub last: i32,
}

impl std::default::Default for Normalizer {
    fn default() -> Self {
        Ranges::default().into()
    }
}

impl Normalizer {
    pub fn new(ranges: Option<Ranges>) -> Self {
        ranges.unwrap_or_default().into()
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
    pub fn calc(&self, z: i32) -> Option<f32> {
        fn helper(n: i32, div: &i32, p: i32) -> Option<f32> {
            let rem = n % div;
            let normal = 2f32.powi(p);

            // Returns the `n`th subnormal number for the given `normal`.
            Some(f32::from_bits(normal.to_bits() - (div - rem) as u32))
        }

        let Normalizer {
            ranges,
            div_lm,
            div_u,
            offset_1,
            offset_2,
            first,
            last,
        } = self;

        match z {
            z_ if z_ == *first => Some(0.0),
            z_ if z_ == *last => Some(1.0),
            _ => {
                if let Some((i, r)) = ranges.iter().enumerate().find(|&(_, r)| r.contains(&z)) {
                    let n = match i {
                        0 => z - r.start(),
                        1 => z - r.start() + offset_1,
                        2 => z - r.start() + offset_2,
                        _ => unreachable!(),
                    };

                    let bound_mu = div_lm * 2;

                    match &n {
                        n_ if (0..*div_lm).contains(n_) => helper(n, div_lm, -2),
                        n_ if (*div_lm..bound_mu).contains(n_) => helper(n - div_lm, div_lm, -1),
                        n_ if (bound_mu..=bound_mu + div_u).contains(n_) => {
                            helper(n - bound_mu, div_u, 0)
                        }
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
            }
        }
    }
}
