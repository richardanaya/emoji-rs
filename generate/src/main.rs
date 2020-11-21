mod sanitize;
use sanitize::*;
mod group_subgroup;
mod emoji;
use emoji::*;
mod lookup_types;
mod create_requests;
use create_requests::*;
mod gather_annotations;
use gather_annotations::*;
mod vectorize_test_data;
use vectorize_test_data::*;
mod lookup_by_glyph;
mod lookup_by_name;
mod gen_lib;
mod search;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Make sure this is ran from workspace root");
    //let annotation_langs = vec!["af", "am", "ar", "ar_SA", "as", "ast", "az", "be", "bg", "bn", "br", "bs", "ca", "ccp", "ceb", "chr", "ckb", "cs", "cy", "da", "de", "de_CH", "doi", "el", "en", "en_001", "en_AU", "en_CA", "en_GB", "en_IN", "es", "es_419", "es_MX", "es_US", "et", "eu", "fa", "fi", "fil", "fo", "fr", "fr_CA", "ga", "gd", "gl", "gu", "ha", "ha_NE", "he", "hi", "hr", "hu", "hy", "ia", "id", "ig", "is", "it", "ja", "jv", "ka", "kab", "kk", "kl", "km", "kn", "ko", "kok", "ku", "ky", "lb", "lo", "lt", "lv", "mai", "mi", "mk", "ml", "mn", "mni", "mr", "ms", "mt", "my", "nb", "ne", "nl", "nn", "or", "pa", "pa_Arab", "pcm", "pl", "ps", "pt", "pt_PT", "qu", "rm", "ro", "root", "ru", "rw", "sa", "sat", "sd", "si", "sk", "sl", "so", "sq", "sr", "sr_Cyrl", "sr_Cyrl_BA", "sr_Latn", "sr_Latn_BA", "su", "sv", "sw", "sw_KE", "ta", "te", "tg", "th", "ti", "tk", "to", "tr", "tt", "ug", "uk", "ur", "uz", "vi", "wo", "xh", "yo", "yo_BJ", "yue", "yue_Hans", "zh", "zh_Hant", "zh_Hant_HK", "zu", ];
    let annotation_langs = vec!["en", "fi"];
    let client = reqwest::Client::new();
    let annotation_text = create_requests(&client, &annotation_langs).await?;

    let annotations = gather_annotations(&annotation_text, &annotation_langs);

    println!("Annotation processing done. Compiling into emoji list");

    let (date, version, groups) = vectorize_test_data(&client, &annotations).await?;

    println!("Generating lookup tables.");
    lookup_by_glyph::dump(&groups);
    lookup_by_name::dump(&groups);
    search::dump();
    
    println!("Dumping emoji library.");
    gen_lib::dump(&groups, &annotation_langs, version, date);
    
    Ok(())
}
