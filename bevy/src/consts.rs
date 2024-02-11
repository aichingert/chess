use bevy::render::color::Color;

pub const WIDTH: f32 = SQUARE_SIZE * 8.;
pub const HEIGHT: f32 = SQUARE_SIZE * 8.;

pub const SQUARE_SIZE: f32 = 75.;
pub const OFFSET: f32 = (-(8. / 2. * SQUARE_SIZE)) + SQUARE_SIZE / 2.;

pub const COLORS: [Color; 2] = [
    Color::rgb(181.0 / 255.0, 136.0 / 255.0, 99.0 / 255.0),
    Color::rgb(240.0 / 255.0, 217.0 / 255.0, 181.0 / 255.0)
];
