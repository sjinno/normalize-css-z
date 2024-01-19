#[cfg(feature = "custom")]
use normalize_css_z::normalizer::Normalizer;
#[cfg(feature = "custom")]
use normalize_css_z::ranges::{range, RangesBuilder};

fn main() {
    #[cfg(feature = "custom")]
    {
        let builder = RangesBuilder::new()
            .with_lower(range(0, 100))
            .with_middle(range(100, 200))
            .with_upper(range(200, 300));
        let normalizer = Normalizer::new(builder.build());

        let smallest = normalizer.calc(0);
        let largest = normalizer.calc(300);
        let mid = normalizer.calc(150);
        let none = normalizer.calc(-2_147_483_647);

        eprintln!("shohei - smallest:? {smallest:?}");
        eprintln!("shohei - largest:? {largest:?}");
        eprintln!("shohei - mid:? {mid:?}");
        eprintln!("shohei - none:? {none:?}");
    }
}
