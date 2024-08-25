use crate::models::entity::Entity::{Game, Movie, Recipe, Tv};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Entity {
    Movie,
    Tv,
    Game,
    Recipe,
}

impl TryFrom<&str> for Entity {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "MOVIE" => { Ok(Movie) }
            "TV" => { Ok(Tv) }
            "GAME" => { Ok(Game) }
            "RECIPE" => { Ok(Recipe) }
            _ => { Err(()) }
        }
    }
}