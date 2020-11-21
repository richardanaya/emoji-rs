#[derive(Debug, PartialEq)]
pub enum Status {
    Component,
    FullyQualified,
    MinimallyQualified,
    Unqualified,
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

#[derive(Debug, PartialEq)]
pub struct Emoji {
    pub codepoint: &'static str,
    pub status: Status,
    pub glyph: &'static str,
    pub introduction_version: f32,
    pub name: &'static str,
    pub group: &'static str,
    pub subgroup: &'static str,
    pub variants: &'static [Emoji],
    pub annotations: &'static [Annotation],
    pub is_variant: bool,
}
#[derive(Debug, PartialEq)]
pub struct Annotation {
    pub lang: &'static str,
    pub tts: Option<&'static str>,
    pub keywords: &'static [&'static str],
}


/// " 🦀" goes in, `emoji::food_and_drink::food_marine::CRAB` goes out  
/// Also defines several other utility functions
pub mod lookup_by_glyph;

/// Search for an emoji::Emoji by name. Yields exact matches only, but is extremely fast
pub mod lookup_by_name;
