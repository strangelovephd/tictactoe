use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
    }
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct TicTacToe;

impl SimpleState for TicTacToe {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        init_camera(world);
        init_squares(world);
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

pub enum State {
    Empty,
    X,
    O,
}

pub struct Square {
    state: State,
    width: f32,
    height: f32,
    loc_x: usize,
    loc_y: usize,
}

impl Square {
    pub fn new(x: usize, y: usize) -> Square {
        Square {
            state: State::Empty,
            width: 33.3,
            height: 33.3,
            loc_x: x,
            loc_y: y,
        }
    }
}

impl Component for Square {
    type Storage = DenseVecStorage<Self>;
}

fn init_squares(world: &mut World) {
    for x in 0..3 {
        for y in 0..3 {
            let mut transform = Transform::default();
            transform.set_xyz(x as f32 * 33.3, y as f32 * 33.3, 0.0);
            world.create_entity()
                .with(Square::new(x, y))
                .with(transform)
                .build();
        }
    }
}