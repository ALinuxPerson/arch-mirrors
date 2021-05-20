macro_rules! countries {
    ($($(#[$docs:meta])* $name:literal ($snake_case:ident): $kind:ident => $code:ident),+) => {
        #[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub enum Kind {
            $($(#[$docs])* $kind),+,

            /// Any unsupported country.
            Other(String)
        }
    };
}

countries! {
    /// Australia.
    "Australia" (AUSTRALIA): Australia => AU,
    "Germany" (GERMANY): Germany => DE
}