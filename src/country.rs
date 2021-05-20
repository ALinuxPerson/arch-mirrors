// use crate::utils::StrExt;
//
// #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
// pub enum Kind {
//     Australia,
//     France,
//     Germany,
//     UnitedStates,
//     Hungary,
//     Ireland,
//     Netherlands,
//     Switzerland,
//     Turkey,
//     UnitedKingdom,
//     Canada,
//     Norway,
//     Israel,
//     Brazil,
//     Russia,
//     Chile,
//     Spain,
//     NewCaledonia,
//     Greece,
//     India,
//     Taiwan,
//     China,
//     Belgium,
//     Portugal,
//     Denmark,
//     Japan,
//     Belarus,
//     Czechia,
//     Italy,
//     Luxembourg,
//     Ukraine,
//     Sweden,
//     NorthMacedonia,
//     Slovakia,
//     Kazakhstan,
//     Serbia,
//     Singapore,
//     Poland,
//     Romania,
//     Iceland,
//     HongKong,
//     Indonesia,
//     SouthKorea,
//     Croatia,
//     Ecuador,
//     Vietnam,
//     Lithuania,
//     Latvia,
//     Bulgaria,
//     Austria,
//     SouthAfrica,
//     Finland,
//     Slovenia,
//     BosniaAndHerzegovina,
//     NewZealand,
//     Thailand,
//     Iran,
//     Bangladesh,
//     Paraguay,
//     Colombia,
//     Georgia,
//     Kenya,
//     Pakistan,
//     Moldova,
//     Estonia,
//     Mexico,
//     Monaco,
//     Other(String),
// }
//
// impl From<String> for Kind {
//     fn from(string: String) -> Self {
//         match string.as_str() {
//             "Australia" => Self::Australia,
//             "France" => Self::France,
//             "Germany" => Self::Germany,
//             "United States" => Self::UnitedStates,
//             "Hungary" => Self::Hungary,
//             "Ireland" => Self::Ireland,
//             "Netherlands" => Self::Netherlands,
//             "Switzerland" => Self::Switzerland,
//             "Turkey" => Self::Turkey,
//             "United Kingdom" => Self::UnitedKingdom,
//             "Canada" => Self::Canada,
//             "Norway" => Self::Norway,
//             "Israel" => Self::Israel,
//             "Brazil" => Self::Brazil,
//             "Russia" => Self::Russia,
//             "Chile" => Self::Chile,
//             "Spain" => Self::Spain,
//             "New Caledonia" => Self::NewCaledonia,
//             "Greece" => Self::Greece,
//             "India" => Self::India,
//             "Taiwan" => Self::Taiwan,
//             "China" => Self::China,
//             "Belgium" => Self::Belgium,
//             "Portugal" => Self::Portugal,
//             "Denmark" => Self::Denmark,
//             "Japan" => Self::Japan,
//             "Belarus" => Self::Belarus,
//             "Czechia" => Self::Czechia,
//             "Italy" => Self::Italy,
//             "Luxembourg" => Self::Luxembourg,
//             "Ukraine" => Self::Ukraine,
//             "Sweden" => Self::Sweden,
//             "North Macedonia" => Self::NorthMacedonia,
//             "Slovakia" => Self::Slovakia,
//             "Kazakhstan" => Self::Kazakhstan,
//             "Serbia" => Self::Serbia,
//             "Singapore" => Self::Singapore,
//             "Poland" => Self::Poland,
//             "Romania" => Self::Romania,
//             "Iceland" => Self::Iceland,
//             "Hong Kong" => Self::HongKong,
//             "Indonesia" => Self::Indonesia,
//             "South Korea" => Self::SouthKorea,
//             "Croatia" => Self::Croatia,
//             "Ecuador" => Self::Ecuador,
//             "Vietnam" => Self::Vietnam,
//             "Lithuania" => Self::Lithuania,
//             "Latvia" => Self::Latvia,
//             "Bulgaria" => Self::Bulgaria,
//             "Austria" => Self::Austria,
//             "South Africa" => Self::SouthAfrica,
//             "Finland" => Self::Finland,
//             "Slovenia" => Self::Slovenia,
//             "Bosnia and Herzegovina" => Self::BosniaAndHerzegovina,
//             "New Zealand" => Self::NewZealand,
//             "Thailand" => Self::Thailand,
//             "Iran" => Self::Iran,
//             "Bangladesh" => Self::Bangladesh,
//             "Paraguay" => Self::Paraguay,
//             "Colombia" => Self::Colombia,
//             "Georgia" => Self::Georgia,
//             "Kenya" => Self::Kenya,
//             "Pakistan" => Self::Pakistan,
//             "Moldova" => Self::Moldova,
//             "Estonia" => Self::Estonia,
//             "Mexico" => Self::Mexico,
//             "Monaco" => Self::Monaco,
//             _ => Self::Other(string),
//         }
//     }
// }
//
// #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
// pub enum Code {
//     AU,
//     FR,
//     DE,
//     US,
//     HU,
//     IE,
//     NL,
//     CH,
//     TR,
//     GB,
//     CA,
//     NO,
//     IL,
//     BR,
//     RU,
//     CL,
//     ES,
//     NC,
//     GR,
//     IN,
//     TW,
//     CN,
//     BE,
//     PT,
//     DK,
//     JP,
//     BY,
//     CZ,
//     IT,
//     LU,
//     UA,
//     SE,
//     MK,
//     SK,
//     KZ,
//     RS,
//     SG,
//     PL,
//     RO,
//     IS,
//     HK,
//     ID,
//     KR,
//     HR,
//     EC,
//     VN,
//     LT,
//     LV,
//     BG,
//     AT,
//     ZA,
//     FI,
//     SI,
//     BA,
//     NZ,
//     TH,
//     IR,
//     BD,
//     PY,
//     CO,
//     GE,
//     KE,
//     PK,
//     MD,
//     EE,
//     MX,
//     MC,
//     Other(String),
// }
//
// impl From<String> for Code {
//     fn from(string: String) -> Self {
//         match string.as_str() {
//             "AU" => Self::AU,
//             "FR" => Self::FR,
//             "DE" => Self::DE,
//             "US" => Self::US,
//             "HU" => Self::HU,
//             "IE" => Self::IE,
//             "NL" => Self::NL,
//             "CH" => Self::CH,
//             "TR" => Self::TR,
//             "GB" => Self::GB,
//             "CA" => Self::CA,
//             "NO" => Self::NO,
//             "IL" => Self::IL,
//             "BR" => Self::BR,
//             "RU" => Self::RU,
//             "CL" => Self::CL,
//             "ES" => Self::ES,
//             "NC" => Self::NC,
//             "GR" => Self::GR,
//             "IN" => Self::IN,
//             "TW" => Self::TW,
//             "CN" => Self::CN,
//             "BE" => Self::BE,
//             "PT" => Self::PT,
//             "DK" => Self::DK,
//             "JP" => Self::JP,
//             "BY" => Self::BY,
//             "CZ" => Self::CZ,
//             "IT" => Self::IT,
//             "LU" => Self::LU,
//             "UA" => Self::UA,
//             "SE" => Self::SE,
//             "MK" => Self::MK,
//             "SK" => Self::SK,
//             "KZ" => Self::KZ,
//             "RS" => Self::RS,
//             "SG" => Self::SG,
//             "PL" => Self::PL,
//             "RO" => Self::RO,
//             "IS" => Self::IS,
//             "HK" => Self::HK,
//             "ID" => Self::ID,
//             "KR" => Self::KR,
//             "HR" => Self::HR,
//             "EC" => Self::EC,
//             "VN" => Self::VN,
//             "LT" => Self::LT,
//             "LV" => Self::LV,
//             "BG" => Self::BG,
//             "AT" => Self::AT,
//             "ZA" => Self::ZA,
//             "FI" => Self::FI,
//             "SI" => Self::SI,
//             "BA" => Self::BA,
//             "NZ" => Self::NZ,
//             "TH" => Self::TH,
//             "IR" => Self::IR,
//             "BD" => Self::BD,
//             "PY" => Self::PY,
//             "CO" => Self::CO,
//             "GE" => Self::GE,
//             "KE" => Self::KE,
//             "PK" => Self::PK,
//             "MD" => Self::MD,
//             "EE" => Self::EE,
//             "MX" => Self::MX,
//             "MC" => Self::MC,
//             _ => Self::Other(string),
//         }
//     }
// }
//
// #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
// pub struct Country {
//     pub kind: Option<Kind>,
//     pub code: Option<Code>,
// }
//
// impl Country {
//     pub const AUSTRALIA: Country = Country {
//         kind: Some(Kind::Australia),
//         code: Some(Code::AU),
//     };
//     pub const FRANCE: Country = Country {
//         kind: Some(Kind::France),
//         code: Some(Code::FR),
//     };
//     pub const GERMANY: Country = Country {
//         kind: Some(Kind::Germany),
//         code: Some(Code::DE),
//     };
//     pub const UNITED_STATES: Country = Country {
//         kind: Some(Kind::UnitedStates),
//         code: Some(Code::US),
//     };
//     pub const HUNGARY: Country = Country {
//         kind: Some(Kind::Hungary),
//         code: Some(Code::HU),
//     };
//     pub const IRELAND: Country = Country {
//         kind: Some(Kind::Ireland),
//         code: Some(Code::IE),
//     };
//     pub const NETHERLANDS: Country = Country {
//         kind: Some(Kind::Netherlands),
//         code: Some(Code::NL),
//     };
//     pub const SWITZERLAND: Country = Country {
//         kind: Some(Kind::Switzerland),
//         code: Some(Code::CH),
//     };
//     pub const TURKEY: Country = Country {
//         kind: Some(Kind::Turkey),
//         code: Some(Code::TR),
//     };
//     pub const UNITED_KINGDOM: Country = Country {
//         kind: Some(Kind::UnitedKingdom),
//         code: Some(Code::GB),
//     };
//     pub const CANADA: Country = Country {
//         kind: Some(Kind::Canada),
//         code: Some(Code::CA),
//     };
//     pub const NORWAY: Country = Country {
//         kind: Some(Kind::Norway),
//         code: Some(Code::NO),
//     };
//     pub const ISRAEL: Country = Country {
//         kind: Some(Kind::Israel),
//         code: Some(Code::IL),
//     };
//     pub const BRAZIL: Country = Country {
//         kind: Some(Kind::Brazil),
//         code: Some(Code::BR),
//     };
//     pub const RUSSIA: Country = Country {
//         kind: Some(Kind::Russia),
//         code: Some(Code::RU),
//     };
//     pub const CHILE: Country = Country {
//         kind: Some(Kind::Chile),
//         code: Some(Code::CL),
//     };
//     pub const SPAIN: Country = Country {
//         kind: Some(Kind::Spain),
//         code: Some(Code::ES),
//     };
//     pub const NEW_CALEDONIA: Country = Country {
//         kind: Some(Kind::NewCaledonia),
//         code: Some(Code::NC),
//     };
//     pub const GREECE: Country = Country {
//         kind: Some(Kind::Greece),
//         code: Some(Code::GR),
//     };
//     pub const INDIA: Country = Country {
//         kind: Some(Kind::India),
//         code: Some(Code::IN),
//     };
//     pub const TAIWAN: Country = Country {
//         kind: Some(Kind::Taiwan),
//         code: Some(Code::TW),
//     };
//     pub const CHINA: Country = Country {
//         kind: Some(Kind::China),
//         code: Some(Code::CN),
//     };
//     pub const BELGIUM: Country = Country {
//         kind: Some(Kind::Belgium),
//         code: Some(Code::BE),
//     };
//     pub const PORTUGAL: Country = Country {
//         kind: Some(Kind::Portugal),
//         code: Some(Code::PT),
//     };
//     pub const DENMARK: Country = Country {
//         kind: Some(Kind::Denmark),
//         code: Some(Code::DK),
//     };
//     pub const JAPAN: Country = Country {
//         kind: Some(Kind::Japan),
//         code: Some(Code::JP),
//     };
//     pub const BELARUS: Country = Country {
//         kind: Some(Kind::Belarus),
//         code: Some(Code::BY),
//     };
//     pub const CZECHIA: Country = Country {
//         kind: Some(Kind::Czechia),
//         code: Some(Code::CZ),
//     };
//     pub const ITALY: Country = Country {
//         kind: Some(Kind::Italy),
//         code: Some(Code::IT),
//     };
//     pub const LUXEMBOURG: Country = Country {
//         kind: Some(Kind::Luxembourg),
//         code: Some(Code::LU),
//     };
//     pub const UKRAINE: Country = Country {
//         kind: Some(Kind::Ukraine),
//         code: Some(Code::UA),
//     };
//     pub const SWEDEN: Country = Country {
//         kind: Some(Kind::Sweden),
//         code: Some(Code::SE),
//     };
//     pub const NORTH_MACEDONIA: Country = Country {
//         kind: Some(Kind::NorthMacedonia),
//         code: Some(Code::MK),
//     };
//     pub const SLOVAKIA: Country = Country {
//         kind: Some(Kind::Slovakia),
//         code: Some(Code::SK),
//     };
//     pub const KAZAKHSTAN: Country = Country {
//         kind: Some(Kind::Kazakhstan),
//         code: Some(Code::KZ),
//     };
//     pub const SERBIA: Country = Country {
//         kind: Some(Kind::Serbia),
//         code: Some(Code::RS),
//     };
//     pub const SINGAPORE: Country = Country {
//         kind: Some(Kind::Singapore),
//         code: Some(Code::SG),
//     };
//     pub const POLAND: Country = Country {
//         kind: Some(Kind::Poland),
//         code: Some(Code::PL),
//     };
//     pub const ROMANIA: Country = Country {
//         kind: Some(Kind::Romania),
//         code: Some(Code::RO),
//     };
//     pub const ICELAND: Country = Country {
//         kind: Some(Kind::Iceland),
//         code: Some(Code::IS),
//     };
//     pub const HONG_KONG: Country = Country {
//         kind: Some(Kind::HongKong),
//         code: Some(Code::HK),
//     };
//     pub const EMPTY: Country = Country {
//         kind: None,
//         code: None,
//     };
//     pub const INDONESIA: Country = Country {
//         kind: Some(Kind::Indonesia),
//         code: Some(Code::ID),
//     };
//     pub const SOUTH_KOREA: Country = Country {
//         kind: Some(Kind::SouthKorea),
//         code: Some(Code::KR),
//     };
//     pub const CROATIA: Country = Country {
//         kind: Some(Kind::Croatia),
//         code: Some(Code::HR),
//     };
//     pub const ECUADOR: Country = Country {
//         kind: Some(Kind::Ecuador),
//         code: Some(Code::EC),
//     };
//     pub const VIETNAM: Country = Country {
//         kind: Some(Kind::Vietnam),
//         code: Some(Code::VN),
//     };
//     pub const LITHUANIA: Country = Country {
//         kind: Some(Kind::Lithuania),
//         code: Some(Code::LT),
//     };
//     pub const LATVIA: Country = Country {
//         kind: Some(Kind::Latvia),
//         code: Some(Code::LV),
//     };
//     pub const BULGARIA: Country = Country {
//         kind: Some(Kind::Bulgaria),
//         code: Some(Code::BG),
//     };
//     pub const AUSTRIA: Country = Country {
//         kind: Some(Kind::Austria),
//         code: Some(Code::AT),
//     };
//     pub const SOUTH_AFRICA: Country = Country {
//         kind: Some(Kind::SouthAfrica),
//         code: Some(Code::ZA),
//     };
//     pub const FINLAND: Country = Country {
//         kind: Some(Kind::Finland),
//         code: Some(Code::FI),
//     };
//     pub const SLOVENIA: Country = Country {
//         kind: Some(Kind::Slovenia),
//         code: Some(Code::SI),
//     };
//     pub const BOSNIA_AND_HERZEGOVINA: Country = Country {
//         kind: Some(Kind::BosniaAndHerzegovina),
//         code: Some(Code::BA),
//     };
//     pub const NEW_ZEALAND: Country = Country {
//         kind: Some(Kind::NewZealand),
//         code: Some(Code::NZ),
//     };
//     pub const THAILAND: Country = Country {
//         kind: Some(Kind::Thailand),
//         code: Some(Code::TH),
//     };
//     pub const IRAN: Country = Country {
//         kind: Some(Kind::Iran),
//         code: Some(Code::IR),
//     };
//     pub const BANGLADESH: Country = Country {
//         kind: Some(Kind::Bangladesh),
//         code: Some(Code::BD),
//     };
//     pub const PARAGUAY: Country = Country {
//         kind: Some(Kind::Paraguay),
//         code: Some(Code::PY),
//     };
//     pub const COLOMBIA: Country = Country {
//         kind: Some(Kind::Colombia),
//         code: Some(Code::CO),
//     };
//     pub const GEORGIA: Country = Country {
//         kind: Some(Kind::Georgia),
//         code: Some(Code::GE),
//     };
//     pub const KENYA: Country = Country {
//         kind: Some(Kind::Kenya),
//         code: Some(Code::KE),
//     };
//     pub const PAKISTAN: Country = Country {
//         kind: Some(Kind::Pakistan),
//         code: Some(Code::PK),
//     };
//     pub const MOLDOVA: Country = Country {
//         kind: Some(Kind::Moldova),
//         code: Some(Code::MD),
//     };
//     pub const ESTONIA: Country = Country {
//         kind: Some(Kind::Estonia),
//         code: Some(Code::EE),
//     };
//     pub const MEXICO: Country = Country {
//         kind: Some(Kind::Mexico),
//         code: Some(Code::MX),
//     };
//     pub const MONACO: Country = Country {
//         kind: Some(Kind::Monaco),
//         code: Some(Code::MC),
//     };
//
//     pub fn new(country: &str, code: &str) -> Self {
//         let kind = country
//             .into_option()
//             .map(ToString::to_string)
//             .map(Kind::from);
//         let code = code.into_option().map(ToString::to_string).map(Code::from);
//
//         Self { kind, code }
//     }
// }

