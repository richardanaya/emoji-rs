use std::process::Command;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn dump() {
    let mut fs = String::new();
    let mut f = File::open("generate/src/search_header.rs").unwrap();
    f.read_to_string(&mut fs).unwrap();

    let path = "emoji/src/search.rs";
    let pb: PathBuf = path.clone().into();
    File::create(pb)
        .unwrap()
        .write_all(fs.as_bytes())
        .unwrap();
    Command::new("rustfmt")
        .arg(path)
        .output()
        .expect("Failed to execute command");
}
