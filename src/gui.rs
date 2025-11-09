use crate::road;
use macroquad::{color::*};

const WINDOW_MARGINS: f32 = 10.;
const CAR_SIZE: (f32, f32) = (28.8, 14.28);

pub struct Window {
    width: f32,
    height: f32,
    car_texture: macroquad::texture::Texture2D,
}

impl Window {
    pub async fn new() -> Self {
        Self {
            width: macroquad::window::screen_width() - WINDOW_MARGINS * 2.,
            height: macroquad::window::screen_height() - WINDOW_MARGINS * 2.,
            car_texture: macroquad::texture::load_texture("resources/textures/car.png").await.expect("Could not load car texture"),
        }
    }

    pub fn draw_road_segment(&self, start: (f32, f32), end: (f32, f32)) {
        macroquad::shapes::draw_line(self.x_to_pixel(start.0), self.y_to_pixel(start.1), self.x_to_pixel(end.0), self.y_to_pixel(end.1), 3., RED);
    }

    pub fn draw_sign(&self, (x, y): (f32, f32), sign_type: &road::SignType) {
        let color = match sign_type {
            road::SignType::SpeedLimit => { ORANGE }
            road::SignType::EndSpeedLimit => { BLUE }
        };
        macroquad::shapes::draw_rectangle(self.x_to_pixel(x) - 5., self.y_to_pixel(y) - 5., 10., 10., color);
    }

    pub fn draw_car(&self, (x, y): (f32, f32), speed: f32) {
        // macroquad::shapes::draw_rectangle(self.x_to_pixel(x) - CAR_SIZE.0 / 2., self.y_to_pixel(y) - CAR_SIZE.1 / 2., CAR_SIZE.0, CAR_SIZE.1, ORANGE);
        macroquad::texture::draw_texture_ex(
            &self.car_texture,
            self.x_to_pixel(x) - CAR_SIZE.0 / 2.,
            self.y_to_pixel(y) - CAR_SIZE.1 / 2.,
            WHITE,
            macroquad::texture::DrawTextureParams { dest_size: Some(macroquad::math::Vec2::new(CAR_SIZE.0, CAR_SIZE.1)), ..Default::default() }
        );
        if speed != -1. {
            macroquad::text::draw_text(&("Speed = ".to_string() + &speed.to_string()), 5., 100., 30., WHITE);
        }
    }

    fn x_to_pixel(&self, x: f32) -> f32 { WINDOW_MARGINS + (x * self.width).round() }
    fn y_to_pixel(&self, y: f32) -> f32 { WINDOW_MARGINS + (y * self.height).round() }
}