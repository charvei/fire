use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Entities, Read, ReadStorage, System, WriteStorage},
    prelude::World,
};

/**
To perform actions on entities (e.g. delete) in a system: 
    - import Entities as part of ecs::prelude crate
    - Add it to system data as in this example
    - run(... entities) { for (entity) in (&*entities) { entities.delete(entity) } }
    
not sure what &* does in rust.
*/

use crate::fire::{Pixel, ARENA_HEIGHT};

pub struct MovePixelSystem;

impl<'s> System<'s> for MovePixelSystem {
    type SystemData = (
        ReadStorage<'s, Pixel>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Entities<'s>,

    );

    fn run(&mut self, (pixels, mut locals, time, entities): Self::SystemData) {
        for (pixel, local, entity) in (&pixels, &mut locals, &*entities).join() {
            local.translate_y(50.0 * time.delta_seconds());
            
            let pixel_y = local.translation().y;
            if pixel_y >= ARENA_HEIGHT*0.5 {
                entities.delete(entity);
            }
        }
        
    }
}