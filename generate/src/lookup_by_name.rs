use crate::group_subgroup::Group;
use std::process::Command;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use proc_macro2::TokenStream;
use quote::quote;
use crate::lookup_types::NameLookupEntry;

pub fn dump(groups: &Vec<Group>) {
    let mut lookup_by_name: Vec<NameLookupEntry> = vec![];
    for g in groups {
	for s in &g.subgroups {
	    for e in &s.emojis {
		lookup_by_name.push(NameLookupEntry::new(&e.group,
							  &e.subgroup,
							  &e.name));
	    }
	}
    }


    let mut fs = String::new();
    let mut f = File::open("generate/src/name_lookup_header.rs").unwrap();
    f.read_to_string(&mut fs).unwrap();
    let ts: TokenStream = fs.parse().unwrap();
    
    let dump = quote! {
	#ts
	static NAME_LOOKUP_MAP: phf::Map<&'static str, crate::Emoji> = phf::phf_map! {
	    #(#lookup_by_name),*
	};
    };

    let path = "emoji/src/lookup_by_name.rs";
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
