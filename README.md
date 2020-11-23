# emoji-rs

<a href="https://docs.rs/emoji"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

## Introduction  
3511 emojis and 4580 emoji variants with localization data in 143 languages  
This crate contains a huge amount of data about every emoji ever. 
Some of the data includes:  
- Name  
- Glyph  
- Unicode Release Version  
- Classification  
- Variants  
- Annotations in many languages  
This crate also provides functions for searching through emojis by 
name and glyph, as well as several fuzzy search functions. 
## Quickstart  
```rust
fn main() {
 println!("{}", emoji::food_and_drink::food_marine::CRAB.glyph); 
}
```  
See more examples [here](https://github.com/Shizcow/emoji-rs/tree/master/examples/). 
## Languages  
By default, only English annotations are compiled in.  
To enable other languages, use the feature corresponding to that languge. An exhaustive 
list of supported languages can be found
[here](https://github.com/Shizcow/emoji-rs/blob/master/emoji/Cargo.toml). 


# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `emoji` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
