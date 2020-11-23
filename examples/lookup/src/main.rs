fn main() {
    println!("There's only one emoji with the name 'shrimp'. It's: {}", emoji::lookup_by_name::lookup("shrimp").unwrap().glyph);

    println!("There's no emoji called 'prawn'. However, searching through annotations for 'prawn' and taking the top result yields: {}",
             emoji::search::search_annotation("prawn", "en").into_iter().map(
		 |e| format!("{} ({})", e.glyph, e.name)
	     ).nth(0).unwrap()
    );
}
