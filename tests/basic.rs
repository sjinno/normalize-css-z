#[cfg(feature = "custom")]
use normalize_css_z::{normalizer::Normalizer, MANTISSA};

#[test]
fn test_all() {
    #[cfg(feature = "custom")]
    {
        let norm = Normalizer::new(None);

        let mut counter = 0;
        let mut prev = -1.0;

        for r in norm.ranges.iter() {
            for z in r.clone() {
                let curr = norm.calc(z).unwrap();
                assert!(curr > prev);
                prev = curr;
                counter += 1;
            }
        }

        assert_eq!(counter, MANTISSA * 3 + 1);
        assert_eq!(prev, 1.0);
    }
}
