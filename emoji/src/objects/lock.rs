#[doc = "🔒"]
pub const LOCKED: crate::Emoji = crate::Emoji {
    glyph: "🔒",
    codepoint: "1F512",
    status: crate::Status::FullyQualified,
    introduction_version: 0.6f32,
    name: "locked",
    group: "Objects",
    subgroup: "lock",
    is_variant: false,
    variants: &[],
    annotations: &[
        #[cfg(feature = "en")]
        crate::Annotation {
            lang: "en",
            tts: Some("locked"),
            keywords: &["closed", "locked"],
        },
        #[cfg(feature = "fi")]
        crate::Annotation {
            lang: "fi",
            tts: Some("lukko"),
            keywords: &["kiinni", "lukko"],
        },
    ],
};
#[doc = "🔓"]
pub const UNLOCKED: crate::Emoji = crate::Emoji {
    glyph: "🔓",
    codepoint: "1F513",
    status: crate::Status::FullyQualified,
    introduction_version: 0.6f32,
    name: "unlocked",
    group: "Objects",
    subgroup: "lock",
    is_variant: false,
    variants: &[],
    annotations: &[
        #[cfg(feature = "en")]
        crate::Annotation {
            lang: "en",
            tts: Some("unlocked"),
            keywords: &["lock", "open", "unlock", "unlocked"],
        },
        #[cfg(feature = "fi")]
        crate::Annotation {
            lang: "fi",
            tts: Some("lukko auki"),
            keywords: &["auki", "avata", "lukko"],
        },
    ],
};
#[doc = "🔏"]
pub const LOCKED_WITH_PEN: crate::Emoji = crate::Emoji {
    glyph: "🔏",
    codepoint: "1F50F",
    status: crate::Status::FullyQualified,
    introduction_version: 0.6f32,
    name: "locked with pen",
    group: "Objects",
    subgroup: "lock",
    is_variant: false,
    variants: &[],
    annotations: &[
        #[cfg(feature = "en")]
        crate::Annotation {
            lang: "en",
            tts: Some("locked with pen"),
            keywords: &["ink", "lock", "locked with pen", "nib", "pen", "privacy"],
        },
        #[cfg(feature = "fi")]
        crate::Annotation {
            lang: "fi",
            tts: Some("lukko ja kynä"),
            keywords: &[
                "kirjoittaa",
                "lukko",
                "lukko ja kynä",
                "mustekynä",
                "suojaus",
            ],
        },
    ],
};
#[doc = "🔐"]
pub const LOCKED_WITH_KEY: crate::Emoji = crate::Emoji {
    glyph: "🔐",
    codepoint: "1F510",
    status: crate::Status::FullyQualified,
    introduction_version: 0.6f32,
    name: "locked with key",
    group: "Objects",
    subgroup: "lock",
    is_variant: false,
    variants: &[],
    annotations: &[
        #[cfg(feature = "en")]
        crate::Annotation {
            lang: "en",
            tts: Some("locked with key"),
            keywords: &["closed", "key", "lock", "locked with key", "secure"],
        },
        #[cfg(feature = "fi")]
        crate::Annotation {
            lang: "fi",
            tts: Some("avain ja lukko kiinni"),
            keywords: &["avain", "avain ja lukko kiinni", "kiinni", "lukko", "turva"],
        },
    ],
};
#[doc = "🔑"]
pub const KEY: crate::Emoji = crate::Emoji {
    glyph: "🔑",
    codepoint: "1F511",
    status: crate::Status::FullyQualified,
    introduction_version: 0.6f32,
    name: "key",
    group: "Objects",
    subgroup: "lock",
    is_variant: false,
    variants: &[],
    annotations: &[
        #[cfg(feature = "en")]
        crate::Annotation {
            lang: "en",
            tts: Some("key"),
            keywords: &["key", "lock", "password"],
        },
        #[cfg(feature = "fi")]
        crate::Annotation {
            lang: "fi",
            tts: Some("avain"),
            keywords: &["avain", "lukittu", "salasana"],
        },
    ],
};
#[doc = "🗝\u{fe0f}"]
pub const OLD_KEY: crate::Emoji = crate::Emoji {
    glyph: "🗝\u{fe0f}",
    codepoint: "1F5DD FE0F",
    status: crate::Status::FullyQualified,
    introduction_version: 0.7f32,
    name: "old key",
    group: "Objects",
    subgroup: "lock",
    is_variant: false,
    variants: &[crate::Emoji {
        glyph: "🗝",
        codepoint: "1F5DD",
        status: crate::Status::Unqualified,
        introduction_version: 0.7f32,
        name: "old key",
        group: "Objects",
        subgroup: "lock",
        is_variant: true,
        variants: &[],
        annotations: &[],
    }],
    annotations: &[
        #[cfg(feature = "en")]
        crate::Annotation {
            lang: "en",
            tts: Some("old key"),
            keywords: &["clue", "key", "lock", "old"],
        },
        #[cfg(feature = "fi")]
        crate::Annotation {
            lang: "fi",
            tts: Some("vanha avain"),
            keywords: &["avain", "lukko", "vanha", "vihje"],
        },
    ],
};
