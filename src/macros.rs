macro_rules! countries {
    ($($(#[$docs:meta])* $name:literal ($snake_case:ident): $kind:ident => $code:ident),+) => {
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

        #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub enum Code {
             $($(#[$docs])* $code),+,

            /// Any unsupported country code.
            Other(String)
        }
    };
}

countries! {
    /// Australia.
    "Australia" (AUSTRALIA): Australia => AU,
    "Germany" (GERMANY): Germany => DE
}