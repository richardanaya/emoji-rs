use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use itertools::Itertools;

lazy_static::lazy_static! {
    static ref MATCHER: SkimMatcherV2 = SkimMatcherV2::default();
}

/// Fuzzy search by canonical (English) name  
/// Returns a [Emoji](../struct.Emoji.html) vector sorted with most relevant results first
pub fn search_name(searchterm: &str) -> Vec<&'static crate::Emoji> {
    crate::lookup_by_name::iter_emoji()
        .filter_map(|e| {
            MATCHER
                .fuzzy_match(e.name, searchterm)
                .map(|score| (e, score))
        })
        .sorted_by(|(e1, score1), (e2, score2)| {
            Ord::cmp(&score2, &score1).then(Ord::cmp(&e1.name, &e2.name))
        })
        .map(|(e, _)| e)
        .collect::<Vec<_>>()
}

/// Searches by localized name in a given lanauge  
/// Returns a [Emoji](../struct.Emoji.html) vector sorted with most relevant results first
pub fn search_tts(searchterm: &str, lang: &str) -> Vec<&'static crate::Emoji> {
    crate::lookup_by_name::iter_emoji()
        .filter_map(|e| {
	    e.annotations.iter().find(|a| a.lang == lang)
		.and_then(|a| a.tts.and_then(|tts| MATCHER.fuzzy_match(tts, searchterm)))
                .map(|score| (e, score))
        })
        .sorted_by(|(e1, score1), (e2, score2)| {
            Ord::cmp(&score2, &score1).then(Ord::cmp(&e1.name, &e2.name))
        })
        .map(|(e, _)| e).collect::<Vec<_>>()
}

/// Fuzzy search by localized name in available all languages (feature dependent)  
/// Returns a [Emoji](../struct.Emoji.html) vector sorted with most relevant results first
pub fn search_tts_all(searchterm: &str) -> Vec<&'static crate::Emoji> {
    crate::lookup_by_name::iter_emoji()
        .filter_map(|e| {
	    e.annotations.iter()
		.filter_map(|a| a.tts.and_then(|tts| MATCHER.fuzzy_match(tts, searchterm)))
		.fold(None, |acc: Option<i64>, score|
		      Some(acc.map_or(score, |a| a.max(score))))
                .map(|score| (e, score))
        })
        .sorted_by(|(e1, score1), (e2, score2)| {
            Ord::cmp(&score2, &score1).then(Ord::cmp(&e1.name, &e2.name))
        })
        .map(|(e, _)| e).collect::<Vec<_>>()
}

/// Fuzzy search by annotations. This includes localized names as well as localized keywords  
/// Returns a [Emoji](../struct.Emoji.html) vector sorted with most relevant results first
pub fn search_annotation(searchterm: &str, lang: &str) -> Vec<&'static crate::Emoji> {
    crate::lookup_by_name::iter_emoji()
        .filter_map(|e| {
	    e.annotations.iter().find(|a| a.lang == lang)
		.and_then(|a|
			  a.tts.iter().chain(a.keywords.iter())
			  .filter_map(|kwd| MATCHER.fuzzy_match(kwd, searchterm))
			  .fold(None, |acc: Option<i64>, score|
				Some(acc.map_or(score, |a| a.max(score))))
			  .map(|score| (a.tts.map(|tts| tts.len())
					.unwrap_or(e.name.len()), score))
		)
                .map(|(namelen, score)| (e, namelen, score))
        })
        .sorted_by(|(_, namelen1, score1), (_, namelen2, score2)| {
            Ord::cmp(&score2, &score1).then(Ord::cmp(&namelen1, &namelen2))
        })
        .map(|(e, _, _)| e).collect::<Vec<_>>()
}

/// Fuzzy search by annotations in all languages (feature dependent)  
/// Returns a [Emoji](../struct.Emoji.html) vector sorted with most relevant results first
pub fn search_annotation_all(searchterm: &str) -> Vec<&'static crate::Emoji> {
    crate::lookup_by_name::iter_emoji()
        .filter_map(|e| {
	    e.annotations.iter()
		.map(|a| a.tts.iter().chain(a.keywords.iter()))
		.flatten()
		.filter_map(|kwd| MATCHER.fuzzy_match(kwd, searchterm))
		.fold(None, |acc: Option<i64>, score|
		      Some(acc.map_or(score, |a| a.max(score))))
                .map(|score| (e, score))
        })
        .sorted_by(|(e1, score1), (e2, score2)| {
            Ord::cmp(&score2, &score1).then(Ord::cmp(&e1.name, &e2.name))
        })
        .map(|(e, _)| e).collect::<Vec<_>>()
}
