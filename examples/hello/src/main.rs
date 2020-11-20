fn main() {
    println!("The {} emoji is at codepoint {}, called {}, is {}, and was introduced in unicode version {}",
	     emoji::food_and_drink::food_marine::CRAB.glyph,
	     emoji::food_and_drink::food_marine::CRAB.codepoint,
	     emoji::food_and_drink::food_marine::CRAB.name,
	     emoji::food_and_drink::food_marine::CRAB.status,
	     emoji::food_and_drink::food_marine::CRAB.introduction_version);
    println!("This emoji has the following annotations associated with it: {:?}",
	     emoji::food_and_drink::food_marine::CRAB.annotations.iter()
	     .find(|a| a.lang == "en").unwrap().keywords);
    println!("In Spanish, it's called {}",
	     emoji::food_and_drink::food_marine::CRAB.annotations.iter()
	     .find(|a| a.lang == "es").unwrap().tts.unwrap());
    println!("In Finnish, it's called {}",
	     emoji::food_and_drink::food_marine::CRAB.annotations.iter()
	     .find(|a| a.lang == "fi").unwrap().tts.unwrap());
    println!("In Turkic, it's called {}",
	     emoji::food_and_drink::food_marine::CRAB.annotations.iter()
	     .find(|a| a.lang == "uz").unwrap().tts.unwrap());
}
