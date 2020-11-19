use chrono::DateTime;
use itertools::Itertools;
use quote::{quote, ToTokens};
use proc_macro2::{TokenStream, Span, Ident};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use unidecode::unidecode;

fn sanitize(input: &String) -> String {
    unidecode(&input
	.replace(" ", "_")
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
	.replace("!",""))
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
	    pub mod #modname {
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
	let path = format!("emoji_subgroup_{}.rs",
		sanitize(&self.name).to_lowercase());
	(quote!{
	    #[path=#path]
	    pub mod #modname;
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
    pub variants: Vec<Emoji>,
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
	Self{codepoint, status, glyph, introduction_version, name, variants: vec![]}
    }
    pub fn ident(&self) -> String {
	sanitize(&self.name).to_uppercase()
    }
    fn tokens_internal(&self) -> TokenStream {
	let glyph = &self.glyph;
	let codepoint = &self.codepoint;
	let name = &self.name;
	let status = Ident::new(&self.status.to_string(), Span::call_site());
	let introduction_version = self.introduction_version;
	let variants: Vec<TokenStream> = self.variants.iter()
	    .map(|e| e.tokens_internal()).collect();
	(quote!{
		crate::Emoji{glyph: #glyph,
			     codepoint: #codepoint,
			     status: crate::Status::#status,
			     introduction_version: #introduction_version,
			     name: #name,
			     variants: &[#(#variants),*]}
	}).into()
    }
}
impl ToTokens for Emoji {
    fn to_tokens(&self, tokens: &mut TokenStream) {
	let ident = Ident::new(&self.ident(), Span::call_site());
	let tokns = self.tokens_internal();
	(quote!{
	    pub const #ident: crate::Emoji = #tokns ;
	}).to_tokens(tokens);
    }
}
#[derive(Debug, PartialEq)]
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
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	use Status::*;
	write!(f, "{}", match self {
	    Component => "Component",
	    FullyQualified => "FullyQualified",
	    MinimallyQualified => "MinimallyQualified",
	    Unqualified => "Unqualified",
	})
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
	let emoji_list = &mut groups.last_mut().unwrap().subgroups.last_mut()
	    .unwrap().emojis;
	let new_emoji = Emoji::new(line);
	match &mut emoji_list.last_mut() {
	    Some(old_emoji) if old_emoji.ident() == new_emoji.ident() => {
		old_emoji.variants.push(new_emoji);
	    },
	    _ => {
		emoji_list.push(new_emoji);
	    },
	}
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

    // skeleton; writes the module structure
    let pb: PathBuf = "emoji/src/emoji_data.rs".into();
    File::create(pb).unwrap().write_all(format!("{}", dump).as_bytes()).unwrap();

    for g in groups {
	let dir = format!("emoji/src/{}",
			  sanitize(&g.name).to_lowercase());
	let pb: PathBuf = dir.clone().into();
	if !pb.exists() {
	    std::fs::create_dir(dir).unwrap();
	}
	for s in g.subgroups {
	    let emojis = &s.emojis;
	    let pb: PathBuf = format!("emoji/src/{}/emoji_subgroup_{}.rs",
				      sanitize(&g.name).to_lowercase(),
				      sanitize(&s.name).to_lowercase()).into();
	    let dump = quote!{
		#(#emojis)*
	    };
	    File::create(pb).unwrap().write_all(format!("{}", dump).as_bytes()).unwrap();
	}
    }
    
    Ok(())
}
