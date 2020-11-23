use std::collections::HashMap;
use chrono::DateTime;
use crate::group_subgroup::*;
use crate::Annotation;

pub async fn vectorize_test_data(client: &reqwest::Client, annotations: &HashMap<String, Vec<Annotation>>) -> reqwest::Result<(String, f32, Vec<Group>)> {
    let mut date = "".to_owned();
    let mut version = 0.0;
    let mut groups: Vec<Group> = vec![];
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
        let new_emoji = crate::Emoji::new(line, annotations, groupname, subgroupname);
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
    Ok((date, version, groups))
}
