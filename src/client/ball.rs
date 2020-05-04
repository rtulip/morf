use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::client_game_model::{ARENA_HEIGHT, ARENA_WIDTH};

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

/// Initialises one ball in the middle-ish of the arena.
pub fn initialise_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    // Assign the sprite for the ball
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1, // ball is the second sprite on the sprite sheet
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}
