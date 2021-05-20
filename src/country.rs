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
    /// The country Australia. It's kind is [Kind::Australia](`Kind::Australia`). It's code is [Code::AU](`Code::AU`). It's constant is [Country::AUSTRALIA](`Country::AUSTRALIA`).
    "Australia" (AUSTRALIA): Australia => AU,
    /// The country France. It's kind is [Kind::France](`Kind::France`). It's code is [Code::FR](`Code::FR`). It's constant is [Country::FRANCE](`Country::FRANCE`).
    "France" (FRANCE): France => FR,
    /// The country Germany. It's kind is [Kind::Germany](`Kind::Germany`). It's code is [Code::DE](`Code::DE`). It's constant is [Country::GERMANY](`Country::GERMANY`).
    "Germany" (GERMANY): Germany => DE,
    /// The country United States. It's kind is [Kind::UnitedStates](`Kind::UnitedStates`). It's code is [Code::US](`Code::US`). It's constant is [Country::UNITED_STATES](`Country::UNITED_STATES`).
    "United States" (UNITED_STATES): UnitedStates => US,
    /// The country Hungary. It's kind is [Kind::Hungary](`Kind::Hungary`). It's code is [Code::HU](`Code::HU`). It's constant is [Country::HUNGARY](`Country::HUNGARY`).
    "Hungary" (HUNGARY): Hungary => HU,
    /// The country Ireland. It's kind is [Kind::Ireland](`Kind::Ireland`). It's code is [Code::IE](`Code::IE`). It's constant is [Country::IRELAND](`Country::IRELAND`).
    "Ireland" (IRELAND): Ireland => IE,
    /// The country Netherlands. It's kind is [Kind::Netherlands](`Kind::Netherlands`). It's code is [Code::NL](`Code::NL`). It's constant is [Country::NETHERLANDS](`Country::NETHERLANDS`).
    "Netherlands" (NETHERLANDS): Netherlands => NL,
    /// The country Switzerland. It's kind is [Kind::Switzerland](`Kind::Switzerland`). It's code is [Code::CH](`Code::CH`). It's constant is [Country::SWITZERLAND](`Country::SWITZERLAND`).
    "Switzerland" (SWITZERLAND): Switzerland => CH,
    /// The country Turkey. It's kind is [Kind::Turkey](`Kind::Turkey`). It's code is [Code::TR](`Code::TR`). It's constant is [Country::TURKEY](`Country::TURKEY`).
    "Turkey" (TURKEY): Turkey => TR,
    /// The country United Kingdom. It's kind is [Kind::UnitedKingdom](`Kind::UnitedKingdom`). It's code is [Code::GB](`Code::GB`). It's constant is [Country::UNITED_KINGDOM](`Country::UNITED_KINGDOM`).
    "United Kingdom" (UNITED_KINGDOM): UnitedKingdom => GB,
    /// The country Canada. It's kind is [Kind::Canada](`Kind::Canada`). It's code is [Code::CA](`Code::CA`). It's constant is [Country::CANADA](`Country::CANADA`).
    "Canada" (CANADA): Canada => CA,
    /// The country Norway. It's kind is [Kind::Norway](`Kind::Norway`). It's code is [Code::NO](`Code::NO`). It's constant is [Country::NORWAY](`Country::NORWAY`).
    "Norway" (NORWAY): Norway => NO,
    /// The country Israel. It's kind is [Kind::Israel](`Kind::Israel`). It's code is [Code::IL](`Code::IL`). It's constant is [Country::ISRAEL](`Country::ISRAEL`).
    "Israel" (ISRAEL): Israel => IL,
    /// The country Brazil. It's kind is [Kind::Brazil](`Kind::Brazil`). It's code is [Code::BR](`Code::BR`). It's constant is [Country::BRAZIL](`Country::BRAZIL`).
    "Brazil" (BRAZIL): Brazil => BR,
    /// The country Russia. It's kind is [Kind::Russia](`Kind::Russia`). It's code is [Code::RU](`Code::RU`). It's constant is [Country::RUSSIA](`Country::RUSSIA`).
    "Russia" (RUSSIA): Russia => RU,
    /// The country Chile. It's kind is [Kind::Chile](`Kind::Chile`). It's code is [Code::CL](`Code::CL`). It's constant is [Country::CHILE](`Country::CHILE`).
    "Chile" (CHILE): Chile => CL,
    /// The country Spain. It's kind is [Kind::Spain](`Kind::Spain`). It's code is [Code::ES](`Code::ES`). It's constant is [Country::SPAIN](`Country::SPAIN`).
    "Spain" (SPAIN): Spain => ES,
    /// The country New Caledonia. It's kind is [Kind::NewCaledonia](`Kind::NewCaledonia`). It's code is [Code::NC](`Code::NC`). It's constant is [Country::NEW_CALEDONIA](`Country::NEW_CALEDONIA`).
    "New Caledonia" (NEW_CALEDONIA): NewCaledonia => NC,
    /// The country Greece. It's kind is [Kind::Greece](`Kind::Greece`). It's code is [Code::GR](`Code::GR`). It's constant is [Country::GREECE](`Country::GREECE`).
    "Greece" (GREECE): Greece => GR,
    /// The country India. It's kind is [Kind::India](`Kind::India`). It's code is [Code::IN](`Code::IN`). It's constant is [Country::INDIA](`Country::INDIA`).
    "India" (INDIA): India => IN,
    /// The country Taiwan. It's kind is [Kind::Taiwan](`Kind::Taiwan`). It's code is [Code::TW](`Code::TW`). It's constant is [Country::TAIWAN](`Country::TAIWAN`).
    "Taiwan" (TAIWAN): Taiwan => TW,
    /// The country China. It's kind is [Kind::China](`Kind::China`). It's code is [Code::CN](`Code::CN`). It's constant is [Country::CHINA](`Country::CHINA`).
    "China" (CHINA): China => CN,
    /// The country Belgium. It's kind is [Kind::Belgium](`Kind::Belgium`). It's code is [Code::BE](`Code::BE`). It's constant is [Country::BELGIUM](`Country::BELGIUM`).
    "Belgium" (BELGIUM): Belgium => BE,
    /// The country Portugal. It's kind is [Kind::Portugal](`Kind::Portugal`). It's code is [Code::PT](`Code::PT`). It's constant is [Country::PORTUGAL](`Country::PORTUGAL`).
    "Portugal" (PORTUGAL): Portugal => PT,
    /// The country Denmark. It's kind is [Kind::Denmark](`Kind::Denmark`). It's code is [Code::DK](`Code::DK`). It's constant is [Country::DENMARK](`Country::DENMARK`).
    "Denmark" (DENMARK): Denmark => DK,
    /// The country Japan. It's kind is [Kind::Japan](`Kind::Japan`). It's code is [Code::JP](`Code::JP`). It's constant is [Country::JAPAN](`Country::JAPAN`).
    "Japan" (JAPAN): Japan => JP,
    /// The country Belarus. It's kind is [Kind::Belarus](`Kind::Belarus`). It's code is [Code::BY](`Code::BY`). It's constant is [Country::BELARUS](`Country::BELARUS`).
    "Belarus" (BELARUS): Belarus => BY,
    /// The country Czechia. It's kind is [Kind::Czechia](`Kind::Czechia`). It's code is [Code::CZ](`Code::CZ`). It's constant is [Country::CZECHIA](`Country::CZECHIA`).
    "Czechia" (CZECHIA): Czechia => CZ,
    /// The country Italy. It's kind is [Kind::Italy](`Kind::Italy`). It's code is [Code::IT](`Code::IT`). It's constant is [Country::ITALY](`Country::ITALY`).
    "Italy" (ITALY): Italy => IT,
    /// The country Luxembourg. It's kind is [Kind::Luxembourg](`Kind::Luxembourg`). It's code is [Code::LU](`Code::LU`). It's constant is [Country::LUXEMBOURG](`Country::LUXEMBOURG`).
    "Luxembourg" (LUXEMBOURG): Luxembourg => LU,
    /// The country Ukraine. It's kind is [Kind::Ukraine](`Kind::Ukraine`). It's code is [Code::UA](`Code::UA`). It's constant is [Country::UKRAINE](`Country::UKRAINE`).
    "Ukraine" (UKRAINE): Ukraine => UA,
    /// The country Sweden. It's kind is [Kind::Sweden](`Kind::Sweden`). It's code is [Code::SE](`Code::SE`). It's constant is [Country::SWEDEN](`Country::SWEDEN`).
    "Sweden" (SWEDEN): Sweden => SE,
    /// The country North Macedonia. It's kind is [Kind::NorthMacedonia](`Kind::NorthMacedonia`). It's code is [Code::MK](`Code::MK`). It's constant is [Country::NORTH_MACEDONIA](`Country::NORTH_MACEDONIA`).
    "North Macedonia" (NORTH_MACEDONIA): NorthMacedonia => MK,
    /// The country Slovakia. It's kind is [Kind::Slovakia](`Kind::Slovakia`). It's code is [Code::SK](`Code::SK`). It's constant is [Country::SLOVAKIA](`Country::SLOVAKIA`).
    "Slovakia" (SLOVAKIA): Slovakia => SK,
    /// The country Kazakhstan. It's kind is [Kind::Kazakhstan](`Kind::Kazakhstan`). It's code is [Code::KZ](`Code::KZ`). It's constant is [Country::KAZAKHSTAN](`Country::KAZAKHSTAN`).
    "Kazakhstan" (KAZAKHSTAN): Kazakhstan => KZ,
    /// The country Serbia. It's kind is [Kind::Serbia](`Kind::Serbia`). It's code is [Code::RS](`Code::RS`). It's constant is [Country::SERBIA](`Country::SERBIA`).
    "Serbia" (SERBIA): Serbia => RS,
    /// The country Singapore. It's kind is [Kind::Singapore](`Kind::Singapore`). It's code is [Code::SG](`Code::SG`). It's constant is [Country::SINGAPORE](`Country::SINGAPORE`).
    "Singapore" (SINGAPORE): Singapore => SG,
    /// The country Poland. It's kind is [Kind::Poland](`Kind::Poland`). It's code is [Code::PL](`Code::PL`). It's constant is [Country::POLAND](`Country::POLAND`).
    "Poland" (POLAND): Poland => PL,
    /// The country Romania. It's kind is [Kind::Romania](`Kind::Romania`). It's code is [Code::RO](`Code::RO`). It's constant is [Country::ROMANIA](`Country::ROMANIA`).
    "Romania" (ROMANIA): Romania => RO,
    /// The country Iceland. It's kind is [Kind::Iceland](`Kind::Iceland`). It's code is [Code::IS](`Code::IS`). It's constant is [Country::ICELAND](`Country::ICELAND`).
    "Iceland" (ICELAND): Iceland => IS,
    /// The country Hong Kong. It's kind is [Kind::HongKong](`Kind::HongKong`). It's code is [Code::HK](`Code::HK`). It's constant is [Country::HONG_KONG](`Country::HONG_KONG`).
    "Hong Kong" (HONG_KONG): HongKong => HK,
    /// The country Indonesia. It's kind is [Kind::Indonesia](`Kind::Indonesia`). It's code is [Code::ID](`Code::ID`). It's constant is [Country::INDONESIA](`Country::INDONESIA`).
    "Indonesia" (INDONESIA): Indonesia => ID,
    /// The country South Korea. It's kind is [Kind::SouthKorea](`Kind::SouthKorea`). It's code is [Code::KR](`Code::KR`). It's constant is [Country::SOUTH_KOREA](`Country::SOUTH_KOREA`).
    "South Korea" (SOUTH_KOREA): SouthKorea => KR,
    /// The country Croatia. It's kind is [Kind::Croatia](`Kind::Croatia`). It's code is [Code::HR](`Code::HR`). It's constant is [Country::CROATIA](`Country::CROATIA`).
    "Croatia" (CROATIA): Croatia => HR,
    /// The country Ecuador. It's kind is [Kind::Ecuador](`Kind::Ecuador`). It's code is [Code::EC](`Code::EC`). It's constant is [Country::ECUADOR](`Country::ECUADOR`).
    "Ecuador" (ECUADOR): Ecuador => EC,
    /// The country Vietnam. It's kind is [Kind::Vietnam](`Kind::Vietnam`). It's code is [Code::VN](`Code::VN`). It's constant is [Country::VIETNAM](`Country::VIETNAM`).
    "Vietnam" (VIETNAM): Vietnam => VN,
    /// The country Lithuania. It's kind is [Kind::Lithuania](`Kind::Lithuania`). It's code is [Code::LT](`Code::LT`). It's constant is [Country::LITHUANIA](`Country::LITHUANIA`).
    "Lithuania" (LITHUANIA): Lithuania => LT,
    /// The country Latvia. It's kind is [Kind::Latvia](`Kind::Latvia`). It's code is [Code::LV](`Code::LV`). It's constant is [Country::LATVIA](`Country::LATVIA`).
    "Latvia" (LATVIA): Latvia => LV,
    /// The country Bulgaria. It's kind is [Kind::Bulgaria](`Kind::Bulgaria`). It's code is [Code::BG](`Code::BG`). It's constant is [Country::BULGARIA](`Country::BULGARIA`).
    "Bulgaria" (BULGARIA): Bulgaria => BG,
    /// The country Austria. It's kind is [Kind::Austria](`Kind::Austria`). It's code is [Code::AT](`Code::AT`). It's constant is [Country::AUSTRIA](`Country::AUSTRIA`).
    "Austria" (AUSTRIA): Austria => AT,
    /// The country South Africa. It's kind is [Kind::SouthAfrica](`Kind::SouthAfrica`). It's code is [Code::ZA](`Code::ZA`). It's constant is [Country::SOUTH_AFRICA](`Country::SOUTH_AFRICA`).
    "South Africa" (SOUTH_AFRICA): SouthAfrica => ZA,
    /// The country Finland. It's kind is [Kind::Finland](`Kind::Finland`). It's code is [Code::FI](`Code::FI`). It's constant is [Country::FINLAND](`Country::FINLAND`).
    "Finland" (FINLAND): Finland => FI,
    /// The country Slovenia. It's kind is [Kind::Slovenia](`Kind::Slovenia`). It's code is [Code::SI](`Code::SI`). It's constant is [Country::SLOVENIA](`Country::SLOVENIA`).
    "Slovenia" (SLOVENIA): Slovenia => SI,
    /// The country Bosnia and Herzegovina. It's kind is [Kind::BosniaAndHerzegovina](`Kind::BosniaAndHerzegovina`). It's code is [Code::BA](`Code::BA`). It's constant is [Country::BOSNIA_AND_HERZEGOVINA](`Country::BOSNIA_AND_HERZEGOVINA`).
    "Bosnia and Herzegovina" (BOSNIA_AND_HERZEGOVINA): BosniaAndHerzegovina => BA,
    /// The country New Zealand. It's kind is [Kind::NewZealand](`Kind::NewZealand`). It's code is [Code::NZ](`Code::NZ`). It's constant is [Country::NEW_ZEALAND](`Country::NEW_ZEALAND`).
    "New Zealand" (NEW_ZEALAND): NewZealand => NZ,
    /// The country Thailand. It's kind is [Kind::Thailand](`Kind::Thailand`). It's code is [Code::TH](`Code::TH`). It's constant is [Country::THAILAND](`Country::THAILAND`).
    "Thailand" (THAILAND): Thailand => TH,
    /// The country Iran. It's kind is [Kind::Iran](`Kind::Iran`). It's code is [Code::IR](`Code::IR`). It's constant is [Country::IRAN](`Country::IRAN`).
    "Iran" (IRAN): Iran => IR,
    /// The country Bangladesh. It's kind is [Kind::Bangladesh](`Kind::Bangladesh`). It's code is [Code::BD](`Code::BD`). It's constant is [Country::BANGLADESH](`Country::BANGLADESH`).
    "Bangladesh" (BANGLADESH): Bangladesh => BD,
    /// The country Paraguay. It's kind is [Kind::Paraguay](`Kind::Paraguay`). It's code is [Code::PY](`Code::PY`). It's constant is [Country::PARAGUAY](`Country::PARAGUAY`).
    "Paraguay" (PARAGUAY): Paraguay => PY,
    /// The country Colombia. It's kind is [Kind::Colombia](`Kind::Colombia`). It's code is [Code::CO](`Code::CO`). It's constant is [Country::COLOMBIA](`Country::COLOMBIA`).
    "Colombia" (COLOMBIA): Colombia => CO,
    /// The country Georgia. It's kind is [Kind::Georgia](`Kind::Georgia`). It's code is [Code::GE](`Code::GE`). It's constant is [Country::GEORGIA](`Country::GEORGIA`).
    "Georgia" (GEORGIA): Georgia => GE,
    /// The country Kenya. It's kind is [Kind::Kenya](`Kind::Kenya`). It's code is [Code::KE](`Code::KE`). It's constant is [Country::KENYA](`Country::KENYA`).
    "Kenya" (KENYA): Kenya => KE,
    /// The country Pakistan. It's kind is [Kind::Pakistan](`Kind::Pakistan`). It's code is [Code::PK](`Code::PK`). It's constant is [Country::PAKISTAN](`Country::PAKISTAN`).
    "Pakistan" (PAKISTAN): Pakistan => PK,
    /// The country Moldova. It's kind is [Kind::Moldova](`Kind::Moldova`). It's code is [Code::MD](`Code::MD`). It's constant is [Country::MOLDOVA](`Country::MOLDOVA`).
    "Moldova" (MOLDOVA): Moldova => MD,
    /// The country Estonia. It's kind is [Kind::Estonia](`Kind::Estonia`). It's code is [Code::EE](`Code::EE`). It's constant is [Country::ESTONIA](`Country::ESTONIA`).
    "Estonia" (ESTONIA): Estonia => EE,
    /// The country Mexico. It's kind is [Kind::Mexico](`Kind::Mexico`). It's code is [Code::MX](`Code::MX`). It's constant is [Country::MEXICO](`Country::MEXICO`).
    "Mexico" (MEXICO): Mexico => MX,
    /// The country Monaco. It's kind is [Kind::Monaco](`Kind::Monaco`). It's code is [Code::MC](`Code::MC`). It's constant is [Country::MONACO](`Country::MONACO`).
    "Monaco" (MONACO): Monaco => MC
}
