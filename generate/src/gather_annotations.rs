use std::collections::HashMap;
use xml::reader::{EventReader, XmlEvent};
use crate::Annotation;

pub fn gather_annotations(annotation_text: &Vec<String>, annotation_langs: &Vec<&str>) -> HashMap<String, Vec<Annotation>> {
    let langs_len = annotation_langs.len();
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
    annotations
}
