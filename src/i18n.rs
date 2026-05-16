use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    En,
    Fr,
    Ar,
}

impl Lang {
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Fr => "fr",
            Lang::Ar => "ar",
        }
    }
    pub fn dir(self) -> &'static str {
        match self {
            Lang::Ar => "rtl",
            _ => "ltr",
        }
    }
    pub fn locale(self) -> &'static str {
        match self {
            Lang::En => "en_US",
            Lang::Fr => "fr_FR",
            Lang::Ar => "ar_DZ",
        }
    }
}

impl FromStr for Lang {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "fr" => Ok(Lang::Fr),
            "ar" => Ok(Lang::Ar),
            "en" | "" => Ok(Lang::En),
            _ => Err(()),
        }
    }
}

/// Trilingual text bundle.
#[derive(Debug, Clone, Copy)]
pub struct T {
    pub en: &'static str,
    pub fr: &'static str,
    pub ar: &'static str,
}

impl T {
    pub const fn new(en: &'static str, fr: &'static str, ar: &'static str) -> Self {
        Self { en, fr, ar }
    }
    pub fn get(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::En => self.en,
            Lang::Fr => self.fr,
            Lang::Ar => self.ar,
        }
    }
}
