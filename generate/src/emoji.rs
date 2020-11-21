use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use crate::sanitize;
use itertools::Itertools;
#[derive(Debug)]
pub struct Emoji {
    pub codepoint: String,
    pub status: Status,
    pub glyph: String,
    pub introduction_version: f32,
    pub name: String,
    pub variants: Vec<Emoji>,
    pub annotations: Vec<Annotation>,
    pub is_variant: bool,
    pub group: String,
    pub subgroup: String,
}
impl Emoji {
    pub fn new(
        line: &str,
        annotations_map: &HashMap<String, Vec<Annotation>>,
        group: String,
        subgroup: String,
    ) -> Self {
        let first_components: Vec<&str> = line.split(";").collect();
        let reformed_first = first_components.iter().skip(1).join(";");
        let codepoint = first_components[0].trim().to_owned();
        let second_components: Vec<&str> = reformed_first.split("#").collect();
        let status = Status::new(second_components[0].trim());
        let reformed_second = second_components.iter().skip(1).join("#");
        let third_components: Vec<&str> = reformed_second.trim().split("E").collect();
        let glyph = third_components[0].trim().to_owned();
        let reformed_third = third_components.iter().skip(1).join("E");
        let introduction_version = reformed_third
            .split(" ")
            .nth(0)
            .unwrap()
            .parse::<f32>()
            .unwrap();
        let name = reformed_third.split(" ").skip(1).join(" ");

        let annotations = match annotations_map.get(&glyph) {
            None => vec![],
            Some(a) => a.to_vec(),
        };

        Self {
            codepoint,
            status,
            glyph,
            introduction_version,
            name,
            variants: vec![],
            annotations,
            is_variant: false,
            group,
            subgroup,
        }
    }
    pub fn add_variant(&mut self, mut variant: Emoji) {
        variant.is_variant = true;
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
        let variants: Vec<TokenStream> =
            self.variants.iter().map(|e| e.tokens_internal()).collect();
        let annotations = &self.annotations;
        let is_variant = &self.is_variant;
        let group = &self.group;
        let subgroup = &self.subgroup;
        (quote! {
            crate::Emoji{
		glyph: #glyph,
		codepoint: #codepoint,
		status: crate::Status::#status,
		introduction_version: #introduction_version,
		name: #name,
		group: #group,
		subgroup: #subgroup,
		is_variant: #is_variant,
		variants: &[#(#variants),*],
		annotations: &[#(#annotations),*],
            }
        })
            .into()
    }
}
impl ToTokens for Emoji {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = Ident::new(&self.ident(), Span::call_site());
        let tokns = self.tokens_internal();
        let glyph = &self.glyph;
        (quote! {
            #[doc = #glyph]
            pub const #ident: crate::Emoji = #tokns;
        }).to_tokens(tokens);
    }
}
#[derive(Debug, PartialEq)]
pub enum Status {
    Component,
    FullyQualified,
    MinimallyQualified,
    Unqualified,
}
impl Status {
    pub fn new(name: &str) -> Self {
        use Status::*;
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
        write!(
            f,
            "{}",
            match self {
                Component => "Component",
                FullyQualified => "FullyQualified",
                MinimallyQualified => "MinimallyQualified",
                Unqualified => "Unqualified",
            }
        )
    }
}
#[derive(Debug, Clone)]
pub struct Annotation {
    pub lang: String,
    pub tts: Option<String>,
    pub keywords: Vec<String>,
}
impl Annotation {
    pub fn new(lang: String, tts: Option<String>, keywords: String) -> Self {
        let mut s = Self {
            lang,
            tts,
            keywords: vec![],
        };
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
        (quote! {
            crate::Annotation {
		lang: #lang,
		tts: #tts,
		keywords: &[#(#keywords),*],
            }
        }).to_tokens(tokens);
    }
}
