//! ## Introduction  
//! 3511 emojis and 4580 emoji variants with localization data in 143 languages  
//! This crate contains a huge amount of data about every emoji ever.
//! Some of the data includes:
//! - Name
//! - Glyph
//! - Unicode Release Version
//! - Classification
//! - Variants
//! - Annotations in many languages
//! 
//! This crate also provides functions for searching through emojis by
//! [name](lookup_by_name/index.html) and [glyph](lookup_by_glyph/index.html),
//! as well as several [fuzzy search](search/index.html) functions.
//! ## Quickstart  
//! ```rust
//! fn main() {
//!    println!("{}", emoji::food_and_drink::food_marine::CRAB.glyph);
//! }
//! ```
//! See more examples [here](https://github.com/Shizcow/emoji-rs/tree/master/examples/).
//! ## Languages  
//! By default, only English annotations are compiled in.
//! To enable other languages, use the feature corresponding to that languge. An exhaustive
//! list of supported languages can be found
//! [here](https://github.com/Shizcow/emoji-rs/blob/master/emoji/Cargo.toml).



/// Emoji status qualifier  
/// In nearly every case, MinimallyQualified or Unqualified will show up in emoji variants.
/// A complete tool needs only to support all of the FullyQualified emojis.
#[derive(Debug, PartialEq)]
pub enum Status {
    /// A qualified emoji character, or an emoji sequence in which each emoji character is qualified. Most emojis fall into this category.
    FullyQualified,
    /// An emoji sequence in which the first character is qualified but the sequence is not fully qualified.
    MinimallyQualified,
    /// An emoji that is neither fully-qualified nor minimally qualified.
    Unqualified,
    /// Used for modifiers, such as skin tone modifiers.
    Component,
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	use Status::*;
	write!(
	    f,
	    "{}",
	    match self {
		Component => "Component",
		FullyQualified => "FullyQualified",
		MinimallyQualified => "MinimallyQualified",
		Unqualified => "Unqualified",
	    }
	)
    }
}

/// Contains all information about an emoji  
/// See the [CLDR](https://raw.githubusercontent.com/unicode-org/cldr/release-38/tools/java/org/unicode/cldr/util/data/emoji/emoji-test.txt) for specific examples of all fields except `variants`.
#[derive(Debug, PartialEq)]
pub struct Emoji {
    /// The ASCII-formatted string representation of this emoji's UTF8 codepoint value  
    /// Ex: `1F441 200D 1F5E8 FE0F`
    pub codepoint: &'static str,
    /// Qualification status
    pub status: Status,
    /// The actual emoji text  
    /// Ex: 😺
    pub glyph: &'static str,
    /// The Unicode release version which this emoji was introduced in
    pub introduction_version: f32,
    /// English [CLDR Short Name](https://unicode.org/emoji/format.html#col-name)
    /// (canonical) name of this emoji  
    /// Ex: `grinning cat`
    pub name: &'static str,
    /// General classification this emoji belongs to  
    /// Ex: `Smileys & Emotion`
    pub group: &'static str,
    /// Specific classification this emoji belongs to  
    /// Ex: `cat-face`
    pub subgroup: &'static str,
    /// All variants of an emoji. If two emojis share the same name, one is a variant.
    /// Variants are always less qualified than their parent. Parents can be found from a
    /// variant via [emoji::lookup_by_glyph::lookup](lookup_by_glyph/fn.lookup.html)
    pub variants: &'static [Emoji],
    /// Is this emoji a variant?
    pub is_variant: bool,
    /// Localizatoin specific annotations
    pub annotations: &'static [Annotation],
}

/// Annotation meta-data for each emoji
#[derive(Debug, PartialEq)]
pub struct Annotation {
    /// Language code of the associated data. Guarenteed to be found in
    /// [ANNOTATION_LANGS_AVAILABLE](constant.ANNOTATION_LANGS_AVAILABLE.html)
    pub lang: &'static str,
    /// Localized name for an emoji  
    /// Ex: `fried shrimp`
    pub tts: Option<&'static str>,
    /// Keywords associated with an emoji, in the localized language  
    /// Ex: `["fried shrimp", "shrimp", "prawn"]`
    pub keywords: &'static [&'static str],
}


/// Defines functions for searching through and iterating over emojis by glyph  
/// Includes variants
pub mod lookup_by_glyph;

/// Defines functions for searching through and iterating over emojis by name  
/// Yields exact matches only, but is extremely fast  
/// Does not include variants
pub mod lookup_by_name;

/// Fuzzy search algorithms for general purpose searching
pub mod search;
