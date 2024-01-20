#[cfg(feature = "custom")]
use normalize_css_z::{
    normalizer::Normalizer,
    ranges::{len, RangesBuilder},
};

#[test]
fn test_custom4() {
    #[cfg(feature = "custom")]
    {
        let lower = 1..=10;
        let middle = 11..=15;
        let upper = 16..=30;

        let builder = RangesBuilder::default()
            .with_lower(lower.clone())
            .with_middle(middle.clone())
            .with_upper(upper.clone());
        let norm = Normalizer::new(builder.build());

        assert_eq!(norm.calc(1), Some(0.0));

        let mut counter = 0;
        let mut prev = -1.0;

        for r in norm.ranges.iter() {
            for z in r.clone() {
                let curr = norm.calc(z).unwrap();
                if curr <= prev {
                    eprintln!("shohei - curr {curr:?}, prev {prev:?}, z {z:?}");
                }
                assert!(curr > prev);
                prev = curr;
                counter += 1;
            }
        }

        assert_eq!(counter, norm.ranges.iter().map(len).sum::<i32>());
        assert_eq!(prev, 1.0);
    }
}
