use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Entities, Read, ReadStorage, System, WriteStorage},
    prelude::World,
    renderer::SpriteRender,
};

use crate::fire::{Pixel, ARENA_HEIGHT, ARENA_WIDTH, FirePixelAnimation};

pub struct SpreadFireSystem;

impl<'s> System<'s> for SpreadFireSystem {
    type SystemData = (
        ReadStorage<'s, Pixel>,
        WriteStorage<'s, FirePixelAnimation>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (pixels, mut animations, locals, mut sprite_renders): Self::SystemData) {
        for (sprite_render, animation, local, pixel) in (&mut sprite_renders, &mut animations, &locals, &pixels).join() {
            if (local.translation().y as u32 != 0) /*&& (animation.current_frame < 34)*/ {
                if animation.current_frame == 34 {
                    animation.current_frame = 0;
                }
                let pixel_y = local.translation().y as u32;
                
                animation.current_frame += 1;
                sprite_render.sprite_number = animation.current_frame;
                

                /*Want to find a way to read all the pixels so that i can find the pixel below the current one, probably involves creating a grid component*/
                
                // for (lower_pixel, lower_animation) in (&pixels, &mut animations).join() {
                //     if local.translation().y as u32 == pixel_y -1 {
                //         animation.current_frame = lower_animation.current_frame;    
                //     }
                // }
                
                
            }

        }
    }
}