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
    #[doc = " Variants are always less qualified than their parent. You can go from a variant to"]
    #[doc = " the parent via [emoji::lookup_by_glyph::lookup](lookup_by_glyph/fn.lookup.html)"]
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
pub const ANNOTATION_LANGS_TOTAL: &'static [&'static str] = &["en", "fi"];
#[doc = r" Enabled annotation languages (feature dependent)  "]
#[doc = r#" Defaults to `["en"]`. Enable the `XX` features for each language to include annotations for another language. For example, to include Finnish annotations, use the `fi` feature."#]
pub const ANNOTATION_LANGS_AVAILABLE: &'static [&'static str] = &[
    #[cfg(feature = "en")]
    "en",
    #[cfg(feature = "fi")]
    "fi",
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
