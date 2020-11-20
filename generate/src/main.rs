use chrono::DateTime;
use itertools::Itertools;
use quote::{quote, ToTokens};
use proc_macro2::{TokenStream, Span, Ident};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use unidecode::unidecode;
use std::process::Command;
use xml::reader::{EventReader, XmlEvent};
use std::collections::HashMap;


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
	(quote!{
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
    variants: Vec<Emoji>,
    pub annotations: Vec<Annotation>,
}
impl Emoji {
    pub fn new(line: &str, annotations_map: &HashMap<String, Vec<Annotation>>) -> Self {
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

	let annotations = match annotations_map.get(&glyph) {
	    None => vec![],
	    Some(a) => a.to_vec(),
	};
	
	Self{codepoint, status, glyph, introduction_version, name, variants: vec![], annotations}
    }
    pub fn add_variant(&mut self, mut variant: Emoji) {
	for a in &mut self.annotations {
	    if let Some(a_other) = variant.annotations.iter_mut().find(|i| i.lang == a.lang) {
		if a_other.tts.is_some() {
		    if a.tts.is_none() {
			a.tts = a_other.tts.clone();
		    }
		    a_other.tts = None;
		}
	    }
	}
	self.annotations.append(&mut variant.annotations);
	self.variants.push(variant);
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
	let annotations = &self.annotations;
	(quote!{
	    crate::Emoji{
		glyph: #glyph,
		codepoint: #codepoint,
		status: crate::Status::#status,
		introduction_version: #introduction_version,
		name: #name,
		annotations: &[#(#annotations),*],
		variants: &[#(#variants),*],
	    }
	}).into()
    }
}
impl ToTokens for Emoji {
    fn to_tokens(&self, tokens: &mut TokenStream) {
	let ident = Ident::new(&self.ident(), Span::call_site());
	let tokns = self.tokens_internal();
	let glyph = &self.glyph;
	(quote!{
	    #[doc = #glyph]
	    pub const #ident: crate::Emoji = #tokns;
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
#[derive(Debug, Clone)]
struct Annotation {
    lang: String,
    pub tts: Option<String>, // TODO: cross reference with emoji.name
    keywords: Vec<String>,
}
impl Annotation {
    pub fn new(lang: String, tts: Option<String>, keywords: String) -> Self {
	let mut s = Self{lang, tts, keywords: vec![]};
	s.add_keywords(keywords);
	s
    }
    pub fn add_keywords(&mut self, keywords: String) {
	let mut v = keywords.split("|").map(|a| a.trim().to_owned()).collect();
	self.keywords.append(&mut v);
	self.keywords.sort();
	self.keywords.dedup();
    }
}
impl ToTokens for Annotation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
	let lang = &self.lang;
	let tts = match &self.tts {
	    None => quote! {
		None
	    },
	    Some(tts) => quote! {
		Some(#tts)
	    },
	};
	let keywords = &self.keywords;
	(quote!{
	    crate::Annotation {
		lang: #lang,
		tts: #tts,
		keywords: &[#(#keywords),*],
	    }
	}).to_tokens(tokens);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let annotation_langs = vec!["en", "fi"]; // will change in future
    let mut date = "".to_owned();
    let mut version = 0.0;
    let mut groups: Vec<Group> = vec![];
    
    let mut annotations: HashMap<String, Vec<Annotation>> = HashMap::new();

    for lang in &annotation_langs {
	let annotation_res = reqwest::get(&format!("https://raw.githubusercontent.com/unicode-org/cldr/release-38/common/annotations/{}.xml", lang)).await?;
	let annotation_text = annotation_res.text().await?;

	let parser = EventReader::from_str(&annotation_text);
	let mut current_glyph = None;
	let mut is_tts = false;
	for e in parser {
            match e {
		Ok(XmlEvent::StartElement { attributes, .. }) => {
		    if let Some(cp) = attributes.iter()
			.find(|attr| attr.name.local_name == "cp") {
			    current_glyph = Some(cp.value.clone());
			}
		    if let Some(tts) = attributes.into_iter()
			.find(|attr| attr.name.local_name == "type") {
			    is_tts = tts.value == "tts";
			} else {
			    is_tts = false;
			}
		}
		Ok(XmlEvent::Characters(data)) => {
		    if let Some(glyph) = current_glyph.clone() {
			match annotations.get_mut(&glyph) {
			    Some(prev_entry) => {
				if let Some(same_lang) = prev_entry.iter_mut()
				    .find(|s| &s.lang == lang) {
					if is_tts {
					    same_lang.tts = Some(data.trim().to_string());
					} else {
					    same_lang.add_keywords(data);
					}
				    } else {
					prev_entry.push(if is_tts {
					    Annotation::new(
						lang.to_string(),
						Some(data.trim().to_string()), "".to_owned(),
					    )
					} else {
					    Annotation::new(
						lang.to_string(),
						None, data,
					    )
					});
				    }
			    },
			    None => {
				if is_tts {
				    annotations.insert(glyph, vec![Annotation::new(
					lang.to_string(),
					Some(data.trim().to_string()), "".to_owned(),
				    )]);
				} else {
				    annotations.insert(glyph, vec![Annotation::new(
					lang.to_string(),
					None, data,
				    )]);
				}
			    },
			};
		    }
		}
		Err(e) => {
                    println!("Error: {}", e);
                    break;
		}
		_ => {}
            }
	}
    }
    
    let emoji_test = reqwest::get("https://raw.githubusercontent.com/unicode-org/cldr/release-38/tools/java/org/unicode/cldr/util/data/emoji/emoji-test.txt").await?;

    let emoji_test_text = emoji_test.text().await?;

    for line in emoji_test_text.split("\n") {
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
	let new_emoji = Emoji::new(line, &annotations);
	match &mut emoji_list.last_mut() {
	    Some(old_emoji) if old_emoji.ident() == new_emoji.ident() => {
		old_emoji.add_variant(new_emoji);
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

    let dump = quote!{
	/// The annotation languages this crate was compiled with
	/// Defaults to `["en"]`. Enable the `lang_XX` features for each language to include annotations for another language. For example, to include Finnish annotations, use the `lang_fi` feature.
	pub const ANNOTATION_LANGS: &'static [&'static str] = &[#(#annotation_langs),*];
	/// The unicode release version that this crate is compiled against
	pub const UNICODE_VERSION: f32 = #version;
	/// The rfc3339 formatted time of the unicode release that this crate is compiled against
	pub const UNICODE_RELEASE_TIME: &'static str = #date;
	#(#groups)*
    };

    // skeleton; writes the module structure
    let path = "emoji/src/emoji_data.rs";
    let pb: PathBuf = path.clone().into();
    File::create(pb).unwrap().write_all(format!("{}", dump).as_bytes()).unwrap();
    Command::new("rustfmt")
        .arg(path)
        .output()
        .expect("Failed to execute command");


    for g in groups {
	let dir = format!("emoji/src/{}",
			  sanitize(&g.name).to_lowercase());
	let pb: PathBuf = dir.clone().into();
	if !pb.exists() {
	    std::fs::create_dir(dir).unwrap();
	}
	for s in g.subgroups {
	    let emojis = &s.emojis;
	    let path = format!("emoji/src/{}/{}.rs",
			       sanitize(&g.name).to_lowercase(),
			       sanitize(&s.name).to_lowercase());
	    let pb: PathBuf = path.clone().into();
	    let dump = quote!{
		#(#emojis)*
	    };
	    File::create(pb).unwrap().write_all(format!("{}", dump).as_bytes()).unwrap();
	    Command::new("rustfmt")
		.arg(path)
		.output()
		.expect("Failed to execute command");
	}
    }
    
    Ok(()) // TODO: lookup_by_glyph() and variants
}
