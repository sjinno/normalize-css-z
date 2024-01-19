use normalize_css_z::normalizer::Normalizer;

fn main() {
    let normalizer = Normalizer::new(None);
    let smallest = normalizer.calc(-2_147_483_647);
    let largest = normalizer.calc(2_147_483_647);

    eprintln!("shohei - smallest {smallest:?}");
    eprintln!("shohei - largest {largest:?}");
}
