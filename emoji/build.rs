use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let path = "src/emoji_data.rs";
    println!("cargo:rerun-if-changed={}", path);
    let pb: PathBuf = path.into();
    if !pb.exists() {
        File::create(pb).unwrap().write_all("".as_bytes()).unwrap();
    }
}
