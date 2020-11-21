#[doc = "🛎\u{fe0f}"]
pub const BELLHOP_BELL: crate::Emoji = crate::Emoji {
    glyph: "🛎\u{fe0f}",
    codepoint: "1F6CE FE0F",
    status: crate::Status::FullyQualified,
    introduction_version: 0.7f32,
    name: "bellhop bell",
    group: "Travel & Places",
    subgroup: "hotel",
    is_variant: false,
    variants: &[crate::Emoji {
        glyph: "🛎",
        codepoint: "1F6CE",
        status: crate::Status::Unqualified,
        introduction_version: 0.7f32,
        name: "bellhop bell",
        group: "Travel & Places",
        subgroup: "hotel",
        is_variant: true,
        variants: &[],
        annotations: &[],
    }],
    annotations: &[
        #[cfg(feature = "en")]
        crate::Annotation {
            lang: "en",
            tts: Some("bellhop bell"),
            keywords: &["bell", "bellhop", "hotel"],
        },
        #[cfg(feature = "fi")]
        crate::Annotation {
            lang: "fi",
            tts: Some("soittokello"),
            keywords: &["hotelli", "hotellipoika", "kello", "soittokello"],
        },
    ],
};
#[doc = "🧳"]
pub const LUGGAGE: crate::Emoji = crate::Emoji {
    glyph: "🧳",
    codepoint: "1F9F3",
    status: crate::Status::FullyQualified,
    introduction_version: 11f32,
    name: "luggage",
    group: "Travel & Places",
    subgroup: "hotel",
    is_variant: false,
    variants: &[],
    annotations: &[
        #[cfg(feature = "en")]
        crate::Annotation {
            lang: "en",
            tts: Some("luggage"),
            keywords: &["luggage", "packing", "travel"],
        },
        #[cfg(feature = "fi")]
        crate::Annotation {
            lang: "fi",
            tts: Some("matkatavara"),
            keywords: &[
                "matka",
                "matkalaukku",
                "matkatavara",
                "matkustus",
                "pakkaaminen",
            ],
        },
    ],
};
