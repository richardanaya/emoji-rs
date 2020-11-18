use chrono::DateTime;

struct Group {
    name: String,
    subgroups: Vec<Subgroup>,
}
impl Group {
    pub fn new(name: String) -> Self {
	Self{name, subgroups: vec![]}
    }
}
struct Subgroup {
    pub name: String,
    pub codepoints: Vec<Emoji>,
}
impl Subgroup {
    pub fn new(name: String) -> Self {
	Self{name, codepoints: vec![]}
    }
}
struct Emoji {
    pub codepoint: String,
    pub qualifier: Status,
    pub glyph: String,
    pub introduction_version: f32,
    pub canonical_name: String,
}
enum Status {
    Component,
    FullyQualified,
    MinimallyQualified,
    Unqualified,
}

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "0.2", features = ["macros"] }`
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut date;
    let mut version;
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
		    &line.chars().skip("# Date: ".len()).collect::<String>().replace("GMT", "+0000"), "%F, %T %z")
		    .unwrap();
	    }
	    if line.starts_with("# Version: ") {
		version = line.chars().skip("# Version: ".len()).collect::<String>().parse::<f32>();
	    }
	    if line.starts_with("# group: ") {
		let groupname = line.chars().skip("# group: ".len()).collect::<String>();
		groups.push(Group::new(groupname));
	    }
	    if line.starts_with("# subgroup: ") {
		let subgroupname = line.chars().skip("# subgroup: ".len()).collect::<String>();
		groups.last_mut().unwrap().subgroups.push(Subgroup::new(subgroupname));
	    }
	    continue;
	}
	//panic!("{}", line);
    }

    for group in groups {
	println!("{}", group.name);
	for subg in group.subgroups {
	    println!("  {}", subg.name);
	}
    }
    
    panic!("I'm here to dump stdout");
    
    Ok(())
}
