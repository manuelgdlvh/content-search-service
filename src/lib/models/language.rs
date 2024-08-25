use crate::models::language::Language::{En, Es};

pub const ES_LANGUAGE: &str = "ES";
pub const EN_LANGUAGE: &str = "EN";

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Es,
    En,
}

impl Language {
    pub fn all() -> Vec<Language> {
        vec![Es, En]
    }
}

impl From<Language> for &str {
    fn from(value: Language) -> Self {
        match value{
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