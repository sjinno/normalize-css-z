//! Normalize a CSS z-index to an f32 floating-point number between 0.0 and 1.0.
//!
//! [Try it in the Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=53960a8bf2314565fd0f8507b833fe71)

#![no_std]

const MAX_CSS_Z: i64 = 2_147_483_647;
const DIV: i64 = 499; // a divisor that is an arbitrary prime number
const EPS: f32 = f32::EPSILON;
// This number looks arbitray, but it's actually the return value of
// `normalize(MAX_CSS_Z)` before dividing by `NORMALIZATION_FACTOR`.
const NORMALIZATION_FACTOR: f32 = 1.0261047;

pub fn normalize(z: i32) -> f32 {
    let z = z as i64 + MAX_CSS_Z;
    let quo = (z / DIV) as f32;
    let rem = (z % DIV) as f32;

    (EPS * quo + EPS * rem) / NORMALIZATION_FACTOR
}

#[cfg(test)]
mod tests {
    use super::*;

    const HALF_MAX_CSS_Z: i32 = MAX_CSS_Z as i32 / 2;

    #[rustfmt::skip]
    #[test]
    fn test_normalize_z() {
        assert_eq!(normalize(-2_147_483_647),                  0.0);
        assert_eq!(normalize(-2_147_483_646),                  0.00000011617654);
        assert_eq!(normalize(-2_147_483_645),                  0.00000023235307);
        assert_eq!(normalize(-2_147_483_644),                  0.00000034852962);
        assert_eq!(normalize(-2_147_483_643),                  0.00000046470615);
        assert_eq!(normalize(-2_147_483_642),                  0.00000058088267);
        assert_eq!(normalize(-2_147_483_641),                  0.00000069705925);
        assert_eq!(normalize(-2_147_483_640),                  0.0000008132358);
        assert_eq!(normalize(-2_147_483_647 + HALF_MAX_CSS_Z), 0.24999994);
        assert_eq!(normalize(-2_147_483_646 + HALF_MAX_CSS_Z), 0.25000006);
        assert_eq!(normalize(-2_147_483_645 + HALF_MAX_CSS_Z), 0.25000018);
        assert_eq!(normalize(-2_147_483_644 + HALF_MAX_CSS_Z), 0.2500003);
        assert_eq!(normalize(-2_147_483_643 + HALF_MAX_CSS_Z), 0.25000042);
        assert_eq!(normalize(-2_147_483_642 + HALF_MAX_CSS_Z), 0.25000054);
        assert_eq!(normalize(-2_147_483_641 + HALF_MAX_CSS_Z), 0.25000063);
        assert_eq!(normalize(-2_147_483_640 + HALF_MAX_CSS_Z), 0.25000075);
        assert_eq!(normalize(-5),                              0.49999943);
        assert_eq!(normalize(-4),                              0.49999952);
        assert_eq!(normalize(-3),                              0.49999964);
        assert_eq!(normalize(-2),                              0.49999976);
        assert_eq!(normalize(-1),                              0.49999988);
        assert_eq!(normalize(0),                               0.5);
        assert_eq!(normalize(1),                               0.5000001);
        assert_eq!(normalize(2),                               0.50000024);
        assert_eq!(normalize(3),                               0.50000036);
        assert_eq!(normalize(4),                               0.5000005);
        assert_eq!(normalize(5),                               0.5000006);
        assert_eq!(normalize(2_147_483_640 - HALF_MAX_CSS_Z),  0.7499992);
        assert_eq!(normalize(2_147_483_641 - HALF_MAX_CSS_Z),  0.74999934);
        assert_eq!(normalize(2_147_483_642 - HALF_MAX_CSS_Z),  0.74999946);
        assert_eq!(normalize(2_147_483_643 - HALF_MAX_CSS_Z),  0.7499996);
        assert_eq!(normalize(2_147_483_644 - HALF_MAX_CSS_Z),  0.7499997);
        assert_eq!(normalize(2_147_483_645 - HALF_MAX_CSS_Z),  0.7499998);
        assert_eq!(normalize(2_147_483_646 - HALF_MAX_CSS_Z),  0.74999994);
        assert_eq!(normalize(2_147_483_647 - HALF_MAX_CSS_Z),  0.75000006);
        assert_eq!(normalize(2_147_483_640),                   0.99999917);
        assert_eq!(normalize(2_147_483_641),                   0.9999993);
        assert_eq!(normalize(2_147_483_642),                   0.9999994);
        assert_eq!(normalize(2_147_483_643),                   0.9999995);
        assert_eq!(normalize(2_147_483_644),                   0.99999964);
        assert_eq!(normalize(2_147_483_645),                   0.99999976);
        assert_eq!(normalize(2_147_483_646),                   0.9999999);
        assert_eq!(normalize(2_147_483_647),                   1.0);
    }
}
