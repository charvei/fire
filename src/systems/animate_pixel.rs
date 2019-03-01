use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Entities, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::fire::{FirePixelAnimation, ARENA_HEIGHT, RELATIVE_FIRE_HEIGHT};

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
            
            //get height of frame length, multiply by the next frame, if it goes over -> increase frame
            let y_animation_height = ARENA_HEIGHT * RELATIVE_FIRE_HEIGHT / (animation.frames as f32);
            let next_frame_height = (animation.current_frame + 1) as f32 * y_animation_height;
            if (pixel_y > next_frame_height) && (animation.current_frame < animation.frames) {
                animation.current_frame += 1;
                sprite_render.sprite_number = animation.current_frame;
                //println!("Changed to texture num:{} @ height:{}", sprite_render.sprite_number, pixel_y);
            }
        } 
    }
}