macro_rules! countries {
    ($($(#[$docs:meta])* $name:literal ($snake_case:ident): $kind:ident => $code:ident),+) => {
        /// The country name.
        #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub enum Kind {
            $($(#[$docs])* $kind),+,

            /// Any unsupported country.
            Other(String)
        }

        impl From<String> for Kind {
            fn from(kind: String) -> Self {
                match kind.as_str() {
                    $($name => Self::$kind),+,
                    _ => Self::Other(kind)
                }
            }
        }

        /// The country code.
        #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub enum Code {
             $($(#[$docs])* $code),+,

            /// Any unsupported country code.
            Other(String)
        }

        impl From<String> for Code {
            fn from(code: String) -> Self {
                match code.as_str() {
                    $(stringify!($code) => Self::$code),+,
                    _ => Self::Other(code)
                }
            }
        }

        /// The country.
        #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
        pub struct Country {
            /// The kind of country.
            pub kind: Option<Kind>,

            /// The country code.
            pub code: Option<Code>
        }

        impl Country {
            $(
            $(#[$docs])*
            pub const $snake_case: Country = Country { kind: Some(Kind::$kind), code: Some(Code::$code) };
            )+

            /// Create a new country. If country or code is empty, this will be set to [`None`](None).
            pub fn new(country: &str, code: &str) -> Self {
                use $crate::utils::StrExt;
                let kind = country.into_option().map(ToString::to_string).map(Kind::from);
                let code = code.into_option().map(ToString::to_string).map(Code::from);

                Self { kind, code }
            }
        }
    };
}

countries! {
    /// Australia.
    "Australia" (AUSTRALIA): Australia => AU,
    "Germany" (GERMANY): Germany => DE
}
