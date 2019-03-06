use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Entities, Read, ReadStorage, System, WriteStorage, LazyUpdate, Builder},
    prelude::World,
    renderer::SpriteRender as SpriteRendererImport,
};

use crate::fire::{PixelGenerator, Pixel, FirePixelAnimation, ARENA_WIDTH};

pub struct GeneratePixelSystem;

impl<'s> System<'s> for GeneratePixelSystem {
    type SystemData = (
        WriteStorage<'s, PixelGenerator>,
        Read<'s, Time>,
        Entities<'s>,
        Read<'s, LazyUpdate>
    );

    fn run (&mut self, (mut generators, time, entities, lazy): Self::SystemData) {
        for (generator) in (&mut generators).join() {
            generator.counter += 1;
            println!("{}", generator.counter);
            if generator.counter > 5 {
                for i in 0..ARENA_WIDTH as u32 {
                    if i % 15 == 0 {
                        let mut transform = Transform::default();
                        transform.set_xyz(i as f32, 0.0, 0.0);

                        let sprite_render = SpriteRendererImport {
                            sprite_sheet: generator.sprite_sheet.clone(),
                            sprite_number: 0,
                        };
                        let my_entity = lazy
                            .create_entity(&entities)
                            .with(sprite_render)
      //                      .with(Pixel::new())
                            .with(transform)
                            .with(FirePixelAnimation::new(0 as usize, 34 as usize))
                            .build();
                    }
                }
                generator.counter = 0;
            }
        }
    }
}