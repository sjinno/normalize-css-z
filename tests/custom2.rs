#[cfg(feature = "custom")]
use normalize_css_z::{
    normalizer::Normalizer,
    ranges::{len, RangesBuilder},
};

#[test]
fn test_custom2() {
    #[cfg(feature = "custom")]
    {
        let range = 0..=100;
        let builder = RangesBuilder::default().with_middle(range.clone());
        let norm = Normalizer::new(builder.build());

        let mut counter = 0;
        let mut prev = -1.0;

        for r in norm.ranges.iter() {
            for z in r.clone() {
                let curr = norm.calc(z).unwrap();
                assert!(curr >= prev);
                prev = curr;
                counter += 1;
            }
        }

        assert_eq!(counter, norm.ranges.iter().map(len).sum::<i32>());
        assert_eq!(prev, 1.0);
    }
}
