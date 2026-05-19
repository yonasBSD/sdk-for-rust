use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectEmailTemplateLocale {
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

impl ProjectEmailTemplateLocale {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectEmailTemplateLocale::Af => "af",
            ProjectEmailTemplateLocale::ArAe => "ar-ae",
            ProjectEmailTemplateLocale::ArBh => "ar-bh",
            ProjectEmailTemplateLocale::ArDz => "ar-dz",
            ProjectEmailTemplateLocale::ArEg => "ar-eg",
            ProjectEmailTemplateLocale::ArIq => "ar-iq",
            ProjectEmailTemplateLocale::ArJo => "ar-jo",
            ProjectEmailTemplateLocale::ArKw => "ar-kw",
            ProjectEmailTemplateLocale::ArLb => "ar-lb",
            ProjectEmailTemplateLocale::ArLy => "ar-ly",
            ProjectEmailTemplateLocale::ArMa => "ar-ma",
            ProjectEmailTemplateLocale::ArOm => "ar-om",
            ProjectEmailTemplateLocale::ArQa => "ar-qa",
            ProjectEmailTemplateLocale::ArSa => "ar-sa",
            ProjectEmailTemplateLocale::ArSy => "ar-sy",
            ProjectEmailTemplateLocale::ArTn => "ar-tn",
            ProjectEmailTemplateLocale::ArYe => "ar-ye",
            ProjectEmailTemplateLocale::As => "as",
            ProjectEmailTemplateLocale::Az => "az",
            ProjectEmailTemplateLocale::Be => "be",
            ProjectEmailTemplateLocale::Bg => "bg",
            ProjectEmailTemplateLocale::Bh => "bh",
            ProjectEmailTemplateLocale::Bn => "bn",
            ProjectEmailTemplateLocale::Bs => "bs",
            ProjectEmailTemplateLocale::Ca => "ca",
            ProjectEmailTemplateLocale::Cs => "cs",
            ProjectEmailTemplateLocale::Cy => "cy",
            ProjectEmailTemplateLocale::Da => "da",
            ProjectEmailTemplateLocale::De => "de",
            ProjectEmailTemplateLocale::DeAt => "de-at",
            ProjectEmailTemplateLocale::DeCh => "de-ch",
            ProjectEmailTemplateLocale::DeLi => "de-li",
            ProjectEmailTemplateLocale::DeLu => "de-lu",
            ProjectEmailTemplateLocale::El => "el",
            ProjectEmailTemplateLocale::En => "en",
            ProjectEmailTemplateLocale::EnAu => "en-au",
            ProjectEmailTemplateLocale::EnBz => "en-bz",
            ProjectEmailTemplateLocale::EnCa => "en-ca",
            ProjectEmailTemplateLocale::EnGb => "en-gb",
            ProjectEmailTemplateLocale::EnIe => "en-ie",
            ProjectEmailTemplateLocale::EnJm => "en-jm",
            ProjectEmailTemplateLocale::EnNz => "en-nz",
            ProjectEmailTemplateLocale::EnTt => "en-tt",
            ProjectEmailTemplateLocale::EnUs => "en-us",
            ProjectEmailTemplateLocale::EnZa => "en-za",
            ProjectEmailTemplateLocale::Eo => "eo",
            ProjectEmailTemplateLocale::Es => "es",
            ProjectEmailTemplateLocale::EsAr => "es-ar",
            ProjectEmailTemplateLocale::EsBo => "es-bo",
            ProjectEmailTemplateLocale::EsCl => "es-cl",
            ProjectEmailTemplateLocale::EsCo => "es-co",
            ProjectEmailTemplateLocale::EsCr => "es-cr",
            ProjectEmailTemplateLocale::EsDo => "es-do",
            ProjectEmailTemplateLocale::EsEc => "es-ec",
            ProjectEmailTemplateLocale::EsGt => "es-gt",
            ProjectEmailTemplateLocale::EsHn => "es-hn",
            ProjectEmailTemplateLocale::EsMx => "es-mx",
            ProjectEmailTemplateLocale::EsNi => "es-ni",
            ProjectEmailTemplateLocale::EsPa => "es-pa",
            ProjectEmailTemplateLocale::EsPe => "es-pe",
            ProjectEmailTemplateLocale::EsPr => "es-pr",
            ProjectEmailTemplateLocale::EsPy => "es-py",
            ProjectEmailTemplateLocale::EsSv => "es-sv",
            ProjectEmailTemplateLocale::EsUy => "es-uy",
            ProjectEmailTemplateLocale::EsVe => "es-ve",
            ProjectEmailTemplateLocale::Et => "et",
            ProjectEmailTemplateLocale::Eu => "eu",
            ProjectEmailTemplateLocale::Fa => "fa",
            ProjectEmailTemplateLocale::Fi => "fi",
            ProjectEmailTemplateLocale::Fo => "fo",
            ProjectEmailTemplateLocale::Fr => "fr",
            ProjectEmailTemplateLocale::FrBe => "fr-be",
            ProjectEmailTemplateLocale::FrCa => "fr-ca",
            ProjectEmailTemplateLocale::FrCh => "fr-ch",
            ProjectEmailTemplateLocale::FrLu => "fr-lu",
            ProjectEmailTemplateLocale::Ga => "ga",
            ProjectEmailTemplateLocale::Gd => "gd",
            ProjectEmailTemplateLocale::He => "he",
            ProjectEmailTemplateLocale::Hi => "hi",
            ProjectEmailTemplateLocale::Hr => "hr",
            ProjectEmailTemplateLocale::Hu => "hu",
            ProjectEmailTemplateLocale::Id => "id",
            ProjectEmailTemplateLocale::Is => "is",
            ProjectEmailTemplateLocale::It => "it",
            ProjectEmailTemplateLocale::ItCh => "it-ch",
            ProjectEmailTemplateLocale::Ja => "ja",
            ProjectEmailTemplateLocale::Ji => "ji",
            ProjectEmailTemplateLocale::Ko => "ko",
            ProjectEmailTemplateLocale::Ku => "ku",
            ProjectEmailTemplateLocale::Lt => "lt",
            ProjectEmailTemplateLocale::Lv => "lv",
            ProjectEmailTemplateLocale::Mk => "mk",
            ProjectEmailTemplateLocale::Ml => "ml",
            ProjectEmailTemplateLocale::Ms => "ms",
            ProjectEmailTemplateLocale::Mt => "mt",
            ProjectEmailTemplateLocale::Nb => "nb",
            ProjectEmailTemplateLocale::Ne => "ne",
            ProjectEmailTemplateLocale::Nl => "nl",
            ProjectEmailTemplateLocale::NlBe => "nl-be",
            ProjectEmailTemplateLocale::Nn => "nn",
            ProjectEmailTemplateLocale::No => "no",
            ProjectEmailTemplateLocale::Pa => "pa",
            ProjectEmailTemplateLocale::Pl => "pl",
            ProjectEmailTemplateLocale::Pt => "pt",
            ProjectEmailTemplateLocale::PtBr => "pt-br",
            ProjectEmailTemplateLocale::Rm => "rm",
            ProjectEmailTemplateLocale::Ro => "ro",
            ProjectEmailTemplateLocale::RoMd => "ro-md",
            ProjectEmailTemplateLocale::Ru => "ru",
            ProjectEmailTemplateLocale::RuMd => "ru-md",
            ProjectEmailTemplateLocale::Sb => "sb",
            ProjectEmailTemplateLocale::Sk => "sk",
            ProjectEmailTemplateLocale::Sl => "sl",
            ProjectEmailTemplateLocale::Sq => "sq",
            ProjectEmailTemplateLocale::Sr => "sr",
            ProjectEmailTemplateLocale::Sv => "sv",
            ProjectEmailTemplateLocale::SvFi => "sv-fi",
            ProjectEmailTemplateLocale::Th => "th",
            ProjectEmailTemplateLocale::Tn => "tn",
            ProjectEmailTemplateLocale::Tr => "tr",
            ProjectEmailTemplateLocale::Ts => "ts",
            ProjectEmailTemplateLocale::Ua => "ua",
            ProjectEmailTemplateLocale::Ur => "ur",
            ProjectEmailTemplateLocale::Ve => "ve",
            ProjectEmailTemplateLocale::Vi => "vi",
            ProjectEmailTemplateLocale::Xh => "xh",
            ProjectEmailTemplateLocale::ZhCn => "zh-cn",
            ProjectEmailTemplateLocale::ZhHk => "zh-hk",
            ProjectEmailTemplateLocale::ZhSg => "zh-sg",
            ProjectEmailTemplateLocale::ZhTw => "zh-tw",
            ProjectEmailTemplateLocale::Zu => "zu",
        }
    }
}

impl std::fmt::Display for ProjectEmailTemplateLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
