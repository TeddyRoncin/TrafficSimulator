use std::f32;

use macroquad::{prelude::*, rand::gen_range};

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
struct Car {
    road_segment: usize,
    progression: f32,
    speed: f32,
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
                Sign {sign_type: SignType::SpeedLimit, value: 4., position: 0.4},
                Sign {sign_type: SignType::SpeedLimit, value: 0.3, position: 0.7},
            ],
            length,
            visual_keypoints,
        }
    }
}

impl Car {
    fn new(road_segment: usize, progression: f32) -> Self {
        Self { road_segment, progression, speed: SPEED }
    }
    fn step(&mut self, step_size: f32, roads: &Roads) {
        let road_segment = &roads.segments[self.road_segment];
        let previous_progression: f32 = self.progression;
        self.progression += self.speed * step_size;
        println!("progression = {}, length = {}, moved = {}", self.progression, road_segment.length, self.speed * step_size);
        if self.progression > road_segment.length {
            self.progression = 0.;
        }
        for sign in &road_segment.signs {
            if previous_progression < sign.position && self.progression > sign.position {
                match sign.sign_type {
                    SignType::SpeedLimit => {
                        self.speed = sign.value;
                    },
                    SignType::EndSpeedLimit => {
                        print!("Problem! EndSpeedLimit not known");
                    }
                }
            }
        }
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
        for segment in &roads.segments {
            let mut start = (roads.nodes[segment.from].x, roads.nodes[segment.from].y);
            for i in 0..segment.visual_keypoints.len() {
                let end = (segment.visual_keypoints[i].x, segment.visual_keypoints[i].y);
                draw_line(self.x_to_pixel(start.0), self.y_to_pixel(start.1), self.x_to_pixel(end.0), self.y_to_pixel(end.1), 3., RED);
                start = end;
            }
            let end = (roads.nodes[segment.to].x, roads.nodes[segment.to].y);
            draw_line(self.x_to_pixel(start.0), self.y_to_pixel(start.1), self.x_to_pixel(end.0), self.y_to_pixel(end.1), 3., RED);
        }
    }
    fn draw_cars(&self, cars: &Vec<Car>, roads: &Roads) {
        for car in cars {
            let mut start: (f32, f32, f32) = (-1., -1., -1.);
            let mut end: (f32, f32, f32) = (-1., -1., -1.);
            for (i, visual_keypoint) in roads.segments[car.road_segment].visual_keypoints.iter().enumerate() {
                end = (visual_keypoint.position, visual_keypoint.x, visual_keypoint.y);
                println!("visual_keypoint pos = {}", visual_keypoint.position);
                if visual_keypoint.position > car.progression {
                    if i == 0 {
                        start = (0., roads.nodes[roads.segments[car.road_segment].from].x, roads.nodes[roads.segments[car.road_segment].from].y);
                    } else {
                        start = (roads.segments[car.road_segment].visual_keypoints[i - 1].position, roads.segments[car.road_segment].visual_keypoints[i - 1].x, roads.segments[car.road_segment].visual_keypoints[i - 1].y);
                    }
                    break;
                }
            }
            if end == (-1., -1., -1.) {
                start = end;
                end = (roads.segments[car.road_segment].length, roads.nodes[roads.segments[car.road_segment].to].x, roads.nodes[roads.segments[car.road_segment].to].y);
            }
            let diff_x = end.1 - start.1;
            let diff_y = end.2 - start.2;

            let progression_on_line = (car.progression - start.0) / (end.0 - start.0);
            let car_position = (start.1 + diff_x * progression_on_line, start.2 + diff_y * progression_on_line);
            draw_rectangle(self.x_to_pixel(car_position.0) - 5., self.y_to_pixel(car_position.1) - 5., 10., 10., BLUE);
        }
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
    for _ in 0..10 {
        let road_idx: usize = gen_range(0, sd.roads.segments.len());
        let progression: f32 = gen_range(0., 1.);
        sd.cars.push(Car::new(road_idx, progression));
    }
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
