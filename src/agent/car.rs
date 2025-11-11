use std::collections::HashMap;
use crate::{gui, road::{self, RoadPoint}};

const SPEED: f32 = 80. / 3.6;
const OPTIMAL_ACCELERATION: f32 = 0.14 * 9.81; // 0.14g, source : https://www.jsheld.com/insights/articles/a-naturalistic-study-of-vehicle-acceleration-and-deceleration-at-an-intersection
const SEEING_DISTANCE: f32 = 100.;

pub struct Car {
    position: road::RoadPoint,
    speed: f32,
    target_speed: f32,
    road_information: RoadInformation,
    planned_trip: road::path::Path,
    is_back: bool,
}


struct RoadInformation {
    current_speed_limit: f32,
    incoming_speed_limits: HashMap<road::RoadPoint, f32>,
}


impl Car {
    pub fn new(position: road::RoadPoint) -> Self {
        Self {
            position,
            speed: 50. / 3.6,
            target_speed: 50. / 3.6,
            road_information: RoadInformation { current_speed_limit: SPEED, incoming_speed_limits: HashMap::new() },
            planned_trip: { road::path::Path::new() },
            is_back: false,
        }
    }
    pub fn update(&mut self, step_size: f32, roads: &road::Roads) {
        self.step(step_size, roads);
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
        self.target_speed = f32::INFINITY;
        for (speed_limit_position, speed) in self.road_information.incoming_speed_limits.iter() {
            // Update the current speed limit information
            if roads.get_distance(speed_limit_position, &self.position) <= roads.get_distance(&self.position, speed_limit_position) {
                self.road_information.current_speed_limit = *speed;
                self.target_speed = self.target_speed.min(self.road_information.current_speed_limit);
            }
            // Look at how much distance it would take to accelerate / decelerate
            let acceleration_distance = (self.speed.powi(2) - speed.powi(2)).abs() / (2. * OPTIMAL_ACCELERATION);
            if acceleration_distance > roads.get_distance(&self.position, speed_limit_position) {
                self.target_speed = self.target_speed.min(*speed);
            }
        }
        self.target_speed = self.target_speed.min(self.road_information.current_speed_limit);
        // Remove speed limits not used anymore
        self.road_information.incoming_speed_limits.retain(|road_point, _speed| { roads.get_distance(&self.position, road_point) < roads.get_distance(road_point, &self.position) });
        self.check_path(roads);
    }

    pub fn render(&self, window: &gui::Window, roads: &road::Roads, draw_speed: bool) {
        window.draw_car(roads.get_position_xy(&self.position), if draw_speed { self.speed } else { -1. });
    }

    fn step(&mut self, step_size: f32, roads: &road::Roads) {
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
        self.planned_trip.move_by(&mut self.position, self.speed * step_size, roads);
    }

    fn check_path(&mut self, roads: &road::Roads) {
        if self.planned_trip.total_distance(roads) < 100. || self.planned_trip.distance_left(&self.position, roads) < 100. {
            let start = road::RoadPoint::new(road::RoadSegmentIdx(0), 1.);
            let end = road::RoadPoint::new(road::RoadSegmentIdx(23), 1.);
            if self.is_back {
                self.planned_trip.append(road::path::pathfinding::pathfind(&end, &start, roads).expect("Could not find a way to go back to start"));
            } else {
                self.planned_trip.append(road::path::pathfinding::pathfind(&start, &end, roads).expect("Could not find a way to go to the top-right corner"));
            }
            self.is_back = !self.is_back;
        }
        if self.planned_trip.total_distance(roads) > 100. {
            self.planned_trip.forget_before(&self.position, roads);
        }
    }
}
