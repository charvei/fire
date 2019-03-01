use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Entities, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::fire::{FirePixelAnimation, ARENA_HEIGHT};

pub struct AnimatePixelSystem;

impl<'s> System<'s> for AnimatePixelSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, FirePixelAnimation>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut sprite_renders, mut animations, locals): Self::SystemData) {
        for (sprite_render, animation, local) in (&mut sprite_renders, &mut animations, &locals).join() {
          let pixel_y = local.translation().y;
          if pixel_y >= ARENA_HEIGHT * 0.25 {
              animation.current_frame = 1;
              sprite_render.sprite_number = 1;
          }  
        } 
    }
}