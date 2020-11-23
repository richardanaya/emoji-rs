#![doc = " ## Introduction  "]
#![doc = " 3511 emojis and 4580 emoji variants with localization data in 143 languages  "]
#![doc = " This crate contains a huge amount of data about every emoji ever."]
#![doc = " Some of the data includes:"]
#![doc = " - Name"]
#![doc = " - Glyph"]
#![doc = " - Unicode Release Version"]
#![doc = " - Classification"]
#![doc = " - Variants"]
#![doc = " - Annotations in many languages  "]
#![doc = " This crate also provides functions for searching through emojis by"]
#![doc = " [name](lookup_by_name/index.html) and [glyph](lookup_by_glyph/index.html),"]
#![doc = " as well as several [fuzzy search](search/index.html) functions."]
#![doc = " ## Quickstart  "]
#![doc = " ```rust"]
#![doc = " fn main() {"]
#![doc = "    println!(\"{}\", emoji::food_and_drink::food_marine::CRAB.glyph);"]
#![doc = " }"]
#![doc = " ```"]
#![doc = " See more examples [here](https://github.com/Shizcow/emoji-rs/tree/master/examples/)."]
#![doc = " ## Languages  "]
#![doc = " By default, only English annotations are compiled in."]
#![doc = " To enable other languages, use the feature corresponding to that languge. An exhaustive"]
#![doc = " list of supported languages can be found"]
#![doc = " [here](https://github.com/Shizcow/emoji-rs/blob/master/emoji/Cargo.toml)."]
#[doc = " Emoji status qualifier  "]
#[doc = " In nearly every case, MinimallyQualified or Unqualified will show up in emoji variants."]
#[doc = " A complete tool needs only to support all of the FullyQualified emojis."]
#[derive(Debug, PartialEq)]
pub enum Status {
    #[doc = " A qualified emoji character, or an emoji sequence in which each emoji character is qualified. Most emojis fall into this category."]
    FullyQualified,
    #[doc = " An emoji sequence in which the first character is qualified but the sequence is not fully qualified."]
    MinimallyQualified,
    #[doc = " An emoji that is neither fully-qualified nor minimally qualified."]
    Unqualified,
    #[doc = " Used for modifiers, such as skin tone modifiers."]
    Component,
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
#[doc = " Contains all information about an emoji  "]
#[doc = " See the [CLDR](https://raw.githubusercontent.com/unicode-org/cldr/release-38/tools/java/org/unicode/cldr/util/data/emoji/emoji-test.txt) for specific examples of all fields except `variants`."]
#[derive(Debug, PartialEq)]
pub struct Emoji {
    #[doc = " The ASCII-formatted string representation of this emoji's UTF8 codepoint value  "]
    #[doc = " Ex: `1F441 200D 1F5E8 FE0F`"]
    pub codepoint: &'static str,
    #[doc = " Qualification status"]
    pub status: Status,
    #[doc = " The actual emoji text  "]
    #[doc = " Ex: 😺"]
    pub glyph: &'static str,
    #[doc = " The Unicode release version which this emoji was introduced in"]
    pub introduction_version: f32,
    #[doc = " English [CLDR Short Name](https://unicode.org/emoji/format.html#col-name)"]
    #[doc = " (canonical) name of this emoji  "]
    #[doc = " Ex: `grinning cat`"]
    pub name: &'static str,
    #[doc = " General classification this emoji belongs to  "]
    #[doc = " Ex: `Smileys & Emotion`"]
    pub group: &'static str,
    #[doc = " Specific classification this emoji belongs to  "]
    #[doc = " Ex: `cat-face`"]
    pub subgroup: &'static str,
    #[doc = " All variants of an emoji. If two emojis share the same name, one is a variant."]
    #[doc = " Variants are always less qualified than their parent. Parents can be found from a"]
    #[doc = " variant via [emoji::lookup_by_glyph::lookup](lookup_by_glyph/fn.lookup.html)"]
    pub variants: &'static [Emoji],
    #[doc = " Is this emoji a variant?"]
    pub is_variant: bool,
    #[doc = " Localizatoin specific annotations"]
    pub annotations: &'static [Annotation],
}
#[doc = " Annotation meta-data for each emoji"]
#[derive(Debug, PartialEq)]
pub struct Annotation {
    #[doc = " Language code of the associated data. Guarenteed to be found in"]
    #[doc = " [ANNOTATION_LANGS_AVAILABLE](constant.ANNOTATION_LANGS_AVAILABLE.html)"]
    pub lang: &'static str,
    #[doc = " Localized name for an emoji  "]
    #[doc = " Ex: `fried shrimp`"]
    pub tts: Option<&'static str>,
    #[doc = " Keywords associated with an emoji, in the localized language  "]
    #[doc = " Ex: `[\"fried shrimp\", \"shrimp\", \"prawn\"]`"]
    pub keywords: &'static [&'static str],
}
#[doc = " Defines functions for searching through and iterating over emojis by glyph  "]
#[doc = " Includes variants"]
pub mod lookup_by_glyph;
#[doc = " Defines functions for searching through and iterating over emojis by name  "]
#[doc = " Yields exact matches only, but is extremely fast  "]
#[doc = " Does not include variants"]
pub mod lookup_by_name;
#[doc = " Fuzzy search algorithms for general purpose searching"]
pub mod search;
#[doc = r" All annotation languages (feature independent)"]
pub const ANNOTATION_LANGS_TOTAL: &'static [&'static str] = &[
    "af",
    "am",
    "ar",
    "ar_SA",
    "as",
    "ast",
    "az",
    "be",
    "bg",
    "bn",
    "br",
    "bs",
    "ca",
    "ccp",
    "ceb",
    "chr",
    "ckb",
    "cs",
    "cy",
    "da",
    "de",
    "de_CH",
    "doi",
    "el",
    "en",
    "en_001",
    "en_AU",
    "en_CA",
    "en_GB",
    "en_IN",
    "es",
    "es_419",
    "es_MX",
    "es_US",
    "et",
    "eu",
    "fa",
    "fi",
    "fil",
    "fo",
    "fr",
    "fr_CA",
    "ga",
    "gd",
    "gl",
    "gu",
    "ha",
    "ha_NE",
    "he",
    "hi",
    "hr",
    "hu",
    "hy",
    "ia",
    "id",
    "ig",
    "is",
    "it",
    "ja",
    "jv",
    "ka",
    "kab",
    "kk",
    "kl",
    "km",
    "kn",
    "ko",
    "kok",
    "ku",
    "ky",
    "lb",
    "lo",
    "lt",
    "lv",
    "mai",
    "mi",
    "mk",
    "ml",
    "mn",
    "mni",
    "mr",
    "ms",
    "mt",
    "my",
    "nb",
    "ne",
    "nl",
    "nn",
    "or",
    "pa",
    "pa_Arab",
    "pcm",
    "pl",
    "ps",
    "pt",
    "pt_PT",
    "qu",
    "rm",
    "ro",
    "root",
    "ru",
    "rw",
    "sa",
    "sat",
    "sd",
    "si",
    "sk",
    "sl",
    "so",
    "sq",
    "sr",
    "sr_Cyrl",
    "sr_Cyrl_BA",
    "sr_Latn",
    "sr_Latn_BA",
    "su",
    "sv",
    "sw",
    "sw_KE",
    "ta",
    "te",
    "tg",
    "th",
    "ti",
    "tk",
    "to",
    "tr",
    "tt",
    "ug",
    "uk",
    "ur",
    "uz",
    "vi",
    "wo",
    "xh",
    "yo",
    "yo_BJ",
    "yue",
    "yue_Hans",
    "zh",
    "zh_Hant",
    "zh_Hant_HK",
    "zu",
];
#[doc = r" Enabled annotation languages (feature dependent)  "]
#[doc = r#" Defaults to `["en"]`. Enable the `XX` features for each language to include annotations for another language. For example, to include Finnish annotations, use the `fi` feature."#]
pub const ANNOTATION_LANGS_AVAILABLE: &'static [&'static str] = &[
    #[cfg(feature = "af")]
    "af",
    #[cfg(feature = "am")]
    "am",
    #[cfg(feature = "ar")]
    "ar",
    #[cfg(feature = "ar_SA")]
    "ar_SA",
    #[cfg(feature = "as")]
    "as",
    #[cfg(feature = "ast")]
    "ast",
    #[cfg(feature = "az")]
    "az",
    #[cfg(feature = "be")]
    "be",
    #[cfg(feature = "bg")]
    "bg",
    #[cfg(feature = "bn")]
    "bn",
    #[cfg(feature = "br")]
    "br",
    #[cfg(feature = "bs")]
    "bs",
    #[cfg(feature = "ca")]
    "ca",
    #[cfg(feature = "ccp")]
    "ccp",
    #[cfg(feature = "ceb")]
    "ceb",
    #[cfg(feature = "chr")]
    "chr",
    #[cfg(feature = "ckb")]
    "ckb",
    #[cfg(feature = "cs")]
    "cs",
    #[cfg(feature = "cy")]
    "cy",
    #[cfg(feature = "da")]
    "da",
    #[cfg(feature = "de")]
    "de",
    #[cfg(feature = "de_CH")]
    "de_CH",
    #[cfg(feature = "doi")]
    "doi",
    #[cfg(feature = "el")]
    "el",
    #[cfg(feature = "en")]
    "en",
    #[cfg(feature = "en_001")]
    "en_001",
    #[cfg(feature = "en_AU")]
    "en_AU",
    #[cfg(feature = "en_CA")]
    "en_CA",
    #[cfg(feature = "en_GB")]
    "en_GB",
    #[cfg(feature = "en_IN")]
    "en_IN",
    #[cfg(feature = "es")]
    "es",
    #[cfg(feature = "es_419")]
    "es_419",
    #[cfg(feature = "es_MX")]
    "es_MX",
    #[cfg(feature = "es_US")]
    "es_US",
    #[cfg(feature = "et")]
    "et",
    #[cfg(feature = "eu")]
    "eu",
    #[cfg(feature = "fa")]
    "fa",
    #[cfg(feature = "fi")]
    "fi",
    #[cfg(feature = "fil")]
    "fil",
    #[cfg(feature = "fo")]
    "fo",
    #[cfg(feature = "fr")]
    "fr",
    #[cfg(feature = "fr_CA")]
    "fr_CA",
    #[cfg(feature = "ga")]
    "ga",
    #[cfg(feature = "gd")]
    "gd",
    #[cfg(feature = "gl")]
    "gl",
    #[cfg(feature = "gu")]
    "gu",
    #[cfg(feature = "ha")]
    "ha",
    #[cfg(feature = "ha_NE")]
    "ha_NE",
    #[cfg(feature = "he")]
    "he",
    #[cfg(feature = "hi")]
    "hi",
    #[cfg(feature = "hr")]
    "hr",
    #[cfg(feature = "hu")]
    "hu",
    #[cfg(feature = "hy")]
    "hy",
    #[cfg(feature = "ia")]
    "ia",
    #[cfg(feature = "id")]
    "id",
    #[cfg(feature = "ig")]
    "ig",
    #[cfg(feature = "is")]
    "is",
    #[cfg(feature = "it")]
    "it",
    #[cfg(feature = "ja")]
    "ja",
    #[cfg(feature = "jv")]
    "jv",
    #[cfg(feature = "ka")]
    "ka",
    #[cfg(feature = "kab")]
    "kab",
    #[cfg(feature = "kk")]
    "kk",
    #[cfg(feature = "kl")]
    "kl",
    #[cfg(feature = "km")]
    "km",
    #[cfg(feature = "kn")]
    "kn",
    #[cfg(feature = "ko")]
    "ko",
    #[cfg(feature = "kok")]
    "kok",
    #[cfg(feature = "ku")]
    "ku",
    #[cfg(feature = "ky")]
    "ky",
    #[cfg(feature = "lb")]
    "lb",
    #[cfg(feature = "lo")]
    "lo",
    #[cfg(feature = "lt")]
    "lt",
    #[cfg(feature = "lv")]
    "lv",
    #[cfg(feature = "mai")]
    "mai",
    #[cfg(feature = "mi")]
    "mi",
    #[cfg(feature = "mk")]
    "mk",
    #[cfg(feature = "ml")]
    "ml",
    #[cfg(feature = "mn")]
    "mn",
    #[cfg(feature = "mni")]
    "mni",
    #[cfg(feature = "mr")]
    "mr",
    #[cfg(feature = "ms")]
    "ms",
    #[cfg(feature = "mt")]
    "mt",
    #[cfg(feature = "my")]
    "my",
    #[cfg(feature = "nb")]
    "nb",
    #[cfg(feature = "ne")]
    "ne",
    #[cfg(feature = "nl")]
    "nl",
    #[cfg(feature = "nn")]
    "nn",
    #[cfg(feature = "or")]
    "or",
    #[cfg(feature = "pa")]
    "pa",
    #[cfg(feature = "pa_Arab")]
    "pa_Arab",
    #[cfg(feature = "pcm")]
    "pcm",
    #[cfg(feature = "pl")]
    "pl",
    #[cfg(feature = "ps")]
    "ps",
    #[cfg(feature = "pt")]
    "pt",
    #[cfg(feature = "pt_PT")]
    "pt_PT",
    #[cfg(feature = "qu")]
    "qu",
    #[cfg(feature = "rm")]
    "rm",
    #[cfg(feature = "ro")]
    "ro",
    #[cfg(feature = "root")]
    "root",
    #[cfg(feature = "ru")]
    "ru",
    #[cfg(feature = "rw")]
    "rw",
    #[cfg(feature = "sa")]
    "sa",
    #[cfg(feature = "sat")]
    "sat",
    #[cfg(feature = "sd")]
    "sd",
    #[cfg(feature = "si")]
    "si",
    #[cfg(feature = "sk")]
    "sk",
    #[cfg(feature = "sl")]
    "sl",
    #[cfg(feature = "so")]
    "so",
    #[cfg(feature = "sq")]
    "sq",
    #[cfg(feature = "sr")]
    "sr",
    #[cfg(feature = "sr_Cyrl")]
    "sr_Cyrl",
    #[cfg(feature = "sr_Cyrl_BA")]
    "sr_Cyrl_BA",
    #[cfg(feature = "sr_Latn")]
    "sr_Latn",
    #[cfg(feature = "sr_Latn_BA")]
    "sr_Latn_BA",
    #[cfg(feature = "su")]
    "su",
    #[cfg(feature = "sv")]
    "sv",
    #[cfg(feature = "sw")]
    "sw",
    #[cfg(feature = "sw_KE")]
    "sw_KE",
    #[cfg(feature = "ta")]
    "ta",
    #[cfg(feature = "te")]
    "te",
    #[cfg(feature = "tg")]
    "tg",
    #[cfg(feature = "th")]
    "th",
    #[cfg(feature = "ti")]
    "ti",
    #[cfg(feature = "tk")]
    "tk",
    #[cfg(feature = "to")]
    "to",
    #[cfg(feature = "tr")]
    "tr",
    #[cfg(feature = "tt")]
    "tt",
    #[cfg(feature = "ug")]
    "ug",
    #[cfg(feature = "uk")]
    "uk",
    #[cfg(feature = "ur")]
    "ur",
    #[cfg(feature = "uz")]
    "uz",
    #[cfg(feature = "vi")]
    "vi",
    #[cfg(feature = "wo")]
    "wo",
    #[cfg(feature = "xh")]
    "xh",
    #[cfg(feature = "yo")]
    "yo",
    #[cfg(feature = "yo_BJ")]
    "yo_BJ",
    #[cfg(feature = "yue")]
    "yue",
    #[cfg(feature = "yue_Hans")]
    "yue_Hans",
    #[cfg(feature = "zh")]
    "zh",
    #[cfg(feature = "zh_Hant")]
    "zh_Hant",
    #[cfg(feature = "zh_Hant_HK")]
    "zh_Hant_HK",
    #[cfg(feature = "zu")]
    "zu",
];
#[doc = r" The unicode release version that this crate is compiled against"]
pub const UNICODE_VERSION: f32 = 13.1f32;
#[doc = r" The rfc3339 formatted time of the unicode release that this crate is compiled against"]
pub const UNICODE_RELEASE_TIME: &'static str = "2020-08-28T05:24:13+00:00";
pub mod smileys_and_emotion {
    pub mod cat_face;
    pub mod emotion;
    pub mod face_affection;
    pub mod face_concerned;
    pub mod face_costume;
    pub mod face_glasses;
    pub mod face_hand;
    pub mod face_hat;
    pub mod face_negative;
    pub mod face_neutral_skeptical;
    pub mod face_sleepy;
    pub mod face_smiling;
    pub mod face_tongue;
    pub mod face_unwell;
    pub mod monkey_face;
}
pub mod people_and_body {
    pub mod body_parts;
    pub mod family;
    pub mod hand_fingers_closed;
    pub mod hand_fingers_open;
    pub mod hand_fingers_partial;
    pub mod hand_prop;
    pub mod hand_single_finger;
    pub mod hands;
    pub mod person;
    pub mod person_activity;
    pub mod person_fantasy;
    pub mod person_gesture;
    pub mod person_resting;
    pub mod person_role;
    pub mod person_sport;
    pub mod person_symbol;
}
pub mod component {
    pub mod hair_style;
    pub mod skin_tone;
}
pub mod animals_and_nature {
    pub mod animal_amphibian;
    pub mod animal_bird;
    pub mod animal_bug;
    pub mod animal_mammal;
    pub mod animal_marine;
    pub mod animal_reptile;
    pub mod plant_flower;
    pub mod plant_other;
}
pub mod food_and_drink {
    pub mod dishware;
    pub mod drink;
    pub mod food_asian;
    pub mod food_fruit;
    pub mod food_marine;
    pub mod food_prepared;
    pub mod food_sweet;
    pub mod food_vegetable;
}
pub mod travel_and_places {
    pub mod hotel;
    pub mod place_building;
    pub mod place_geographic;
    pub mod place_map;
    pub mod place_other;
    pub mod place_religious;
    pub mod sky_and_weather;
    pub mod time;
    pub mod transport_air;
    pub mod transport_ground;
    pub mod transport_water;
}
pub mod activities {
    pub mod arts_and_crafts;
    pub mod award_medal;
    pub mod event;
    pub mod game;
    pub mod sport;
}
pub mod objects {
    pub mod book_paper;
    pub mod clothing;
    pub mod computer;
    pub mod household;
    pub mod light_and_video;
    pub mod lock;
    pub mod mail;
    pub mod medical;
    pub mod money;
    pub mod music;
    pub mod musical_instrument;
    pub mod office;
    pub mod other_object;
    pub mod phone;
    pub mod science;
    pub mod sound;
    pub mod tool;
    pub mod writing;
}
pub mod symbols {
    pub mod alphanum;
    pub mod arrow;
    pub mod av_symbol;
    pub mod currency;
    pub mod gender;
    pub mod geometric;
    pub mod keycap;
    pub mod math;
    pub mod other_symbol;
    pub mod punctuation;
    pub mod religion;
    pub mod transport_sign;
    pub mod warning;
    pub mod zodiac;
}
pub mod flags {
    pub mod country_flag;
    pub mod flag;
    pub mod subdivision_flag;
}
