use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct App;

pub const ARENA_HEIGHT: f32 = 500.0;
pub const ARENA_WIDTH: f32 = 500.0;

// SHIP
pub const SHIP_HEIGHT: f32 = 16.0;
pub const SHIP_WIDTH: f32 = 4.0;

pub struct Ship {
    pub width: f32,
    pub height: f32,
}


impl SimpleState for App {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
      let world = data.world;
      let sprite_sheet_handle = load_sprite_sheet(world);
      world.register::<Ship>();
      init_ship(world, sprite_sheet_handle);
      initialise_camera(world);
      println!("hello world");
    }
}


fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}


impl Ship {
    fn new() -> Ship {
        Ship {
            height: SHIP_HEIGHT,
            width: SHIP_WIDTH,
        }
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

fn init_ship(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
  
    let mut position = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    let x = (ARENA_WIDTH / 2.0) - (SHIP_WIDTH * 0.5);
    position.set_translation_xyz(x, y, 0.0);
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(position)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}