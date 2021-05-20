pub trait StrExt<'a> {
    fn into_option(self) -> Option<&'a str>;
}

impl<'a> StrExt<'a> for &'a str {
    fn into_option(self) -> Option<&'a str> {
        match self.is_empty() {
            true => None,
            false => Some(self),
        }
    }
}
