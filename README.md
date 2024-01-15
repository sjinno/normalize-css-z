# Normalization of a CSS z-index value to a 32-bit floating-point number

[![crate](https://img.shields.io/crates/v/normalize-css-z.svg)](https://crates.io/crates/normalize-css-z)
[![documentation](https://docs.rs/normalize-css-z/badge.svg)](https://docs.rs/normalize-css-z)
[![build status](https://github.com/sjinno/normalize-css-z/actions/workflows/rust.yml/badge.svg)](https://github.com/sjinno/normalize-css-z/actions)

## Origin

I struggled to map CSS z-index values to 32-bit floating-point numbers between 0.0 and 1.0 because dividing a large number by another large number, such as `2_147_483_646.0 / 2_147_483_647.0`, does not yield precise results.

To illustrate the issue, here are some examples (see [playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d46e7034b55a1fd0362d1ef3f000304a)):

```
2_147_483_646.0 / 2_147_483_647.0 = 1.0
2_147_483_645.0 / 2_147_483_647.0 = 1.0
2_147_483_644.0 / 2_147_483_647.0 = 1.0
2_147_483_643.0 / 2_147_483_647.0 = 1.0
2_147_483_642.0 / 2_147_483_647.0 = 1.0
2_147_483_641.0 / 2_147_483_647.0 = 1.0
2_147_483_640.0 / 2_147_483_647.0 = 1.0
2_147_483_639.0 / 2_147_483_647.0 = 1.0
2_147_483_638.0 / 2_147_483_647.0 = 1.0
2_147_483_637.0 / 2_147_483_647.0 = 1.0
```

I spent some time experimenting to figure out a reasonable approach to this challenge, and I may have finally found a way to manage the headache of mapping CSS z-index values.

It's important to note that this approach is quite basic and does not guarantee 100% precision. In other words, further stress testing is necessary to ensure the reliability of this implementation.

Additionally, I haven't done any parameter tuning yet, so there could very well be a more efficient method to achieve this that I haven’t discovered.

## Usage

Run the following Cargo command in your project directory:

```zsh
cargo add normalize-css-z
```

Or add the following line to your Cargo.toml:

```toml
[dependencies]
normalize-css-z = "0.4"
```

## Supported ranges of z-indexes

| lower                             | middle                   | upper                           |
| --------------------------------- | ------------------------ | ------------------------------- |
| `-2_147_483_647..=-2_139_095_039` | `-4_194_303..=4_194_304` | `2_139_095_040..=2_147_483_647` |

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.
