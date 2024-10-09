use bevy::{color::palettes::css, prelude::*};

// Screen format
pub const RES_Y: f32 = 1080.0; // well a bit too modern

// pub const RES_X: f32 = RES_Y * 4.0 / 3.0;
pub const RES_X: f32 = RES_Y * 16.0 / 9.0;

pub const HALF_HEIGHT: f32 = RES_Y / 2.0;
pub const HALF_WIDTH: f32 = RES_X / 2.0;

pub const SCREEN_MIN: Vec2 = Vec2 {
    x: -HALF_WIDTH,
    y: -HALF_HEIGHT,
};

pub const SCREEN_MAX: Vec2 = Vec2 {
    x: HALF_WIDTH,
    y: HALF_HEIGHT,
};

pub const SCREEN_RECT: Rect = Rect {
    min: SCREEN_MIN,
    max: SCREEN_MAX,
};

pub const PLAYER_SPEED: f32 = 500.0;

pub const BLOCK_ROTATION_SPEED: f32 = 10.0;
pub const BLOCKS_SPEED: f32 = 400.0;

pub const SHOT_SPEED: f32 = 600.0;

pub const STATUS_BAR_FONT_SIZE: f32 = 50.0;

pub const SELECTOR_FONT_SIZE: f32 = 50.0;
pub const SELECTOR_TEXT_COLOR: Srgba = css::WHITE_SMOKE;
