#[cfg(feature = "custom")]
use normalize_css_z::{
    normalizer::Normalizer,
    ranges::{len, RangesBuilder},
};

#[test]
fn test_custom3() {
    #[cfg(feature = "custom")]
    {
        let lower = 0..=100;
        let middle = 101..=200;
        let upper = 201..=300;

        let builder = RangesBuilder::default()
            .with_lower(lower.clone())
            .with_middle(middle.clone())
            .with_upper(upper.clone());
        let norm = Normalizer::new(builder.build());

        assert_eq!(norm.calc(0), Some(0.0));

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

        assert_eq!(counter, norm.ranges.iter().map(len).sum::<i32>());
        assert_eq!(prev, 1.0);
    }
}
