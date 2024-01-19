//! Normalize a CSS z-index to an f32 floating-point number between `0.0` and `1.0`. (not exactly...)
//!
//! Theoretically, this is not entirely feasible because we can generate only
//! `2^23 (subnormal) * 127 (normal) = 1,065,353,216` distinct floating-point numbers in this range.
//!
//! And, the best part is that we only get a handful of precise numbers. Specifically, this crate
//! can generate `25,165,825` unique floating-point numbers between `0.0` and `1.0`, which should be more
//! than sufficient for most use cases (hopefully).
//!
//! ## Supported ranges of z-indexes (default)
//!
//! | LOWER                             | MIDDLE                   | UPPER                           |
//! | --------------------------------- | ------------------------ | ------------------------------- |
//! | `-2_147_483_647..=-2_139_095_039` | `-4_194_303..=4_194_304` | `2_139_095_040..=2_147_483_647` |
//!
//! # Example
//!
//! ```
//! # use normalize_css_z::normalize;
//! # fn main() {
//! let z_ = 2_147_483_647;
//! if let Some(z) = normalize(z_) {
//!     // Do something with `z`.
//! } else {
//!     // Handle unsupported z-index.
//! }
//! # }
//! ```
//!
//! Later, I aim to expand this to allow for customizable ranges, but for now, this should be adequate.

#[cfg(feature = "custom")]
pub mod normalizer;
#[cfg(feature = "custom")]
pub mod ranges;

pub const MAX_CSS_Z: i32 = 2_147_483_647;
pub const MANTISSA: i32 = 8_388_608;

pub const RANGE_UPPER_U: i32 = MAX_CSS_Z;
pub const RANGE_UPPER_L: i32 = MAX_CSS_Z - MANTISSA + 1;

pub const RANGE_MIDDLE_U: i32 = MANTISSA / 2;
pub const RANGE_MIDDLE_L: i32 = -MANTISSA / 2 + 1;

pub const RANGE_LOWER_U: i32 = -MAX_CSS_Z + MANTISSA;
pub const RANGE_LOWER_L: i32 = -MAX_CSS_Z;

/// Normalizes a CSS z-index to an f32 floating-point number between 0.0 and 1.0.
///
/// This is the most hassle-free way to use this crate if you don't need to customize the ranges.
///
/// ```
/// # use normalize_css_z::normalize;
/// # fn main() {
/// let z_ = 2_147_483_647;
/// let z = normalize(z_);
/// assert_eq!(z, Some(1.0));
/// # }
/// ```
pub fn normalize(z: i32) -> Option<f32> {
    fn helper(z_: i32, upper_bound: i32, exp_offset: i32) -> Option<f32> {
        let z = upper_bound - z_;
        let quo = z / MANTISSA;
        let rem = z % MANTISSA;
        let normal = 2f32.powi(-quo - exp_offset);

        // Returns the `n`th subnormal number for the given `normal`.
        Some(f32::from_bits(normal.to_bits() - rem as u32))
    }

    match z {
        RANGE_LOWER_L => Some(0.0),
        RANGE_UPPER_U => Some(1.0),
        RANGE_LOWER_L..=RANGE_LOWER_U => helper(z, RANGE_LOWER_U, 2),
        RANGE_MIDDLE_L..=RANGE_MIDDLE_U => helper(z, RANGE_MIDDLE_U, 1),
        RANGE_UPPER_L..=RANGE_UPPER_U => helper(z, RANGE_UPPER_U, 0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsupported_z1() {
        assert!(normalize(RANGE_UPPER_L - 1).is_none());
    }
    #[test]
    fn test_unsupported_z2() {
        assert!(normalize(RANGE_MIDDLE_U + 1).is_none());
    }
    #[test]
    fn test_unsupported_z3() {
        assert!(normalize(RANGE_MIDDLE_L - 1).is_none());
    }
    #[test]
    fn test_unsupported_z4() {
        assert!(normalize(RANGE_LOWER_L - 1).is_none());
    }
    #[test]
    fn test_unsupported_z5() {
        assert!(normalize(RANGE_LOWER_U + 1).is_none());
    }

    #[test]
    fn test_supported_z1() {
        assert!(normalize(RANGE_UPPER_U).is_some());
    }
    #[test]
    fn test_supported_z2() {
        assert!(normalize(RANGE_UPPER_L).is_some());
    }
    #[test]
    fn test_supported_z3() {
        assert!(normalize(RANGE_MIDDLE_L).is_some());
    }
    #[test]
    fn test_supported_z4() {
        assert!(normalize(RANGE_MIDDLE_U).is_some());
    }
    #[test]
    fn test_supported_z5() {
        assert!(normalize(RANGE_LOWER_L).is_some());
    }
    #[test]
    fn test_supported_z6() {
        assert!(normalize(RANGE_LOWER_U).is_some());
    }

    #[ignore]
    #[test]
    fn test_normalize_upper() {
        let mut prev = -1.0;
        let mut count = 0;
        for i in RANGE_UPPER_L..=RANGE_UPPER_U {
            count += 1;
            let curr = normalize(i).unwrap();
            assert!(curr > prev, "{i}: {} < {}", curr, prev);
            prev = curr;
        }
        eprintln!("{count}");
    }

    #[ignore]
    #[test]
    fn test_normalize_middle() {
        let mut prev = -1.0;
        let mut count = 0;
        for i in RANGE_MIDDLE_L..=RANGE_MIDDLE_U {
            count += 1;
            let curr = normalize(i).unwrap();
            assert!(curr > prev, "{i}: {} < {}", curr, prev);
            prev = curr;
        }
        eprintln!("{count}");
    }

    #[ignore]
    #[test]
    fn test_normalize_lower() {
        let mut prev = -1.0;
        let mut count = 0;
        for i in RANGE_LOWER_L..=RANGE_LOWER_U {
            count += 1;
            let curr = normalize(i).unwrap();
            assert!(curr > prev, "{i}: {} < {}", curr, prev);
            prev = curr;
        }
        eprintln!("{count}");
    }

    #[ignore]
    #[test]
    fn test_continuity() {
        let mut prev = -1.0;
        let curr = normalize(RANGE_LOWER_U).unwrap();
        assert!(curr > prev, "{}: {} > {}", RANGE_LOWER_U, curr, prev);
        eprintln!("(LU): {curr} : {}", RANGE_LOWER_U);
        prev = curr;
        let curr = normalize(RANGE_MIDDLE_L).unwrap();
        assert!(curr > prev, "{}: {} > {}", RANGE_MIDDLE_L, curr, prev);
        eprintln!("(ML): {curr} : {}", RANGE_MIDDLE_L);
        prev = curr;
        let curr = normalize(RANGE_MIDDLE_U).unwrap();
        assert!(curr > prev, "{}: {} > {}", RANGE_MIDDLE_U, curr, prev);
        eprintln!("(MU): {curr} : {}", RANGE_MIDDLE_U);
        prev = curr;
        let curr = normalize(RANGE_UPPER_L).unwrap();
        assert!(curr > prev, "{}: {} > {}", RANGE_UPPER_L, curr, prev);
        eprintln!("(UL): {curr} : {}", RANGE_UPPER_L);
    }

    #[test]
    fn test_normalize_all() {
        test_normalize_lower();
        test_normalize_middle();
        test_normalize_upper();
        test_continuity();
    }
}
