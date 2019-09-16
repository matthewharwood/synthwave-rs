// SHIP
pub const SHIP_HEIGHT: f32 = 16.0;
pub const SHIP_WIDTH: f32 = 4.0;

pub struct Ship {
    pub width: f32,
    pub height: f32,
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
        .with(Ship::new())
        .with(sprite_render.clone())
        .with(position)
        .build();
}