//use fuzzy_matcher::skim::SkimMatcherV2;
//use fuzzy_matcher::FuzzyMatcher;

fn main() {
    println!(
        "The {} emoji is at codepoint {}, called {}, is {}, and was introduced in unicode version {}",
        emoji::food_and_drink::food_marine::CRAB.glyph,
        emoji::food_and_drink::food_marine::CRAB.codepoint,
        emoji::food_and_drink::food_marine::CRAB.name,
        emoji::food_and_drink::food_marine::CRAB.status,
        emoji::food_and_drink::food_marine::CRAB.introduction_version
    );
    println!("{}", emoji::lookup_by_name::lookup("crab").unwrap().glyph);

    println!("{:?}", emoji::search::search_annotation_all("näyttävä").into_iter().map(|e| e.name).collect::<Vec<_>>()[0]);
    

    /*
    for e in emoji::lookup_by_name::iter_emoji() {
	if let Some(score) = matcher.fuzzy_match(e.name, searchterm) {
	    println!("{}", score);
	}
    }*/
}
