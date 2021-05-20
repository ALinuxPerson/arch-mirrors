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