use crate::road;
use macroquad::{color::*, prelude::rand};

const CAR_SIZE: (f32, f32) = (28.8, 14.28);
const DEFAULT_SCALE: f32 = 10.; // px / meter
const ZOOMING_SPEED: f32 = 2.;

pub struct Window {
    zoom: f32,
    center: (f32, f32),
    car_texture: macroquad::texture::Texture2D,
}

impl Window {
    pub async fn new() -> Self {
        macroquad::window::request_new_screen_size(800., 800.);
        Self {
            zoom: 0.,
            center: (0., 0.),
            car_texture: macroquad::texture::load_texture("resources/textures/car.png").await.expect("Could not load car texture"),
        }
    }

    pub fn draw_road_segment(&self, start: (f32, f32), end: (f32, f32)) {
        let window_start = (self.x_to_pixel(start.0), self.y_to_pixel(start.1));
        let window_end = (self.x_to_pixel(end.0), self.y_to_pixel(end.1));
        if (window_start.0 < 0. && window_end.0 < 0.)
            || (window_start.0 >= macroquad::window::screen_width() && window_end.0 >= macroquad::window::screen_width())
            || (window_start.1 < 0. && window_end.1 < 0.)
            || (window_start.1 >= macroquad::window::screen_height() && window_end.1 >= macroquad::window::screen_height()) {
            return;
        }
        macroquad::shapes::draw_line(window_start.0, window_start.1, window_end.0, window_end.1, 3., RED);
    }

    pub fn draw_sign(&self, (x, y): (f32, f32), sign_type: &road::SignType) {
        let window_position = (self.x_to_pixel(x), self.y_to_pixel(y));
        if window_position.0 < -5.
            || window_position.0 - 5. >= macroquad::window::screen_width()
            || window_position.1 < -5.
            || window_position.1 - 5. >= macroquad::window::screen_height() {
            return;
        }
        let color = match sign_type {
            road::SignType::SpeedLimit => { ORANGE }
            road::SignType::EndSpeedLimit => { BLUE }
        };
        macroquad::shapes::draw_rectangle(self.x_to_pixel(x) - 5., self.y_to_pixel(y) - 5., 10., 10., color);
    }

    pub fn draw_car(&self, (x, y): (f32, f32), speed: f32) {
        macroquad::texture::draw_texture_ex(
            &self.car_texture,
            self.x_to_pixel(x) - CAR_SIZE.0 / 2.,
            self.y_to_pixel(y) - CAR_SIZE.1 / 2.,
            WHITE,
            macroquad::texture::DrawTextureParams { dest_size: Some(macroquad::math::Vec2::new(CAR_SIZE.0, CAR_SIZE.1)), rotation: rand::rand() as f32, ..Default::default() }
        );
        if speed != -1. {
            macroquad::text::draw_text(&format!("Speed = {}km/h", speed * 3.6), 5., 100., 30., WHITE);
        }
    }

    pub fn update(&mut self) {
        let frame_zoom = macroquad::input::mouse_wheel().1;
        if frame_zoom != 0. {
            let mouse_position = macroquad::input::mouse_position();
            self.center.0 -= (self.center.0 - self.px_to_x(mouse_position.0)) * (1. - 1. / ZOOMING_SPEED.powf(frame_zoom));
            self.center.1 -= (self.center.1 - self.px_to_y(mouse_position.1)) * (1. - 1. / ZOOMING_SPEED.powf(frame_zoom));
            self.zoom += frame_zoom;
        }
        if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Left) {
            self.center.0 += macroquad::input::mouse_delta_position().x / 2. * macroquad::window::screen_width() / self.scale();
            self.center.1 -= macroquad::input::mouse_delta_position().y / 2. * macroquad::window::screen_height() / self.scale();
        }
    }

    fn x_to_pixel(&self, x: f32) -> f32 { macroquad::window::screen_width() / 2. + (x - self.center.0) * self.scale() }
    fn y_to_pixel(&self, y: f32) -> f32 { macroquad::window::screen_height() / 2. - (y - self.center.1) * self.scale() }
    fn px_to_x(&self, px: f32) -> f32 { (px - macroquad::window::screen_width() / 2.) / self.scale() + self.center.0 }
    fn px_to_y(&self, px: f32) -> f32 { (macroquad::window::screen_height() / 2. - px) / self.scale() + self.center.1 }
    fn scale(&self) -> f32 { DEFAULT_SCALE * ZOOMING_SPEED.powf(self.zoom) }
}