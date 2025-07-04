pub mod game_assets;
pub mod background;

use crate::utils::ParsingError;

pub trait ToCPPBackend {
    fn to_dir(&self, path: &String) -> Result<(), ParsingError>;
}
