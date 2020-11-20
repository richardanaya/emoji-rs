#[doc = "🔒"]
pub const LOCKED: crate::Emoji = crate::Emoji {
    glyph: "🔒",
    codepoint: "1F512",
    status: crate::Status::FullyQualified,
    introduction_version: 0.6f32,
    name: "locked",
    annotations: &[
        crate::Annotation {
            lang: "en",
            tts: Some("locked"),
            keywords: &["closed", "locked"],
        },
        crate::Annotation {
            lang: "fi",
            tts: Some("lukko"),
            keywords: &["kiinni", "lukko"],
        },
    ],
    variants: &[],
};
#[doc = "🔓"]
pub const UNLOCKED: crate::Emoji = crate::Emoji {
    glyph: "🔓",
    codepoint: "1F513",
    status: crate::Status::FullyQualified,
    introduction_version: 0.6f32,
    name: "unlocked",
    annotations: &[
        crate::Annotation {
            lang: "en",
            tts: Some("unlocked"),
            keywords: &["lock", "open", "unlock", "unlocked"],
        },
        crate::Annotation {
            lang: "fi",
            tts: Some("lukko auki"),
            keywords: &["auki", "avata", "lukko"],
        },
    ],
    variants: &[],
};
#[doc = "🔏"]
pub const LOCKED_WITH_PEN: crate::Emoji = crate::Emoji {
    glyph: "🔏",
    codepoint: "1F50F",
    status: crate::Status::FullyQualified,
    introduction_version: 0.6f32,
    name: "locked with pen",
    annotations: &[
        crate::Annotation {
            lang: "en",
            tts: Some("locked with pen"),
            keywords: &["ink", "lock", "locked with pen", "nib", "pen", "privacy"],
        },
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
    variants: &[],
};
#[doc = "🔐"]
pub const LOCKED_WITH_KEY: crate::Emoji = crate::Emoji {
    glyph: "🔐",
    codepoint: "1F510",
    status: crate::Status::FullyQualified,
    introduction_version: 0.6f32,
    name: "locked with key",
    annotations: &[
        crate::Annotation {
            lang: "en",
            tts: Some("locked with key"),
            keywords: &["closed", "key", "lock", "locked with key", "secure"],
        },
        crate::Annotation {
            lang: "fi",
            tts: Some("avain ja lukko kiinni"),
            keywords: &["avain", "avain ja lukko kiinni", "kiinni", "lukko", "turva"],
        },
    ],
    variants: &[],
};
#[doc = "🔑"]
pub const KEY: crate::Emoji = crate::Emoji {
    glyph: "🔑",
    codepoint: "1F511",
    status: crate::Status::FullyQualified,
    introduction_version: 0.6f32,
    name: "key",
    annotations: &[
        crate::Annotation {
            lang: "en",
            tts: Some("key"),
            keywords: &["key", "lock", "password"],
        },
        crate::Annotation {
            lang: "fi",
            tts: Some("avain"),
            keywords: &["avain", "lukittu", "salasana"],
        },
    ],
    variants: &[],
};
#[doc = "🗝\u{fe0f}"]
pub const OLD_KEY: crate::Emoji = crate::Emoji {
    glyph: "🗝\u{fe0f}",
    codepoint: "1F5DD FE0F",
    status: crate::Status::FullyQualified,
    introduction_version: 0.7f32,
    name: "old key",
    annotations: &[
        crate::Annotation {
            lang: "en",
            tts: Some("old key"),
            keywords: &["clue", "key", "lock", "old"],
        },
        crate::Annotation {
            lang: "fi",
            tts: Some("vanha avain"),
            keywords: &["avain", "lukko", "vanha", "vihje"],
        },
    ],
    variants: &[crate::Emoji {
        glyph: "🗝",
        codepoint: "1F5DD",
        status: crate::Status::Unqualified,
        introduction_version: 0.7f32,
        name: "old key",
        annotations: &[],
        variants: &[],
    }],
};
