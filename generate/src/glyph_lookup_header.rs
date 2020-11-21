/// Total number of glyphs available for lookup
pub fn num_glyphs() -> usize {
    GLYPH_LOOKUP_MAP.len()
}

/// Is the glyph provided documented in this crate?
pub fn contains_glyph(glyph: &str) -> bool {
    GLYPH_LOOKUP_MAP.contains_key(glyph)
}

/// Get the emoji::Emoji associated with this glyph
pub fn lookup(glyph: &str) -> Option<&'static crate::Emoji> {
    GLYPH_LOOKUP_MAP.get(glyph)
}

/// An iterator over every glyph->emoji::Emoji pair
pub fn iter_glyph_emoji() -> phf::map::Entries<'static, &'static str, crate::Emoji> {
    GLYPH_LOOKUP_MAP.entries()   
}

/// An iterator over every glyph
pub fn iter_glyph() -> phf::map::Keys<'static, &'static str, crate::Emoji> {
    GLYPH_LOOKUP_MAP.keys()
}

/// An iterator over every emoji::Emoji
pub fn iter_emoji() -> phf::map::Values<'static, &'static str, crate::Emoji> {
    GLYPH_LOOKUP_MAP.values()
}
