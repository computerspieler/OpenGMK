use std::{convert::From, fs};

use gm8exe::{asset::background, GameAssets};

use crate::{
    cpp_backend::ToCPPBackend,
    utils::{write_str_to_file, ParsingError, ToParsingError}
};

use crate::copy_and_replace;

impl ToCPPBackend for GameAssets {
    fn to_dir(&self, path: &String) -> Result<(), ParsingError> {
        fs::create_dir(path).convert()?;
        fs::create_dir(path.to_owned() + "/res").convert()?;

        fs::copy("static/alarm.hpp", path.to_owned() + "/alarm.hpp").convert()?;
        fs::copy("static/utils.hpp", path.to_owned() + "/utils.hpp").convert()?;
        fs::copy("static/room.hpp", path.to_owned() + "/room.hpp").convert()?;
        fs::copy("static/object.hpp", path.to_owned() + "/object.hpp").convert()?;
        fs::copy("static/sprite.hpp", path.to_owned() + "/sprite.hpp").convert()?;
        fs::copy("static/sprite.cpp", path.to_owned() + "/sprite.cpp").convert()?;
        fs::copy("static/background.hpp", path.to_owned() + "/background.hpp").convert()?;

        copy_and_replace!(path,
            "static/main.cpp" => "/main.cpp",
/*
            SCREEN_POS_X: val self.game_informations.window_position.left,
            SCREEN_POS_Y: val self.game_informations.window_position.top,
            SCREEN_WIDTH: val self.game_informations.window_position.width,
            SCREEN_HEIGHT: val self.game_informations.window_position.height,
            SCREEN_CAPTION: str self.game_informations.form_caption,

            SCREEN_RESIZABLE: val (
                if self.global_settings.windowing.allow_window_resize
                { "SDL_WINDOW_RESIZABLE" } else { "0" }
            ),
            SCREEN_BORDERLESS: val (
                if self.global_settings.windowing.dont_draw_border
                { "SDL_WINDOW_BORDERLESS" } else { "0" }
            ),
            SCREEN_FULLSCREEN: val (
                if self.global_settings.windowing.start_fullscreen
                { "SDL_WINDOW_FULLSCREEN" } else { "0" }
            ),

            CURSOR_SHOW: val (
                if self.global_settings.graphics.display_cursor
                { "1" } else { "0" }
            ),
            SCREEN_VSYNC: val (
                if self.global_settings.graphics.use_vsync
                { "1" } else { "0" }
            ),

            GAME_DEFAULT_BACKGROUND_COLOR_R: val self.game_informations.background_color.r,
            GAME_DEFAULT_BACKGROUND_COLOR_G: val self.game_informations.background_color.g,
            GAME_DEFAULT_BACKGROUND_COLOR_B: val self.game_informations.background_color.b
*/
        );
        write_str_to_file(path.clone() + "/backgrounds.cpp","
#include \"background.hpp\"

const Background backgrounds [] = {
"
        )?;
        for background in self.backgrounds.iter() {
            match background {
            None => {}
            Some(b_background) => {
                let background = b_background.as_ref();
                background.to_dir(&(path.clone() + "/res".into()))?;
            }
            }
        }

        write_str_to_file(path.clone() + "/backgrounds.cpp","};")?;

        Ok(())
    }
}
