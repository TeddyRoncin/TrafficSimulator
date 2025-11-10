use std::{f32, hash::Hash};
use crate::gui;

struct RoadSegment {
    from: usize,
    to: usize,
    length: f32,
    signs: Vec<Sign>,
    visual_keypoints: Vec<RoadVisualKeypoint>,
}

pub struct Roads {
    segments: Vec<RoadSegment>,
    nodes: Vec<RoadNode>,
}

struct RoadVisualKeypoint {
    position: f32,
    x: f32,
    y: f32,
}

struct RoadNode {
    x: f32,
    y: f32,
}

pub struct Sign {
    pub sign_type: SignType,
    pub value: f32,
    pub position: RoadPoint,
}

pub enum SignType {
    SpeedLimit,
    EndSpeedLimit,
}

#[derive(PartialEq, Clone, Copy)]
pub struct RoadPoint {
    road_segment: usize,
    position: f32,
}

impl Roads {
    pub fn new() -> Self {
        let mut instance = Self { nodes: Vec::new(), segments: Vec::new() };
        instance.init_roads();

        instance
    }

    pub fn get_position_xy(&self, position: &RoadPoint) -> (f32, f32) {
        let mut start: (f32, f32, f32) = (-1., -1., -1.);
        let mut end: (f32, f32, f32) = (-1., -1., -1.);
        for (i, visual_keypoint) in self.segments[position.road_segment].visual_keypoints.iter().enumerate() {
            end = (visual_keypoint.position, visual_keypoint.x, visual_keypoint.y);
            if visual_keypoint.position > position.position {
                if i == 0 {
                    start = (0., self.nodes[self.segments[position.road_segment].from].x, self.nodes[self.segments[position.road_segment].from].y);
                } else {
                    start = (self.segments[position.road_segment].visual_keypoints[i - 1].position, self.segments[position.road_segment].visual_keypoints[i - 1].x, self.segments[position.road_segment].visual_keypoints[i - 1].y);
                }
                break;
            }
        }
        if end == (-1., -1., -1.) {
            start = end;
            end = (self.segments[position.road_segment].length, self.nodes[self.segments[position.road_segment].to].x, self.nodes[self.segments[position.road_segment].to].y);
        }
        let diff_x = end.1 - start.1;
        let diff_y = end.2 - start.2;
        let progression_on_line = (position.position - start.0) / (end.0 - start.0);

        (start.1 + diff_x * progression_on_line, start.2 + diff_y * progression_on_line)
    }

    pub fn get_signs(&self, position: &RoadPoint, seeing_distance: f32) -> Vec<&Sign>{
        let mut signs = Vec::new();
        for sign in &self.segments[position.road_segment].signs {
            if position.position < sign.position.position && sign.position.position < position.position + seeing_distance {
                signs.push(sign);
            }
        }

        signs
    }

    pub fn get_distance(&self, point_1: &RoadPoint, point_2: &RoadPoint) -> f32 {
        if point_1.road_segment != point_2.road_segment {
            return f32::INFINITY;
        }
        if point_1.position <= point_2.position {
            return point_2.position - point_1.position;
        }
        return self.segments[point_1.road_segment].length - point_1.position + point_2.position;
    }

    pub fn render(&self, window: &gui::Window) {
        for segment in &self.segments {
            let mut start = (self.nodes[segment.from].x, self.nodes[segment.from].y);
            for visual_keypoint in &segment.visual_keypoints {
                let end = (visual_keypoint.x, visual_keypoint.y);
                window.draw_road_segment(start, end);
                start = end;
            }
            window.draw_road_segment(start, (self.nodes[segment.to].x, self.nodes[segment.to].y));
            for sign in &segment.signs {
                window.draw_sign(self.get_position_xy(&sign.position), &sign.sign_type);
            }
        }
    }

    fn init_roads(&mut self) {
        self.nodes.push(RoadNode { x: 1., y: 0. });
        let mut visual_keypoints: Vec<RoadVisualKeypoint> = Vec::new();
        for i in 1..100 {
            let angle = i as f32 * 2. * f32::consts::PI / 100.;
            visual_keypoints.push(RoadVisualKeypoint { position: i as f32 / 100., x: f32::cos(angle), y: f32::sin(angle) });
        }
        self.segments.push(RoadSegment::new(0, 0, 0, &self.nodes[0], &self.nodes[0], visual_keypoints));
    }
}

impl RoadSegment {
    fn new(index: usize, from: usize, to: usize, from_node: &RoadNode, to_node: &RoadNode, mut visual_keypoints: Vec<RoadVisualKeypoint>) -> Self {
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
                Sign { sign_type: SignType::SpeedLimit, value: 0.5, position: RoadPoint::new(index, 0.4) },
                Sign { sign_type: SignType::SpeedLimit, value: 0.1, position: RoadPoint::new(index, 1.4) },
                Sign { sign_type: SignType::EndSpeedLimit, value: 0., position: RoadPoint::new(index, 1.6) },
            ],
            length,
            visual_keypoints,
        }
    }
}

impl RoadPoint {
    pub fn new(road_segment: usize, position: f32) -> Self {
        Self { road_segment, position }
    }

    pub fn move_by(&mut self, amount: f32, roads: &Roads) {
        self.position += amount;
        if self.position > roads.segments[self.road_segment].length {
            self.position = 0.;
        }
    }
}

impl Hash for RoadPoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.road_segment.hash(state);
        self.position.to_bits().hash(state);
    }
}

impl Eq for RoadPoint {}