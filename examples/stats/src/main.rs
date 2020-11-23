fn main() {
    println!("This crate knows {} emojis, {} emoji variants in {} different languages!",
	     emoji::lookup_by_name::iter_emoji().len(),
	     emoji::lookup_by_glyph::iter_emoji().len(),
	     emoji::ANNOTATION_LANGS_TOTAL.len());
}
