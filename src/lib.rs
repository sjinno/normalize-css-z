//! Normalize a CSS z-index to an f32 floating-point number between 0.0 and 1.0. (not quite...)
//!
//! Theoretically, this is not entirely feasible because we can generate only
//! `2^23 (subnormal) * 127 (normal) = 1,065,353,216` distinct floating-point numbers in this range.
//!
//! And, the best part is that we only get a handful of precise numbers. Specifically, this crate
//! can generate `25,165,825` unique floating-point numbers between 0.0 and 1.0, which should be more
//! than sufficient for most use cases (hopefully).
//!
//! ## Supported ranges of z-indexes
//!
//! ```markdown
//! | lower                             | middle                   | upper                           |
//! | --------------------------------- | ------------------------ | ------------------------------- |
//! | `-2_147_483_647..=-2_139_095_039` | `-4_194_303..=4_194_304` | `2_139_095_040..=2_147_483_647` |
//! ```
//!
//! Later, I aim to expand this to allow for customizable ranges, but for now, this should be adequate.

const MAX_CSS_Z: i32 = 2147483647;
const MANTISSA: i32 = 8388608;

const EXP_OFFSET1: i32 = 0;
const EXP_OFFSET2: i32 = 1;
const EXP_OFFSET3: i32 = 2;

const RANGE_UPPER_U: i32 = MAX_CSS_Z;
const RANGE_UPPER_L: i32 = MAX_CSS_Z - MANTISSA + 1;

const RANGE_MIDDLE_U: i32 = MANTISSA / 2;
const RANGE_MIDDLE_L: i32 = -MANTISSA / 2 + 1;

const RANGE_LOWER_U: i32 = -MAX_CSS_Z + MANTISSA;
const RANGE_LOWER_L: i32 = -MAX_CSS_Z;

/// Normalizes a CSS z-index to an f32 floating-point number between 0.0 and 1.0.
///
/// ```
/// # use normalize_css_z::normalize;
/// # fn main() {
/// let z_ = 2_147_483_647;
/// let z = normalize(z_);
/// assert_eq!(z, 1.0);
/// # }
/// ```
pub fn normalize(z: i32) -> f32 {
    assert!(
        (z >= RANGE_UPPER_L)
            || (RANGE_MIDDLE_L <= z && z <= RANGE_MIDDLE_U)
            || (RANGE_LOWER_L <= z && z <= RANGE_LOWER_U),
        "Unsupported z-index value: {}",
        z
    );

    match z {
        RANGE_UPPER_U => 1.0,
        RANGE_LOWER_L => 0.0,
        RANGE_UPPER_L..=RANGE_UPPER_U => normalize_helper(z, RANGE_UPPER_U, EXP_OFFSET1),
        RANGE_MIDDLE_L..=RANGE_MIDDLE_U => normalize_helper(z, RANGE_MIDDLE_U, EXP_OFFSET2),
        RANGE_LOWER_L..=RANGE_LOWER_U => normalize_helper(z, RANGE_LOWER_U, EXP_OFFSET3),
        _ => unreachable!(),
    }
}

fn normalize_helper(z_: i32, upper_bound: i32, exp_offset: i32) -> f32 {
    let z = upper_bound - z_;
    let quo = z / MANTISSA;
    let rem = z % MANTISSA;
    let normal = 2f32.powi(-quo - exp_offset);

    // Returns the `n`th subnormal number for the given `normal`.
    f32::from_bits(normal.to_bits() - rem as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_unsupported_z1() {
        normalize(RANGE_UPPER_L - 1);
    }
    #[test]
    #[should_panic]
    fn test_unsupported_z2() {
        normalize(RANGE_MIDDLE_U + 1);
    }
    #[test]
    #[should_panic]
    fn test_unsupported_z3() {
        normalize(RANGE_MIDDLE_L - 1);
    }
    #[test]
    #[should_panic]
    fn test_unsupported_z4() {
        normalize(RANGE_LOWER_L - 1);
    }
    #[test]
    #[should_panic]
    fn test_unsupported_z5() {
        normalize(RANGE_LOWER_U + 1);
    }

    #[test]
    fn test_supported_z1() {
        normalize(RANGE_UPPER_U);
    }
    #[test]
    fn test_supported_z2() {
        normalize(RANGE_UPPER_L);
    }
    #[test]
    fn test_supported_z3() {
        normalize(RANGE_MIDDLE_L);
    }
    #[test]
    fn test_supported_z4() {
        normalize(RANGE_MIDDLE_U);
    }
    #[test]
    fn test_supported_z5() {
        normalize(RANGE_LOWER_L);
    }
    #[test]
    fn test_supported_z6() {
        normalize(RANGE_LOWER_U);
    }

    #[ignore]
    #[test]
    fn test_normalize_upper() {
        let mut prev = -f32::MIN_POSITIVE;
        let mut count = 0;
        for i in RANGE_UPPER_L..=RANGE_UPPER_U {
            count += 1;
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} < {}", curr, prev);
            prev = curr;
        }
        eprintln!("{count}");
    }

    #[ignore]
    #[test]
    fn test_normalize_middle() {
        let mut prev = -f32::MIN_POSITIVE;
        let mut count = 0;
        for i in RANGE_MIDDLE_L..=RANGE_MIDDLE_U {
            count += 1;
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} < {}", curr, prev);
            prev = curr;
        }
        eprintln!("{count}");
    }

    #[ignore]
    #[test]
    fn test_normalize_lower() {
        let mut prev = -f32::MIN_POSITIVE;
        let mut count = 0;
        for i in RANGE_LOWER_L..=RANGE_LOWER_U {
            count += 1;
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} < {}", curr, prev);
            prev = curr;
        }
        eprintln!("{count}");
    }

    #[ignore]
    #[test]
    fn test_continuity() {
        let mut prev = -f32::MIN_POSITIVE;
        let curr = normalize(RANGE_LOWER_U);
        assert!(curr > prev, "{}: {} > {}", RANGE_LOWER_U, curr, prev);
        eprintln!("(LU): {curr} : {}", RANGE_LOWER_U);
        prev = curr;
        let curr = normalize(RANGE_MIDDLE_L);
        assert!(curr > prev, "{}: {} > {}", RANGE_MIDDLE_L, curr, prev);
        eprintln!("(ML): {curr} : {}", RANGE_MIDDLE_L);
        prev = curr;
        let curr = normalize(RANGE_MIDDLE_U);
        assert!(curr > prev, "{}: {} > {}", RANGE_MIDDLE_U, curr, prev);
        eprintln!("(MU): {curr} : {}", RANGE_MIDDLE_U);
        prev = curr;
        let curr = normalize(RANGE_UPPER_L);
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
