use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::app::{Ship};

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ship>,
        Read<'s, InputHandler<StringBindings>>,
    );
    
    fn run(&mut self, (mut transform, mut ship, input): Self::SystemData) {
      
       for (ship, transform) in (&ship, &mut transform).join() {
            let horizontal = input.axis_value("horizontal").unwrap_or(0.0);
            let vertical = input.axis_value("vertical").unwrap_or(0.0);
            
            let shoot = input.action_is_down("shoot").unwrap_or(false);
            
            transform.move_right(horizontal);
            transform.move_up(vertical);
        }
    }
}