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

pub const SELECTOR_FONT_SIZE: f32 = 30.0;
pub const SELECTOR_TEXT_COLOR: Srgba = css::WHITE_SMOKE;
pub const SELECTOR_WHEEL_COLOR: Srgba = css::LIGHT_SKY_BLUE;
pub const SELECTOR_SELECT_COLOR: Srgba = css::BLUE;
pub const SELECTOR_COLOR: Srgba = css::STEEL_BLUE;
pub const SELECTOR_SELECTOR_COLOR: Srgba = css::ALICE_BLUE;

pub const SELECTOR_RADIUS: f32 = 150.0;
pub const SELECTOR_RADIUS_ICON: f32 = 100.0;
pub const SELECTOR_SIZE: u32 = SELECTOR_RADIUS as u32 * 2;
pub const SELECTOR_BOTTOM: u32 = 40;

pub const SPRITE_SHEET: &str = "sprites/Sprite-sheet.png";

pub const HID_HEIGHT: u32 = 120;
pub const HID_HEIGHT_PX: Val = Val::Px(HID_HEIGHT as f32);
pub const HID_BORDER_PX: Val = Val::Px(5.0);
pub const HID_WIDTH_PX: Val = Val::Px(120.0);

pub const HID_BACKGROUND_COLOR: Srgba = css::SKY_BLUE;
pub const HID_BORDER_COLOR: Srgba = css::BLUE;

#[derive(Component)]
pub struct CustomCamera;
