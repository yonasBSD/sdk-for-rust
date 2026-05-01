use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum EmailTemplateLocale {
    #[serde(rename = "af")]
    #[default]
    Af,
    #[serde(rename = "ar-ae")]
    ArAe,
    #[serde(rename = "ar-bh")]
    ArBh,
    #[serde(rename = "ar-dz")]
    ArDz,
    #[serde(rename = "ar-eg")]
    ArEg,
    #[serde(rename = "ar-iq")]
    ArIq,
    #[serde(rename = "ar-jo")]
    ArJo,
    #[serde(rename = "ar-kw")]
    ArKw,
    #[serde(rename = "ar-lb")]
    ArLb,
    #[serde(rename = "ar-ly")]
    ArLy,
    #[serde(rename = "ar-ma")]
    ArMa,
    #[serde(rename = "ar-om")]
    ArOm,
    #[serde(rename = "ar-qa")]
    ArQa,
    #[serde(rename = "ar-sa")]
    ArSa,
    #[serde(rename = "ar-sy")]
    ArSy,
    #[serde(rename = "ar-tn")]
    ArTn,
    #[serde(rename = "ar-ye")]
    ArYe,
    #[serde(rename = "as")]
    As,
    #[serde(rename = "az")]
    Az,
    #[serde(rename = "be")]
    Be,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "bh")]
    Bh,
    #[serde(rename = "bn")]
    Bn,
    #[serde(rename = "bs")]
    Bs,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "cy")]
    Cy,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "de-at")]
    DeAt,
    #[serde(rename = "de-ch")]
    DeCh,
    #[serde(rename = "de-li")]
    DeLi,
    #[serde(rename = "de-lu")]
    DeLu,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "en-au")]
    EnAu,
    #[serde(rename = "en-bz")]
    EnBz,
    #[serde(rename = "en-ca")]
    EnCa,
    #[serde(rename = "en-gb")]
    EnGb,
    #[serde(rename = "en-ie")]
    EnIe,
    #[serde(rename = "en-jm")]
    EnJm,
    #[serde(rename = "en-nz")]
    EnNz,
    #[serde(rename = "en-tt")]
    EnTt,
    #[serde(rename = "en-us")]
    EnUs,
    #[serde(rename = "en-za")]
    EnZa,
    #[serde(rename = "eo")]
    Eo,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "es-ar")]
    EsAr,
    #[serde(rename = "es-bo")]
    EsBo,
    #[serde(rename = "es-cl")]
    EsCl,
    #[serde(rename = "es-co")]
    EsCo,
    #[serde(rename = "es-cr")]
    EsCr,
    #[serde(rename = "es-do")]
    EsDo,
    #[serde(rename = "es-ec")]
    EsEc,
    #[serde(rename = "es-gt")]
    EsGt,
    #[serde(rename = "es-hn")]
    EsHn,
    #[serde(rename = "es-mx")]
    EsMx,
    #[serde(rename = "es-ni")]
    EsNi,
    #[serde(rename = "es-pa")]
    EsPa,
    #[serde(rename = "es-pe")]
    EsPe,
    #[serde(rename = "es-pr")]
    EsPr,
    #[serde(rename = "es-py")]
    EsPy,
    #[serde(rename = "es-sv")]
    EsSv,
    #[serde(rename = "es-uy")]
    EsUy,
    #[serde(rename = "es-ve")]
    EsVe,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "fa")]
    Fa,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fo")]
    Fo,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "fr-be")]
    FrBe,
    #[serde(rename = "fr-ca")]
    FrCa,
    #[serde(rename = "fr-ch")]
    FrCh,
    #[serde(rename = "fr-lu")]
    FrLu,
    #[serde(rename = "ga")]
    Ga,
    #[serde(rename = "gd")]
    Gd,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "it-ch")]
    ItCh,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ji")]
    Ji,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "ku")]
    Ku,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "mk")]
    Mk,
    #[serde(rename = "ml")]
    Ml,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "mt")]
    Mt,
    #[serde(rename = "nb")]
    Nb,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "nl-be")]
    NlBe,
    #[serde(rename = "nn")]
    Nn,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "pa")]
    Pa,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "pt-br")]
    PtBr,
    #[serde(rename = "rm")]
    Rm,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ro-md")]
    RoMd,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "ru-md")]
    RuMd,
    #[serde(rename = "sb")]
    Sb,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sq")]
    Sq,
    #[serde(rename = "sr")]
    Sr,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "sv-fi")]
    SvFi,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "tn")]
    Tn,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "ts")]
    Ts,
    #[serde(rename = "ua")]
    Ua,
    #[serde(rename = "ur")]
    Ur,
    #[serde(rename = "ve")]
    Ve,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "xh")]
    Xh,
    #[serde(rename = "zh-cn")]
    ZhCn,
    #[serde(rename = "zh-hk")]
    ZhHk,
    #[serde(rename = "zh-sg")]
    ZhSg,
    #[serde(rename = "zh-tw")]
    ZhTw,
    #[serde(rename = "zu")]
    Zu,
}

