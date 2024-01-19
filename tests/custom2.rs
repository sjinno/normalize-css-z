use normalize_css_z::{
    normalizer::Normalizer,
    ranges::{range, RangesBuilder},
};

#[test]
fn test_custom() {
    let range = range(0, 100);
    let builder = RangesBuilder::default().with_middle(range.clone());
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

    assert_eq!(
        counter,
        norm.ranges.into_iter().map(|r| r.count() as i32).sum()
    );
    assert_eq!(prev, 1.0);
}
