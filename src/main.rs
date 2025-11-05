use std::{collections::HashMap, f32, hash::Hash};

use macroquad::{prelude::*};

struct SimulationData {
    roads: Roads,
    cars: Vec<Car>,
}

struct RoadNode {
    x: f32,
    y: f32,
}

struct RoadSegment {
    from: usize,
    to: usize,
    length: f32,
    signs: Vec<Sign>,
    visual_keypoints: Vec<RoadVisualKeypoint>,
}

struct Roads {
    segments: Vec<RoadSegment>,
    nodes: Vec<RoadNode>,
}

struct RoadVisualKeypoint {
    position: f32,
    x: f32,
    y: f32,
}

const SPEED: f32 = 1.;
const OPTIMAL_ACCELERATION: f32 = 0.5;
const SEEING_DISTANCE: f32 = 3.;

struct Car {
    position: RoadPoint,
    speed: f32,
    target_speed: f32,
    road_information: RoadInformation,
}

#[derive(PartialEq)]
struct RoadPoint {
    road_segment: usize,
    position: f32,
}

struct RoadInformation {
    current_speed_limit: f32,
    incoming_speed_limits: HashMap<RoadPoint, f32>,
}

const MARGINS: f32 = 10.;

struct Window {
    width: f32,
    height: f32,
}

struct Sign {
    sign_type: SignType,
    value: f32,
    position: f32,
}

enum SignType {
    SpeedLimit,
    EndSpeedLimit,
}

impl Roads {
    fn new() -> Self {
        Self { nodes: Vec::new(), segments: Vec::new() }
    }
    fn get_position_xy(&self, road_segment: usize, position: f32) -> (f32, f32) {
        let mut start: (f32, f32, f32) = (-1., -1., -1.);
        let mut end: (f32, f32, f32) = (-1., -1., -1.);
        for (i, visual_keypoint) in self.segments[road_segment].visual_keypoints.iter().enumerate() {
            end = (visual_keypoint.position, visual_keypoint.x, visual_keypoint.y);
            if visual_keypoint.position > position {
                if i == 0 {
                    start = (0., self.nodes[self.segments[road_segment].from].x, self.nodes[self.segments[road_segment].from].y);
                } else {
                    start = (self.segments[road_segment].visual_keypoints[i - 1].position, self.segments[road_segment].visual_keypoints[i - 1].x, self.segments[road_segment].visual_keypoints[i - 1].y);
                }
                break;
            }
        }
        if end == (-1., -1., -1.) {
            start = end;
            end = (self.segments[road_segment].length, self.nodes[self.segments[road_segment].to].x, self.nodes[self.segments[road_segment].to].y);
        }
        let diff_x = end.1 - start.1;
        let diff_y = end.2 - start.2;
        let progression_on_line = (position - start.0) / (end.0 - start.0);

        (start.1 + diff_x * progression_on_line, start.2 + diff_y * progression_on_line)
    }
}

impl RoadSegment {
    fn new(from: usize, to: usize, from_node: &RoadNode, to_node: &RoadNode, mut visual_keypoints: Vec<RoadVisualKeypoint>) -> Self {
        let mut curr_x = from_node.x;
        let mut curr_y = from_node.y;
        let mut length: f32 = 0.;
        for kp in &mut visual_keypoints {
            length += ((kp.x - curr_x).powi(2) + (kp.y - curr_y).powi(2)).sqrt();
            kp.position = length;
            curr_x = kp.x;
            curr_y = kp.y;
        }
        length += ((to_node.x - curr_x).powi(2) + (to_node.y - curr_y).powi(2)).sqrt();
        Self {
            from,
            to,
            signs: vec![
                Sign {sign_type: SignType::SpeedLimit, value: 0.5, position: 0.4},
                Sign {sign_type: SignType::SpeedLimit, value: 0.1, position: 1.4},
                Sign { sign_type: SignType::EndSpeedLimit, value: 0., position: 1.6 },
            ],
            length,
            visual_keypoints,
        }
    }
}

impl RoadPoint {
    fn new(road_segment: usize, position: f32) -> Self {
        Self { road_segment, position }
    }
}

impl Hash for RoadPoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.road_segment.hash(state);
        self.position.to_bits().hash(state);
    }
}

impl Eq for RoadPoint {}

