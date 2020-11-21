use chrono::DateTime;
use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use xml::reader::{EventReader, XmlEvent};
use futures::future::join_all;
use std::collections::HashMap;

mod sanitize;
use sanitize::*;
mod group_subgroup;
use group_subgroup::*;
mod emoji;
use emoji::*;
mod lookup_types;
use lookup_types::*;

async fn create_requests(client: &reqwest::Client, annotation_langs: &Vec<&str>) -> reqwest::Result<Vec<String>> {
    let annotation_requests: Vec<reqwest::Response> = join_all(
	annotation_langs.iter().map(|lang| {
	    client.get(&format!("https://raw.githubusercontent.com/unicode-org/cldr/release-38/common/annotations/{}.xml", lang)).send()
	})).await.into_iter().collect::<Result<_, _>>()?;
    join_all(annotation_requests.into_iter().map(|r| {
	r.text()
    })).await.into_iter().collect::<Result<_, _>>()
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Make sure this is ran from workspace root");
    //let annotation_langs = vec!["af", "am", "ar", "ar_SA", "as", "ast", "az", "be", "bg", "bn", "br", "bs", "ca", "ccp", "ceb", "chr", "ckb", "cs", "cy", "da", "de", "de_CH", "doi", "el", "en", "en_001", "en_AU", "en_CA", "en_GB", "en_IN", "es", "es_419", "es_MX", "es_US", "et", "eu", "fa", "fi", "fil", "fo", "fr", "fr_CA", "ga", "gd", "gl", "gu", "ha", "ha_NE", "he", "hi", "hr", "hu", "hy", "ia", "id", "ig", "is", "it", "ja", "jv", "ka", "kab", "kk", "kl", "km", "kn", "ko", "kok", "ku", "ky", "lb", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mni", "mr", "ms", "mt", "my", "nb", "ne", "nl", "nn", "or", "pa", "pa_Arab", "pcm", "pl", "ps", "pt", "pt_PT", "qu", "rm", "ro", "root", "ru", "rw", "sa", "sat", "sd", "si", "sk", "sl", "so", "sq", "sr", "sr_Cyrl", "sr_Cyrl_BA", "sr_Latn", "sr_Latn_BA", "su", "sv", "sw", "sw_KE", "ta", "te", "tg", "th", "ti", "tk", "to", "tr", "tt", "ug", "uk", "ur", "uz", "vi", "wo", "xh", "yo", "yo_BJ", "yue", "yue_Hans", "zh", "zh_Hant", "zh_Hant_HK", "zu", ];
    let annotation_langs = vec!["en"];
    let client = reqwest::Client::new();
    let annotation_text = create_requests(&client, &annotation_langs).await?;
    
    let langs_len = annotation_langs.len();
    let mut date = "".to_owned();
    let mut version = 0.0;
    let mut groups: Vec<Group> = vec![];

    let mut annotations: HashMap<String, Vec<Annotation>> = HashMap::new();

    for (i, lang) in annotation_langs.clone().into_iter().enumerate() {
        println!(
            "Processing annotations for language {}. Progress {}/{}",
            lang, i, langs_len
        );

        let parser = EventReader::from_str(&annotation_text[i]);
        let mut current_glyph = None;
        let mut is_tts = false;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { attributes, .. }) => {
                    if let Some(cp) = attributes.iter().find(|attr| attr.name.local_name == "cp") {
                        current_glyph = Some(cp.value.clone());
                    }
                    if let Some(tts) = attributes
                        .into_iter()
                        .find(|attr| attr.name.local_name == "type")
                    {
                        is_tts = tts.value == "tts";
                    } else {
                        is_tts = false;
                    }
                }
                Ok(XmlEvent::Characters(data)) => {
                    if let Some(glyph) = current_glyph.clone() {
                        match annotations.get_mut(&glyph) {
                            Some(prev_entry) => {
                                if let Some(same_lang) =
                                    prev_entry.iter_mut().find(|s| &s.lang == lang)
                                {
                                    if is_tts {
                                        same_lang.tts = Some(data.trim().to_string());
                                    } else {
                                        same_lang.add_keywords(data);
                                    }
                                } else {
                                    prev_entry.push(if is_tts {
                                        Annotation::new(
                                            lang.to_string(),
                                            Some(data.trim().to_string()),
                                            "".to_owned(),
                                        )
                                    } else {
                                        Annotation::new(lang.to_string(), None, data)
                                    });
                                }
                            }
                            None => {
                                if is_tts {
                                    annotations.insert(
                                        glyph,
                                        vec![Annotation::new(
                                            lang.to_string(),
                                            Some(data.trim().to_string()),
                                            "".to_owned(),
                                        )],
                                    );
                                } else {
                                    annotations.insert(
                                        glyph,
                                        vec![Annotation::new(lang.to_string(), None, data)],
                                    );
                                }
                            }
                        };
                    }
                }
                Err(e) => {
                    panic!("xml parse error for language {}: {}", lang, e);
                }
                _ => {}
            }
        }
    }

    println!("Annotation processing done. Compiling into emoji list");

    let emoji_test = client.get("https://raw.githubusercontent.com/unicode-org/cldr/release-38/tools/java/org/unicode/cldr/util/data/emoji/emoji-test.txt").send().await?;

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
                    &line
                        .chars()
                        .skip("# Date: ".len())
                        .collect::<String>()
                        .replace("GMT", "+0000"),
                    "%F, %T %z",
                )
                    .unwrap()
                    .to_rfc3339();
            }
            if line.starts_with("# Version: ") {
                version = line
                    .chars()
                    .skip("# Version: ".len())
                    .collect::<String>()
                    .parse::<f32>()
                    .unwrap();
            }
            if line.starts_with("# group: ") {
                let groupname = line.chars().skip("# group: ".len()).collect::<String>();
                groups.push(Group::new(groupname));
            }
            if line.starts_with("# subgroup: ") {
                let subgroupname = line.chars().skip("# subgroup: ".len()).collect::<String>();
                groups
                    .last_mut()
                    .unwrap()
                    .subgroups
                    .push(Subgroup::new(subgroupname));
            }
            continue;
        }
        let groupname = groups.last().unwrap().name.clone();
        let subgroupname = groups
            .last()
            .unwrap()
            .subgroups
            .last()
            .unwrap()
            .name
            .clone();
        let emoji_list = &mut groups
            .last_mut()
            .unwrap()
            .subgroups
            .last_mut()
            .unwrap()
            .emojis;
        let new_emoji = Emoji::new(line, &annotations, groupname, subgroupname);
        match &mut emoji_list.last_mut() {
            Some(old_emoji) if old_emoji.ident() == new_emoji.ident() => {
                old_emoji.add_variant(new_emoji);
            }
            _ => {
                emoji_list.push(new_emoji);
            }
        }
    }

    if version == 0.0 {
        panic!("No unicode version found while parsing emoji data");
    }
    if date == "" {
        panic!("No release date found while parsing emoji data");
    }

    println!("Generating lookup tables.");
    let mut lookup_by_glyph: Vec<GlyphLookupEntry> = vec![];
    for g in &groups {
	for s in &g.subgroups {
	    for e in &s.emojis {
		lookup_by_glyph.push(GlyphLookupEntry::new(&e.glyph,
							   &e.group,
							   &e.subgroup,
							   &e.name));
		for v in &e.variants {
		    lookup_by_glyph.push(GlyphLookupEntry::new(&v.glyph,
							       &v.group,
							       &v.subgroup,
							       &v.name));
		}
	    }
	}
    }

    println!("Turning into rust and dumping to file.");

    {
	let mut fs = String::new();
	let mut f = File::open("generate/src/glyph_lookup_header.rs").unwrap();
	f.read_to_string(&mut fs).unwrap();
	let ts: TokenStream = fs.parse().unwrap();
	
	let dump = quote! {
	    #ts
	    static GLYPH_LOOKUP_MAP: phf::Map<&'static str, crate::Emoji> = phf::phf_map! {
		#(#lookup_by_glyph),*
	    };
	};

    let path = "emoji/src/lookup_by_glyph.rs";
    let pb: PathBuf = path.clone().into();
    File::create(pb)
        .unwrap()
        .write_all(format!("{}", dump).as_bytes())
        .unwrap();
    Command::new("rustfmt")
        .arg(path)
        .output()
            .expect("Failed to execute command");
    }
    {
	let mut fs = String::new();
	let mut f = File::open("generate/src/library_header.rs").unwrap();
	f.read_to_string(&mut fs).unwrap();
	let ts: TokenStream = fs.parse().unwrap();
	
	let dump = quote! {
	    #ts
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
	let path = "emoji/src/lib.rs";
	let pb: PathBuf = path.clone().into();
	File::create(pb)
            .unwrap()
            .write_all(format!("{}", dump).as_bytes())
            .unwrap();
	Command::new("rustfmt")
            .arg(path)
            .output()
            .expect("Failed to execute command");

    }

    for g in groups {
        let dir = format!("emoji/src/{}", sanitize(&g.name).to_lowercase());
        let pb: PathBuf = dir.clone().into();
        if !pb.exists() {
            std::fs::create_dir(dir).unwrap();
        }
        for s in g.subgroups {
            let emojis = &s.emojis;
            let path = format!(
                "emoji/src/{}/{}.rs",
                sanitize(&g.name).to_lowercase(),
                sanitize(&s.name).to_lowercase()
            );
            let pb: PathBuf = path.clone().into();
            let dump = quote! {
		#(#emojis)*
            };
            File::create(pb)
                .unwrap()
                .write_all(format!("{}", dump).as_bytes())
                .unwrap();
            Command::new("rustfmt")
                .arg(path)
                .output()
                .expect("Failed to execute command");
        }
    }

    Ok(())
}
