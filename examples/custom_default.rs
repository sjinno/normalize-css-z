#[cfg(feature = "custom")]
use normalize_css_z::normalizer::Normalizer;
#[cfg(feature = "custom")]
use normalize_css_z::ranges::RangesBuilder;

fn main() {
    #[cfg(feature = "custom")]
    {
        let builder = RangesBuilder::new();
        let normalizer = Normalizer::new(builder.build());

        let smallest = normalizer.calc(-2_147_483_647);
        let largest = normalizer.calc(2_147_483_647);
        let mid = normalizer.calc(0);
        let none = normalizer.calc(-147_483_647);

        eprintln!("shohei - smallest:? {smallest:?}");
        eprintln!("shohei - largest:? {largest:?}");
        eprintln!("shohei - mid:? {mid:?}");
        eprintln!("shohei - none:? {none:?}");
    }
}
