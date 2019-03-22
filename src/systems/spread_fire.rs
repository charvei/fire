use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Entities, Read, ReadStorage, System, WriteStorage},
    prelude::World,
    renderer::SpriteRender,
};
use rand::prelude::*;
use crate::fire::{Pixel, PixelGrid, ARENA_HEIGHT, ARENA_WIDTH, FirePixelAnimation};

pub struct SpreadFireSystem;

impl<'s> System<'s> for SpreadFireSystem {
    type SystemData = (
        WriteStorage<'s, PixelGrid>,
        WriteStorage<'s, Pixel>,
        WriteStorage<'s, FirePixelAnimation>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
    );

    fn run(&mut self, (mut grid, mut pixels, mut anims, locals, mut sprite_renders): Self::SystemData) {
        for pixel_grid in (&mut grid).join() {
            //only one grid
            let rows = pixel_grid.grid.len();
            for i in 0..35   {
                for j in 0..35 {
                    if i > 0 /*&& i as u32 == pixel_grid.current_row*/ {
                        if rand::random() {
                            if anims.get_mut(pixel_grid.grid[35-i][35-j]).unwrap().current_frame < 35 {
                            //println!("this entities pixel's x,y: {}", anims.get_mut(pixel_grid.grid[i][j]).unwrap().current_frame);
                            
                            anims.get_mut(pixel_grid.grid[35-i][35-j]).unwrap().current_frame = anims.get_mut(pixel_grid.grid[35-i-1][35-j]).unwrap().current_frame+1;
                            sprite_renders.get_mut(pixel_grid.grid[35-i][35-j]).unwrap().sprite_number = anims.get_mut(pixel_grid.grid[35-i][35-j]).unwrap().current_frame;
                            }
                        }
                        
                    }
                }
            }
            if (pixel_grid.current_row < 35) {
                //println!("pixelgrid:current_row: {}", pixel_grid.current_row)
                pixel_grid.current_row += 1;    
            } else {
                pixel_grid.current_row = 0;
            }
            
        }    
    }

}