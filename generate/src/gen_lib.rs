use crate::group_subgroup::Group;
use std::process::Command;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use proc_macro2::TokenStream;
use quote::quote;
use crate::sanitize;

pub fn dump(groups: &Vec<Group>, annotation_langs: &Vec<&str>, version: f32, date: String) {
    let mut fs = String::new();
    let mut f = File::open("generate/src/library_header.rs").unwrap();
    f.read_to_string(&mut fs).unwrap();
    let ts: TokenStream = fs.parse().unwrap();
    let dump = quote! {
	#ts
	/// All annotation languages (feature independent)
	pub const ANNOTATION_LANGS_TOTAL: &'static [&'static str] = &[
	    #(#annotation_langs),*
	];
	/// Enabled annotation languages (feature dependent)  
	/// Defaults to `["en"]`. Enable the `XX` features for each language to include annotations for another language. For example, to include Finnish annotations, use the `fi` feature.
	pub const ANNOTATION_LANGS_AVAILABLE: &'static [&'static str] = &[
	    #(#[cfg(feature = #annotation_langs)]#annotation_langs),*
	];
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

    for g in groups {
	let dir = format!("emoji/src/{}", sanitize(&g.name).to_lowercase());
	let pb: PathBuf = dir.clone().into();
	if !pb.exists() {
	    std::fs::create_dir(dir).unwrap();
	}
	for s in &g.subgroups {
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
}