impl EmailTemplateLocale {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            EmailTemplateLocale::Af => "af",
            EmailTemplateLocale::ArAe => "ar-ae",
            EmailTemplateLocale::ArBh => "ar-bh",
            EmailTemplateLocale::ArDz => "ar-dz",
            EmailTemplateLocale::ArEg => "ar-eg",
            EmailTemplateLocale::ArIq => "ar-iq",
            EmailTemplateLocale::ArJo => "ar-jo",
            EmailTemplateLocale::ArKw => "ar-kw",
            EmailTemplateLocale::ArLb => "ar-lb",
            EmailTemplateLocale::ArLy => "ar-ly",
            EmailTemplateLocale::ArMa => "ar-ma",
            EmailTemplateLocale::ArOm => "ar-om",
            EmailTemplateLocale::ArQa => "ar-qa",
            EmailTemplateLocale::ArSa => "ar-sa",
            EmailTemplateLocale::ArSy => "ar-sy",
            EmailTemplateLocale::ArTn => "ar-tn",
            EmailTemplateLocale::ArYe => "ar-ye",
            EmailTemplateLocale::As => "as",
            EmailTemplateLocale::Az => "az",
            EmailTemplateLocale::Be => "be",
            EmailTemplateLocale::Bg => "bg",
            EmailTemplateLocale::Bh => "bh",
            EmailTemplateLocale::Bn => "bn",
            EmailTemplateLocale::Bs => "bs",
            EmailTemplateLocale::Ca => "ca",
            EmailTemplateLocale::Cs => "cs",
            EmailTemplateLocale::Cy => "cy",
            EmailTemplateLocale::Da => "da",
            EmailTemplateLocale::De => "de",
            EmailTemplateLocale::DeAt => "de-at",
            EmailTemplateLocale::DeCh => "de-ch",
            EmailTemplateLocale::DeLi => "de-li",
            EmailTemplateLocale::DeLu => "de-lu",
            EmailTemplateLocale::El => "el",
            EmailTemplateLocale::En => "en",
            EmailTemplateLocale::EnAu => "en-au",
            EmailTemplateLocale::EnBz => "en-bz",
            EmailTemplateLocale::EnCa => "en-ca",
            EmailTemplateLocale::EnGb => "en-gb",
            EmailTemplateLocale::EnIe => "en-ie",
            EmailTemplateLocale::EnJm => "en-jm",
            EmailTemplateLocale::EnNz => "en-nz",
            EmailTemplateLocale::EnTt => "en-tt",
            EmailTemplateLocale::EnUs => "en-us",
            EmailTemplateLocale::EnZa => "en-za",
            EmailTemplateLocale::Eo => "eo",
            EmailTemplateLocale::Es => "es",
            EmailTemplateLocale::EsAr => "es-ar",
            EmailTemplateLocale::EsBo => "es-bo",
            EmailTemplateLocale::EsCl => "es-cl",
            EmailTemplateLocale::EsCo => "es-co",
            EmailTemplateLocale::EsCr => "es-cr",
            EmailTemplateLocale::EsDo => "es-do",
            EmailTemplateLocale::EsEc => "es-ec",
            EmailTemplateLocale::EsGt => "es-gt",
            EmailTemplateLocale::EsHn => "es-hn",
            EmailTemplateLocale::EsMx => "es-mx",
            EmailTemplateLocale::EsNi => "es-ni",
            EmailTemplateLocale::EsPa => "es-pa",
            EmailTemplateLocale::EsPe => "es-pe",
            EmailTemplateLocale::EsPr => "es-pr",
            EmailTemplateLocale::EsPy => "es-py",
            EmailTemplateLocale::EsSv => "es-sv",
            EmailTemplateLocale::EsUy => "es-uy",
            EmailTemplateLocale::EsVe => "es-ve",
            EmailTemplateLocale::Et => "et",
            EmailTemplateLocale::Eu => "eu",
            EmailTemplateLocale::Fa => "fa",
            EmailTemplateLocale::Fi => "fi",
            EmailTemplateLocale::Fo => "fo",
            EmailTemplateLocale::Fr => "fr",
            EmailTemplateLocale::FrBe => "fr-be",
            EmailTemplateLocale::FrCa => "fr-ca",
            EmailTemplateLocale::FrCh => "fr-ch",
            EmailTemplateLocale::FrLu => "fr-lu",
            EmailTemplateLocale::Ga => "ga",
            EmailTemplateLocale::Gd => "gd",
            EmailTemplateLocale::He => "he",
            EmailTemplateLocale::Hi => "hi",
            EmailTemplateLocale::Hr => "hr",
            EmailTemplateLocale::Hu => "hu",
            EmailTemplateLocale::Id => "id",
            EmailTemplateLocale::Is => "is",
            EmailTemplateLocale::It => "it",
            EmailTemplateLocale::ItCh => "it-ch",
            EmailTemplateLocale::Ja => "ja",
            EmailTemplateLocale::Ji => "ji",
            EmailTemplateLocale::Ko => "ko",
            EmailTemplateLocale::Ku => "ku",
            EmailTemplateLocale::Lt => "lt",
            EmailTemplateLocale::Lv => "lv",
            EmailTemplateLocale::Mk => "mk",
            EmailTemplateLocale::Ml => "ml",
            EmailTemplateLocale::Ms => "ms",
            EmailTemplateLocale::Mt => "mt",
            EmailTemplateLocale::Nb => "nb",
            EmailTemplateLocale::Ne => "ne",
            EmailTemplateLocale::Nl => "nl",
            EmailTemplateLocale::NlBe => "nl-be",
            EmailTemplateLocale::Nn => "nn",
            EmailTemplateLocale::No => "no",
            EmailTemplateLocale::Pa => "pa",
            EmailTemplateLocale::Pl => "pl",
            EmailTemplateLocale::Pt => "pt",
            EmailTemplateLocale::PtBr => "pt-br",
            EmailTemplateLocale::Rm => "rm",
            EmailTemplateLocale::Ro => "ro",
            EmailTemplateLocale::RoMd => "ro-md",
            EmailTemplateLocale::Ru => "ru",
            EmailTemplateLocale::RuMd => "ru-md",
            EmailTemplateLocale::Sb => "sb",
            EmailTemplateLocale::Sk => "sk",
            EmailTemplateLocale::Sl => "sl",
            EmailTemplateLocale::Sq => "sq",
            EmailTemplateLocale::Sr => "sr",
            EmailTemplateLocale::Sv => "sv",
            EmailTemplateLocale::SvFi => "sv-fi",
            EmailTemplateLocale::Th => "th",
            EmailTemplateLocale::Tn => "tn",
            EmailTemplateLocale::Tr => "tr",
            EmailTemplateLocale::Ts => "ts",
            EmailTemplateLocale::Ua => "ua",
            EmailTemplateLocale::Ur => "ur",
            EmailTemplateLocale::Ve => "ve",
            EmailTemplateLocale::Vi => "vi",
            EmailTemplateLocale::Xh => "xh",
            EmailTemplateLocale::ZhCn => "zh-cn",
            EmailTemplateLocale::ZhHk => "zh-hk",
            EmailTemplateLocale::ZhSg => "zh-sg",
            EmailTemplateLocale::ZhTw => "zh-tw",
            EmailTemplateLocale::Zu => "zu",
        }
    }
}

impl std::fmt::Display for EmailTemplateLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
