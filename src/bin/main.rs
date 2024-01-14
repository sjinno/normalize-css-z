use normalize_css_z::*;

fn main() {
    let mantissa = 8388608;
    let mut count = 0;

    for p in 0..=126 {
        let normal = 2f32.powi(-p);
        let mut curr = normal;

        for i in 0..(1 << 23) {
            let new = get_nth_subnormal(i + 1, normal);
            assert!(new < curr && curr <= 1.0 && curr > 0.0);
            curr = new;
        }

        count += mantissa;
    }

    println!("{count}");
}
