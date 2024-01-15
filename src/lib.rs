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
const MANTISSA: i32 = 8388608;
const MANTISSA2: i32 = 8388608 * 2;

const EXP: i32 = 7;
const EXP_OFFSET1: i32 = 0;
const EXP_OFFSET2: i32 = (EXP + 1) / 4;
const EXP_OFFSET3: i32 = EXP_OFFSET2 * 3;

const RANGE_1: (i32, i32) = (MAX_CSS_Z - MANTISSA2 + 1, MAX_CSS_Z);
const RANGE_2: (i32, i32) = (-MANTISSA2, MANTISSA2 - 1);
const RANGE_3: (i32, i32) = (-MAX_CSS_Z, -MAX_CSS_Z + MANTISSA2);

pub fn normalize(z: i32) -> f32 {
    if z == MAX_CSS_Z {
        return 1.0;
    } else if z == -MAX_CSS_Z {
        return 0.0;
    }

    assert!(
        (z >= RANGE_1.0)
            || (RANGE_2.0 <= z && z <= RANGE_2.1)
            || (RANGE_3.0 <= z && z <= RANGE_3.1),
        "Unsupported z-index value: {}",
        z
    );

    match z {
        _ if RANGE_1.0 <= z && z <= RANGE_1.1 => normalize_helper(z, RANGE_1.1, EXP_OFFSET1),
        _ if RANGE_2.0 <= z && z <= RANGE_2.1 => normalize_helper(z, RANGE_2.1, EXP_OFFSET2),
        _ if RANGE_3.0 <= z && z <= RANGE_3.1 => normalize_helper(z, RANGE_3.1, EXP_OFFSET3),
        _ => unreachable!(),
    }
}

fn normalize_helper(z_: i32, upper_bound: i32, exp_offset: i32) -> f32 {
    let z = upper_bound - z_;
    let quo = z / MANTISSA;
    let rem = z % MANTISSA;
    let normal = 2f32.powi(-quo - exp_offset);
    get_nth_subnormal(rem as u32, normal)
}

// Generates the `n`th subnormal number for the given `normal`.
pub fn get_nth_subnormal(n: u32, normal: f32) -> f32 {
    let bits = normal.to_bits();
    f32::from_bits(bits - n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_unsupported_z1() {
        normalize(RANGE_1.0 - 1);
    }
    #[test]
    #[should_panic]
    fn test_unsupported_z2() {
        normalize(RANGE_2.1 + 1);
    }
    #[test]
    #[should_panic]
    fn test_unsupported_z3() {
        normalize(RANGE_2.0 - 1);
    }
    #[test]
    #[should_panic]
    fn test_unsupported_z4() {
        normalize(RANGE_3.0 - 1);
    }
    #[test]
    #[should_panic]
    fn test_unsupported_z5() {
        normalize(RANGE_3.1 + 1);
    }

    #[test]
    fn test_supported_z1() {
        normalize(RANGE_1.1);
    }
    #[test]
    fn test_supported_z2() {
        normalize(RANGE_1.0);
    }
    #[test]
    fn test_supported_z3() {
        normalize(RANGE_2.0);
    }
    #[test]
    fn test_supported_z4() {
        normalize(RANGE_2.1);
    }
    #[test]
    fn test_supported_z5() {
        normalize(RANGE_3.0);
    }
    #[test]
    fn test_supported_z6() {
        normalize(RANGE_3.1);
    }

    #[test]
    fn test_normalize_upper() {
        let mut prev = -f32::MIN_POSITIVE;
        let mut count = 0;
        for i in RANGE_1.0..=RANGE_1.1 {
            count += 1;
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} < {}", curr, prev);
            prev = curr;
        }
        eprintln!("{count}");
    }

    #[test]
    fn test_normalize_middle() {
        let mut prev = -f32::MIN_POSITIVE;
        let mut count = 0;
        for i in RANGE_2.0..=RANGE_2.1 {
            count += 1;
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} < {}", curr, prev);
            prev = curr;
        }
        eprintln!("{count}");
    }

    #[test]
    fn test_normalize_lower() {
        let mut prev = -f32::MIN_POSITIVE;
        let mut count = 0;
        for i in RANGE_3.0..=RANGE_3.1 {
            count += 1;
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} < {}", curr, prev);
            prev = curr;
        }
        eprintln!("{count}");
    }

    #[test]
    fn test_normalize_check() {
        test_normalize_lower();
        test_normalize_middle();
        test_normalize_upper();
        test_continuity();
    }

    #[test]
    fn test_continuity() {
        let mut prev = -f32::MIN_POSITIVE;
        for i in RANGE_3.0..=RANGE_3.0 + 10 {
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} > {}", curr, prev);
            eprintln!("(L1): {curr} : {i}");
            prev = curr;
        }
        for i in RANGE_3.1 - 10..=RANGE_3.1 {
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} > {}", curr, prev);
            eprintln!("(L2): {curr} : {i}");
            prev = curr;
        }
        for i in RANGE_2.0..=RANGE_2.0 + 10 {
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} > {}", curr, prev);
            eprintln!("(M1): {curr} : {i}");
            prev = curr;
        }
        for i in RANGE_2.1 - 10..=RANGE_2.1 {
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} > {}", curr, prev);
            eprintln!("(M2): {curr} : {i}");
            prev = curr;
        }
        for i in RANGE_1.0..=RANGE_1.0 + 10 {
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} > {}", curr, prev);
            eprintln!("(U1): {curr} : {i}");
            prev = curr;
        }
        for i in RANGE_1.1 - 10..=RANGE_1.1 {
            let curr = normalize(i);
            assert!(curr > prev, "{i}: {} > {}", curr, prev);
            eprintln!("(U2): {curr} : {i}");
            prev = curr;
        }
    }
}