impl Car {
    fn new(road_segment: usize, progression: f32) -> Self {
        Self { position: RoadPoint { road_segment, position: progression }, speed: SPEED, target_speed: SPEED, road_information: RoadInformation { current_speed_limit: SPEED, incoming_speed_limits: HashMap::new() } }
    }
    fn step(&mut self, step_size: f32, roads: &Roads) {
        let road_segment = &roads.segments[self.position.road_segment];
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
        self.position.position += self.speed * step_size;
        if self.position.position > road_segment.length {
            self.position.position = 0.;
        }
        for sign in &road_segment.signs {
            let sign_position = RoadPoint::new(self.position.road_segment, sign.position);
            if self.position.position < sign.position - SEEING_DISTANCE || self.position.position > sign.position || self.road_information.incoming_speed_limits.contains_key(&sign_position) {
                // Skip the sign if we are too far, we already know about it, or we passed it already.
                continue;
            }
            match sign.sign_type {
                SignType::SpeedLimit => {
                    self.road_information.incoming_speed_limits.insert(sign_position, sign.value);
                },
                SignType::EndSpeedLimit => {
                    self.road_information.incoming_speed_limits.insert(sign_position, SPEED);
                }
            }
        }
        self.target_speed = self.road_information.current_speed_limit;
        // println!("{}", self.road_information.incoming_speed_limits.len());
        for (speed_limit_position, speed) in self.road_information.incoming_speed_limits.iter() {
            // Update the current speed limit information
            if speed_limit_position.position < self.position.position {
                self.road_information.current_speed_limit = *speed;
                self.target_speed = self.target_speed.min(self.road_information.current_speed_limit);
            }
            // Look at how much distance it would take to accelerate / decelerate
            let acceleration_distance = (self.speed.powi(2) - speed.powi(2)).abs() / (2. * OPTIMAL_ACCELERATION);
            if acceleration_distance > (speed_limit_position.position - self.position.position) {
                self.target_speed = self.target_speed.min(*speed);
            }
        }
        // Remove speed limits not used anymore
        self.road_information.incoming_speed_limits.retain(|road_point, _speed| { road_point.position > self.position.position });
        // println!("{}", self.road_information.incoming_speed_limits.len());
        
    }

    fn _request_target_speed(&mut self, target_speed: f32) {
        self.target_speed = self.target_speed.min(target_speed).min(self.road_information.current_speed_limit);
    }
}

impl Window {
    fn x_to_pixel(&self, x: f32) -> f32 { MARGINS + (x * self.width).round() }
    fn y_to_pixel(&self, y: f32) -> f32 { MARGINS + (y * self.height).round() }
    fn new() -> Self {
        Self {
            width: screen_width() - MARGINS * 2.,
            height: screen_height() - MARGINS * 2.,
        }
    }
    fn draw_roads(&self, roads: &Roads) {
        for (i_segment, segment) in roads.segments.iter().enumerate() {
            let mut start = (roads.nodes[segment.from].x, roads.nodes[segment.from].y);
            for i in 0..segment.visual_keypoints.len() {
                let end = (segment.visual_keypoints[i].x, segment.visual_keypoints[i].y);
                draw_line(self.x_to_pixel(start.0), self.y_to_pixel(start.1), self.x_to_pixel(end.0), self.y_to_pixel(end.1), 3., RED);
                start = end;
            }
            let end = (roads.nodes[segment.to].x, roads.nodes[segment.to].y);
            draw_line(self.x_to_pixel(start.0), self.y_to_pixel(start.1), self.x_to_pixel(end.0), self.y_to_pixel(end.1), 3., RED);
            for sign in &segment.signs {
                let (x, y) = roads.get_position_xy(i_segment, sign.position);
                draw_rectangle(self.x_to_pixel(x) - 5., self.y_to_pixel(y) - 5., 10., 10., ORANGE);
            }
        }
    }
    fn draw_cars(&self, cars: &Vec<Car>, roads: &Roads) {
        for car in cars {
            let (x, y) = roads.get_position_xy(car.position.road_segment, car.position.position);
            draw_rectangle(self.x_to_pixel(x) - 5., self.y_to_pixel(y) - 5., 10., 10., BLUE);
        }
        draw_text(&("Speed = ".to_string() + &cars[0].speed.to_string()), 5., 100., 30., WHITE);
    }
}

fn init_roads(sd: &mut SimulationData) {
    sd.roads.nodes.push(RoadNode { x: 1. / 3. + 0.5, y: 0.5 });
    let mut visual_keypoints: Vec<RoadVisualKeypoint> = Vec::new();
    for i in 1..100 {
        let angle = i as f32 * 2. * f32::consts::PI / 100.;
        visual_keypoints.push(RoadVisualKeypoint { position: i as f32 / 100., x: f32::cos(angle) / 3. + 0.5, y: f32::sin(angle) / 3. + 0.5 });
    }
    sd.roads.segments.push(RoadSegment::new(0, 0, &sd.roads.nodes[0], &sd.roads.nodes[0], visual_keypoints));
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut sd: SimulationData = SimulationData { roads: Roads::new(), cars: Vec::new() };
    init_roads(&mut sd);
    let window: Window = Window::new();
    sd.cars.push(Car::new(0, 0.));
    // for _ in 0..10 {
    //     let road_idx: usize = gen_range(0, sd.roads.segments.len());
    //     let progression: f32 = gen_range(0., 1.);
    //     sd.cars.push(Car::new(road_idx, progression));
    // }
    loop {
        let step_size = get_frame_time();
        let roads = &sd.roads;
        for car in sd.cars.iter_mut() {
            car.step(step_size, roads);
        }
        clear_background(BLACK);
        window.draw_roads(&sd.roads);
        window.draw_cars(&sd.cars, &sd.roads);
        draw_fps();
        next_frame().await;
    }
}
