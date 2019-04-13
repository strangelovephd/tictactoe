extern crate amethyst;
mod tictactoe;
use crate::tictactoe::TicTacToe;

use std::path::Path;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, DrawFlat2D, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};


fn main() -> amethyst::Result<()> {

    let path = Path::new("resources/display_config.ron");    
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0,0.0,0.0,1.0], 1.0)
                .with_pass(DrawFlat2D::new()),   
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
            .with_sprite_sheet_processor()   
        )?;

    let mut game = Application::new("./", TicTacToe, game_data)?;

    game.run();

    Ok(())
}
