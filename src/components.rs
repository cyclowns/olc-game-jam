use specs::*;
use specs_derive::*;
use ggez_goodies::{Point2, Vector2};

// ////////////////////
// Components
// ////////////////////

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub Point2);

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Motion {
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

#[derive(Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Player;

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Player>();
}