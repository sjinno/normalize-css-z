//! Normalize a CSS z-index to an f32 floating-point number between 0.0 and 1.0. (not quite...)
//!
//! Theoretically, this is not entirely feasible because we can generate only
//! `2^23 (subnormal) * 127 (normal) = 1065353216` distinct floating-point numbers in this range.
//!
//! And, the best part is that we only get a handful of precise numbers. Specifically, this crate
//! can generate 67,108,865 unique floating-point numbers between 0.0 and 1.0, which should be more
//! than sufficient for most use cases (hopefully).
//!
//! ## Supported ranges of z-indexes
//!
//! ```markdown
//! | upper                    | middle                  | lower                      |
//! | ------------------------ | ----------------------- | -------------------------- |
//! | [2130706432, 2147483647] | [-16777216, 16777216-1] | [-2147483647, -2130706431] |
//! ```
//!
//! Later, I aim to expand this to allow for customizable ranges, but for now, this should be adequate.

const MAX_CSS_Z: i32 = 2147483647;

const EXP: i32 = 7;
const X: i32 = (EXP + 1) / 4;

const EXP_OFFSET1: i32 = 0;
const EXP_OFFSET2: i32 = X;
const EXP_OFFSET3: i32 = EXP_OFFSET2 * 3;

const MANTISSA: i32 = 8388608;
const MANTISSA_X: i32 = 8388608 * X;

const RANGE_UPPER_1: i32 = MAX_CSS_Z;
const RANGE_LOWER_1: i32 = MAX_CSS_Z - MANTISSA_X + 1;

const RANGE_UPPER_2: i32 = MANTISSA_X - 1;
const RANGE_LOWER_2: i32 = -MANTISSA_X;

const RANGE_UPPER_3: i32 = -MAX_CSS_Z + MANTISSA_X;
const RANGE_LOWER_3: i32 = -MAX_CSS_Z;

pub fn normalize(z: i32) -> f32 {
    assert!(
        (z >= RANGE_LOWER_1)
            || (RANGE_LOWER_2 <= z && z <= RANGE_UPPER_2)
            || (RANGE_LOWER_3 <= z && z <= RANGE_UPPER_3),
        "Unsupported z-index value: {}",
        z
    );

    match z {
        RANGE_UPPER_1 => 1.0,
        RANGE_LOWER_3 => 0.0,
        RANGE_LOWER_1..=RANGE_UPPER_1 => normalize_helper(z, RANGE_UPPER_1, EXP_OFFSET1),
        RANGE_LOWER_2..=RANGE_UPPER_2 => normalize_helper(z, RANGE_UPPER_2, EXP_OFFSET2),
        RANGE_LOWER_3..=RANGE_UPPER_3 => normalize_helper(z, RANGE_UPPER_3, EXP_OFFSET3),
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
        normalize(RANGE_LOWER_1 - 1);
    }
    #[test]
    #[should_panic]
    fn test_unsupported_z2() {
        normalize(RANGE_UPPER_2 + 1);
    }
    #[test]
    #[should_panic]
    fn test_unsupported_z3() {
        normalize(RANGE_LOWER_2 - 1);
    }
    #[test]
    #[should_panic]
    fn test_unsupported_z4() {
        normalize(RANGE_LOWER_3 - 1);
    }
    #[test]
    #[should_panic]
    fn test_unsupported_z5() {
        normalize(RANGE_UPPER_3 + 1);
    }

    #[test]
    fn test_supported_z1() {
        normalize(RANGE_UPPER_1);
    }
    #[test]
    fn test_supported_z2() {
        normalize(RANGE_LOWER_1);
    }
    #[test]
    fn test_supported_z3() {
        normalize(RANGE_LOWER_2);
    }
    #[test]
    fn test_supported_z4() {
        normalize(RANGE_UPPER_2);
    }
    #[test]
    fn test_supported_z5() {
        normalize(RANGE_LOWER_3);
    }
    #[test]
    fn test_supported_z6() {
        normalize(RANGE_UPPER_3);
    }

    #[ignore]
    #[test]
    fn test_normalize_upper() {
        let mut prev = -f32::MIN_POSITIVE;
        let mut count = 0;
        for i in RANGE_LOWER_1..=RANGE_UPPER_1 {
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
        for i in RANGE_LOWER_2..=RANGE_UPPER_2 {
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
        for i in RANGE_LOWER_3..=RANGE_UPPER_3 {
            count += 1;
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} < {}", curr, prev);
            prev = curr;
        }
        eprintln!("{count}");
    }

    #[test]
    fn test_normalize_all() {
        test_normalize_lower();
        test_normalize_middle();
        test_normalize_upper();
        test_continuity();
    }

    #[ignore]
    #[test]
    fn test_continuity() {
        let mut prev = -f32::MIN_POSITIVE;
        let curr = normalize(RANGE_UPPER_3);
        assert!(curr > prev, "{}: {} > {}", RANGE_UPPER_3, curr, prev);
        eprintln!("(L2): {curr} : {}", RANGE_UPPER_3);
        prev = curr;
        let curr = normalize(RANGE_LOWER_2);
        assert!(curr > prev, "{}: {} > {}", RANGE_LOWER_2, curr, prev);
        eprintln!("(M1): {curr} : {}", RANGE_LOWER_2);
        prev = curr;
        let curr = normalize(RANGE_UPPER_2);
        assert!(curr > prev, "{}: {} > {}", RANGE_UPPER_2, curr, prev);
        eprintln!("(M2): {curr} : {}", RANGE_UPPER_2);
        prev = curr;
        let curr = normalize(RANGE_LOWER_1);
        assert!(curr > prev, "{}: {} > {}", RANGE_LOWER_1, curr, prev);
        eprintln!("(U1): {curr} : {}", RANGE_LOWER_1);
    }
}
