use normalize_css_z::{normalizer::Normalizer, ranges::RangesBuilder, MANTISSA};

#[test]
fn test_custom() {
    let builder = RangesBuilder::default();
    let norm = Normalizer::new(builder.build());

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
