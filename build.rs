use chrono::DateTime;
use itertools::Itertools;
use quote::{quote, ToTokens};
use proc_macro2::{TokenStream, Span, Ident};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn sanitize(input: &String) -> String {
    input.replace(" ", "_")
	.replace("&", "and")
	.replace("#", "pound")
	.replace("*", "asterisk")
	.replace("1st", "first")
	.replace("2nd", "second")
	.replace("3rd", "third")
	.replace("(","")
	.replace(")","")
	.replace(":","")
	.replace(".","")
	.replace(".","")
	.replace("'","")
	.replace("’","")
	.replace(",","")
	.replace(",","")
	.replace(",","")
	.replace("-","_")
	.replace("-","_")
	.replace("“","_")
	.replace("”","_")
	.replace("!","")
	.replace("Ñ","N")
	.replace("Å","A")
	.replace("É","E")
	.replace("Ã","A")
	.replace("Í","I")
	.replace("Ç","C")
	.replace("Ô","O")
}

struct Group {
    name: String,
    subgroups: Vec<Subgroup>,
}
impl Group {
    pub fn new(name: String) -> Self {
	Self{name, subgroups: vec![]}
    }
}
impl ToTokens for Group {
    fn to_tokens(&self, tokens: &mut TokenStream) {
	let modname = Ident::new(&sanitize(&self.name).to_lowercase()
				 , Span::call_site());
	let subgroups = &self.subgroups;
	(quote!{
	    mod #modname {
		#(#subgroups)*
	    }
	}).to_tokens(tokens);
    }
}
struct Subgroup {
    pub name: String,
    pub emojis: Vec<Emoji>,
}
impl Subgroup {
    pub fn new(name: String) -> Self {
	Self{name, emojis: vec![]}
    }
}
impl ToTokens for Subgroup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
	let modname = Ident::new(&sanitize(&self.name).to_lowercase()
				 , Span::call_site());
	let emojis = &self.emojis;
	(quote!{
	    mod #modname {
		#(#emojis)*
	    }
	}).to_tokens(tokens);
    }
}
#[derive(Debug)]
struct Emoji {
    pub codepoint: String,
    pub status: Status,
    pub glyph: String,
    pub introduction_version: f32,
    pub name: String,
}
impl Emoji {
    pub fn new(line: &str) -> Self {
	let first_components: Vec<&str> = line.split(";").collect();
	let reformed_first = first_components.iter().skip(1).join(";");
	let codepoint = first_components[0].trim().to_owned();
	let second_components: Vec<&str> = reformed_first.split("#").collect();
	let status = Status::new(second_components[0].trim());
	let reformed_second = second_components.iter().skip(1).join("#");
	let third_components: Vec<&str> = reformed_second.trim().split("E").collect();
	let glyph = third_components[0].trim().to_owned();
	let reformed_third = third_components
	    .iter().skip(1).join("E");
	let introduction_version = reformed_third.split(" ").nth(0)
	    .unwrap().parse::<f32>().unwrap();
	let name = reformed_third.split(" ").skip(1).join(" ");
	Self{codepoint, status, glyph, introduction_version, name}
    }
}
impl ToTokens for Emoji {
    fn to_tokens(&self, tokens: &mut TokenStream) {
	use Status::*;
	let glyph = match self.status {
	    Component | FullyQualified => self.glyph.clone(),
	    Unqualified => format!("{}_Unqualified", self.glyph),
	    MinimallyQualified => format!("{}_MinimallyQualified", self.glyph),
	};
	if sanitize(&self.name).to_uppercase().len() == 0 {
	    panic!("{:?}", self);
	}
	let ident = Ident::new(&sanitize(&self.name).to_uppercase()
			       , Span::call_site());
	(quote!{
	    pub const #ident: &'static str = #glyph;
	}).to_tokens(tokens);
    }
}
#[derive(Debug)]
enum Status {
    Component,
    FullyQualified,
    MinimallyQualified,
    Unqualified,
}
impl Status {
    pub fn new(name: &str) -> Self {
	use crate::Status::*;
	match name {
	    "component" => Component,
	    "fully-qualified" => FullyQualified,
	    "minimally-qualified" => MinimallyQualified,
	    "unqualified" => Unqualified,
	    unknown => panic!("Unknown qualifier {}", unknown),
	}
    }
}

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "0.2", features = ["macros"] }`
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut date = "".to_owned();
    let mut version = 0.0;
    let mut groups: Vec<Group> = vec![];
    
    
    //let res = reqwest::get("https://unicode.org/Public/cldr/38/cldr-tools-38.0.zip").await?;
    let res = reqwest::get("https://raw.githubusercontent.com/unicode-org/cldr/release-38/tools/java/org/unicode/cldr/util/data/emoji/emoji-test.txt").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    for line in body.split("\n") {
	if line.len() == 0 {
	    continue;
	}
	if line.chars().nth(0) == Some('#') {
	    // These lines are either comments or data related to one of:
	    // - group
	    // - subgroup
	    // - Date published
	    // - Publication Version
	    if line.starts_with("# Date: ") {
		date = DateTime::parse_from_str(
		    &line.chars().skip("# Date: ".len()).collect::<String>()
			.replace("GMT", "+0000"), "%F, %T %z")
		    .unwrap().to_rfc3339();
	    }
	    if line.starts_with("# Version: ") {
		version = line.chars().skip("# Version: ".len())
		    .collect::<String>().parse::<f32>().unwrap();
	    }
	    if line.starts_with("# group: ") {
		let groupname = line.chars().skip("# group: ".len())
		    .collect::<String>();
		groups.push(Group::new(groupname));
	    }
	    if line.starts_with("# subgroup: ") {
		let subgroupname = line.chars().skip("# subgroup: ".len())
		    .collect::<String>();
		groups.last_mut().unwrap().subgroups.push(Subgroup::new(subgroupname));
	    }
	    continue;
	}
	groups.last_mut().unwrap().subgroups.last_mut().unwrap().emojis
	    .push(Emoji::new(line));
    }

    if version == 0.0 {
	panic!("No unicode version found while parsing emoji data");
    }
    if date == "" {
	panic!("No release date found while parsing emoji data");
    }

    let datestr = &date; // quote is picky
    let dump = quote!{
	pub const UNICODE_VERSION: f32 = #version;
	pub const UNICODE_RELEASE_TIME: &'static str = #datestr; // rfc3339 formatted chrono::DateTime
	#(#groups)*
    };

    let pb: PathBuf = format!("{}/emoji_data.rs", std::env::var("OUT_DIR").unwrap()).into();
    File::create(pb).unwrap().write_all(format!("{}", dump).as_bytes());
    
    Ok(())
}
