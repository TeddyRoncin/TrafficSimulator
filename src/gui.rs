use macroquad::prelude::*;
use crate::road;

const WINDOW_MARGINS: f32 = 10.;

pub struct Window {
    width: f32,
    height: f32,
}

impl Window {
    pub fn new() -> Self {
        Self {
            width: screen_width() - WINDOW_MARGINS * 2.,
            height: screen_height() - WINDOW_MARGINS * 2.,
        }
    }

    pub fn draw_road_segment(&self, start: (f32, f32), end: (f32, f32)) {
        draw_line(self.x_to_pixel(start.0), self.y_to_pixel(start.1), self.x_to_pixel(end.0), self.y_to_pixel(end.1), 3., RED);
    }

    pub fn draw_sign(&self, (x, y): (f32, f32), sign_type: &road::SignType) {
        let color = match sign_type {
            road::SignType::SpeedLimit => { ORANGE }
            road::SignType::EndSpeedLimit => { BLUE }
        };
        draw_rectangle(self.x_to_pixel(x) - 5., self.y_to_pixel(y) - 5., 10., 10., color);
    }

    pub fn draw_car(&self, (x, y): (f32, f32), speed: f32) {
        draw_rectangle(self.x_to_pixel(x) - 5., self.y_to_pixel(y) - 5., 10., 10., BLUE);
        if speed != -1. {
            draw_text(&("Speed = ".to_string() + &speed.to_string()), 5., 100., 30., WHITE);
        }
    }

    fn x_to_pixel(&self, x: f32) -> f32 { WINDOW_MARGINS + (x * self.width).round() }
    fn y_to_pixel(&self, y: f32) -> f32 { WINDOW_MARGINS + (y * self.height).round() }
}