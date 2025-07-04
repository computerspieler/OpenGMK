use gm8exe::asset::Background;

use crate::{
    cpp_backend::ToCPPBackend, replace_and_amend, utils::{save_image, ParsingError, ToParsingError}
};

impl ToCPPBackend for Background {
    fn to_dir(&self, path: &String) -> Result<(), ParsingError> {
        let img_path = format!("{}.bmp", self.name);
        
        match &self.data {
        None => {}
        Some(pix) => 
            save_image(path.clone() + "/" + &img_path,
                pix,
                self.width,
                self.height
            ).convert()?
        }
        /*
        if let Some(tileset) = &self.settings.tiles {
            replace_and_amend!(path,
                "static/background_tileset.cpp.snippet" => "/backgrounds.cpp",
                PATH:           str img_path,
                PRELOAD:        val self.settings.preload,
                SMOOTH_EDGE:    val self.settings.smooth_edges,
                TRANSPARENT:    val self.settings.transparent,
                OFFSET_X:       val tileset.offset.x,
                OFFSET_Y:       val tileset.offset.y,
                SEPARATION_X:   val tileset.separation.x,
                SEPARATION_Y:   val tileset.separation.y,
                SIZE_X:         val tileset.size.width,
                SIZE_Y:         val tileset.size.height
            );
        } else {
            replace_and_amend!(path,
                "static/background.cpp.snippet" => "/backgrounds.cpp",
                PATH:           str img_path,
                PRELOAD:        val self.settings.preload,
                SMOOTH_EDGE:    val self.settings.smooth_edges,
                TRANSPARENT:    val self.settings.transparent
            );
        }
        */
        Ok(())
    }
}