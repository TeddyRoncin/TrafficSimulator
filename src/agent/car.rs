use std::collections::HashMap;
use crate::{gui, road};

const SPEED: f32 = 1.;
const OPTIMAL_ACCELERATION: f32 = 0.5;
const SEEING_DISTANCE: f32 = 3.;

pub struct Car {
    position: road::RoadPoint,
    speed: f32,
    target_speed: f32,
    road_information: RoadInformation,
}


struct RoadInformation {
    current_speed_limit: f32,
    incoming_speed_limits: HashMap<road::RoadPoint, f32>,
}


impl Car {
    pub fn new(position: road::RoadPoint) -> Self {
        Self {
            position,
            speed: SPEED,
            target_speed: SPEED,
            road_information: RoadInformation { current_speed_limit: SPEED, incoming_speed_limits: HashMap::new() }
        }
    }
    pub fn step(&mut self, step_size: f32, roads: &road::Roads) {
        if self.speed > self.target_speed {
            self.speed -= OPTIMAL_ACCELERATION * step_size;
            if self.speed < self.target_speed {
                self.speed = self.target_speed;
            }
        } else if self.speed < self.target_speed {
            self.speed += OPTIMAL_ACCELERATION * step_size;
            if self.speed > self.target_speed {
                self.speed = self.target_speed;
            }
        }
        self.position.move_by(self.speed * step_size, roads);
        for sign in roads.get_signs(&self.position, SEEING_DISTANCE) {
            if self.road_information.incoming_speed_limits.contains_key(&sign.position) {
                // Skip the sign if we passed it already.
                continue;
            }
            match sign.sign_type {
                road::SignType::SpeedLimit => {
                    self.road_information.incoming_speed_limits.insert(sign.position, sign.value);
                },
                road::SignType::EndSpeedLimit => {
                    self.road_information.incoming_speed_limits.insert(sign.position, SPEED);
                }
            }
        }
        self.target_speed = self.road_information.current_speed_limit;
        // println!("{}", self.road_information.incoming_speed_limits.len());
        for (speed_limit_position, speed) in self.road_information.incoming_speed_limits.iter() {
            // Update the current speed limit information
            if roads.get_distance(speed_limit_position, &self.position) < roads.get_distance(&self.position, speed_limit_position) {
                self.road_information.current_speed_limit = *speed;
                self.target_speed = self.target_speed.min(self.road_information.current_speed_limit);
            }
            // Look at how much distance it would take to accelerate / decelerate
            let acceleration_distance = (self.speed.powi(2) - speed.powi(2)).abs() / (2. * OPTIMAL_ACCELERATION);
            if acceleration_distance > roads.get_distance(&self.position, speed_limit_position) {
                self.target_speed = self.target_speed.min(*speed);
            }
        }
        // Remove speed limits not used anymore
        self.road_information.incoming_speed_limits.retain(|road_point, _speed| { roads.get_distance(&self.position, road_point) < roads.get_distance(road_point, &self.position) });
    }

    pub fn render(&self, window: &gui::Window, roads: &road::Roads, draw_speed: bool) {
        window.draw_car(roads.get_position_xy(&self.position), if draw_speed { self.speed } else { -1. });
    }
}
