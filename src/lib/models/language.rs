use crate::models::language::Language::{En, Es};

pub const ES_LANGUAGE: &'static str = "ES";
pub const EN_LANGUAGE: &'static str = "EN";

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Es,
    En,
}

impl Language {
    pub fn all() -> Vec<Language> {
        let mut result = Vec::new();
        result.push(Language::Es);
        result.push(Language::En);
        result
    }
}

impl Into<&str> for Language {
    fn into(self) -> &'static str {
        match self {
            Es => { ES_LANGUAGE }
            En => { EN_LANGUAGE }
        }
    }
}

impl TryFrom<&str> for Language {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ES_LANGUAGE => { Ok(Es) }
            EN_LANGUAGE => { Ok(En) }
            _ => { Err(()) }
        }
    }
}