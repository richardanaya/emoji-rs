

fn main() {
    println!(
        "The {} emoji is at codepoint {}, called {}, is {}, and was introduced in unicode version {}",
        emoji::food_and_drink::food_marine::CRAB.glyph,
        emoji::food_and_drink::food_marine::CRAB.codepoint,
        emoji::food_and_drink::food_marine::CRAB.name,
        emoji::food_and_drink::food_marine::CRAB.status,
        emoji::food_and_drink::food_marine::CRAB.introduction_version
    );
    println!("{:?}", emoji::lookup_by_glyph::lookup("🤳"));
    let mut names: Vec<&emoji::Emoji> = emoji::lookup_by_glyph::iter_emoji().collect();
    names.dedup();
    for i in 1..names.len() {
	if names[i-1] == names[i] {
	    println!("{} is a dupe", names[i].name);
	}
    }
}
