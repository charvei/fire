extern crate amethyst;

mod fire;
mod systems;    //import systems

use crate::fire::Fire;

use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig, DrawFlat2D, Event, Pipeline,
    RenderBundle, Stage, VirtualKeyCode
};
use amethyst::utils::application_root_dir;
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::{
    ui::{DrawUi, UiBundle}
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default()); //so we can see errors, warnings and debug messages

    /*Load configuration from config file, sets things like screen size and title*/
    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    /*Rendering code -- dunno what it does yet*/
    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0) //Changes color of background
                .with_pass(DrawFlat2D::new())
                .with_pass(DrawUi::new())
        );

    /*Input stuff*/
    let binding_path = format!(
        "{}/resources/bindings_config.ron",
        application_root_dir()
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;
    /*End of input stuff*/

    /*Add systems to game*/
    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(systems::MovePixelSystem, "move_pixel_system", &[])
        //.with(systems::PaddleSystem, "paddle_system", &["input_system"])
        ;

    let mut game = Application::new("./", Fire, game_data)?;

    /*Run the game*/
    game.run();

    Ok(())
}