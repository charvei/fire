use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Entity};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
    Flipped,
};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

/*Good idea to define constants in the 'game' code*/
//Arena
pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
pub const RELATIVE_FIRE_HEIGHT: f32 = 0.5;

pub struct Fire;

impl SimpleState for Fire {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics
        //let sprite_sheet_handle = load_sprite_sheet(world);
        let sprite_sheet_handle_pixel = load_sprite_sheet_pixel(world);

        //initialise_paddles(world, sprite_sheet_handle_pixel);
        //world.register::<Pixel>();
        //initialise_pixel(world, sprite_sheet_handle_pixel);
        initialise_generator(world, sprite_sheet_handle_pixel);
        initialise_camera(world);
    }

}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

pub struct Pixel {
    pub width: f32,
    pub height: f32,
}

impl Pixel {
    pub fn new() -> Pixel {
        Pixel {
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Pixel {
    type Storage = DenseVecStorage<Self>;
}

//Initialise a pixel in center of screen
fn initialise_pixel(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    transform.set_xyz(ARENA_WIDTH * 0.5, 0.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Pixel::new())
        .with(transform)
        .with(FirePixelAnimation::new(0 as usize, 34 as usize))
        .build();

}

fn load_sprite_sheet_pixel(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/fire2.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    //note that sprite sheet has new colour every +25 pixels on x axis
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/fire_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

pub struct FirePixelAnimation {
    pub start_sprite_index: usize,
    pub frames: usize,
    pub current_frame: usize,
}

//Simple animation component
impl FirePixelAnimation {
    pub fn new(start_sprite_index: usize, frames: usize) -> FirePixelAnimation {
        FirePixelAnimation {
            start_sprite_index: start_sprite_index,
            frames: frames,
            current_frame: 0,
        }
    }
}

impl Component for FirePixelAnimation{
    type Storage = DenseVecStorage<Self>;
}

pub struct PixelGenerator {
    pub frequency: f32,
    pub sprite_sheet: SpriteSheetHandle,
    pub counter: u32,
}

impl PixelGenerator {
    pub fn new(frequency: f32, sprite_sheet: SpriteSheetHandle) -> PixelGenerator {
        PixelGenerator {
            frequency: frequency,
            sprite_sheet: sprite_sheet,
            counter: 0,
        }
    }
}

impl Component for PixelGenerator {
    type Storage = DenseVecStorage<Self>;
}

//Initialise a pixel in center of screen
fn initialise_generator(world: &mut World, sprite_sheet: SpriteSheetHandle) {

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(PixelGenerator::new(1.0, sprite_sheet))
        .build();

}