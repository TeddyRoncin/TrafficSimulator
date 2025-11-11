pub mod path;

use std::{f32, fmt::Display, hash::Hash};
use crate::gui;
use crate::generate_custom_vec;

generate_custom_vec!(RoadNode, RoadNodeIdx);
generate_custom_vec!(RoadSegment, RoadSegmentIdx);
generate_custom_vec!(RoadVisualKeypoint, RoadVisualKeypointIdx);


struct RoadSegment {
    from: RoadNodeIdx,
    to: RoadNodeIdx,
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

#[derive(PartialEq)]
struct RoadNode {
    x: f32,
    y: f32,
    road_segments: Vec<RoadSegmentIdx>,
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct RoadPoint {
    road_segment: RoadSegmentIdx,
    position: f32,
}

impl Roads {
    pub fn new() -> Self {
        let mut instance = Self { nodes: Vec::new(), segments: Vec::new() };
        instance.init_roads();

        instance
    }

    pub fn get_position_xy(&self, position: &RoadPoint) -> (f32, f32) {
        // (position of the keypoint on the segment, position x of the keypoint, position y of the keypoint)
        let mut start: (f32, f32, f32) = (0., self.nodes[self.segments[position.road_segment].from].x, self.nodes[self.segments[position.road_segment].from].y);
        let mut end: (f32, f32, f32) = (self.segments[position.road_segment].length, self.nodes[self.segments[position.road_segment].to].x, self.nodes[self.segments[position.road_segment].to].y);
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
        return self.segments[point_1.road_segment].length - point_1.position + point_2.position + f32::INFINITY;
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
        self.init_roads_mesh();
    }

    fn init_roads_circle(&mut self) {
        self.nodes.push(RoadNode { x: 20., y: 0., road_segments: Vec::new() });
        let mut visual_keypoints: Vec<RoadVisualKeypoint> = Vec::new();
        for i in 1..100 {
            let angle = i as f32 * 2. * f32::consts::PI / 100.;
            visual_keypoints.push(RoadVisualKeypoint { position: i as f32 / 100., x: f32::cos(angle) * 20., y: f32::sin(angle) * 20. });
        }
        let [node1, node2] = self.nodes.get_disjoint_mut([0, 0]).unwrap();
        self.segments.push(RoadSegment::new(RoadSegmentIdx(0), RoadNodeIdx(0), RoadNodeIdx(0), node1, node2, visual_keypoints));
    }

    fn init_roads_mesh(&mut self) {
        // Nodes, from left to right and bottom to top
        self.nodes.push(RoadNode { x: 0., y: 0., road_segments: Vec::new() });
        self.nodes.push(RoadNode { x: 30., y: 0., road_segments: Vec::new() });
        self.nodes.push(RoadNode { x: 60., y: 0., road_segments: Vec::new() });
        self.nodes.push(RoadNode { x: 90., y: 0., road_segments: Vec::new() });

        self.nodes.push(RoadNode { x: 0., y: 30., road_segments: Vec::new() });
        self.nodes.push(RoadNode { x: 30., y: 30., road_segments: Vec::new() });
        self.nodes.push(RoadNode { x: 60., y: 30., road_segments: Vec::new() });
        self.nodes.push(RoadNode { x: 90., y: 30., road_segments: Vec::new() });

        self.nodes.push(RoadNode { x: 0., y: 60., road_segments: Vec::new() });
        self.nodes.push(RoadNode { x: 30., y: 60., road_segments: Vec::new() });
        self.nodes.push(RoadNode { x: 60., y: 60., road_segments: Vec::new() });
        self.nodes.push(RoadNode { x: 90., y: 60., road_segments: Vec::new() });

        self.nodes.push(RoadNode { x: 0., y: 90., road_segments: Vec::new() });
        self.nodes.push(RoadNode { x: 30., y: 90., road_segments: Vec::new() });
        self.nodes.push(RoadNode { x: 60., y: 90., road_segments: Vec::new() });
        self.nodes.push(RoadNode { x: 90., y: 90., road_segments: Vec::new() });

        let mut index = 0;
        let mut create_road_segment = |from: usize, to: usize| {
            let [node1, node2] = self.nodes.get_disjoint_mut([from, to]).unwrap();

            self.segments.push(RoadSegment::new(RoadSegmentIdx(index), RoadNodeIdx(from), RoadNodeIdx(to), node1, node2, Vec::new()));
            self.segments.push(RoadSegment::new(RoadSegmentIdx(index + 1), RoadNodeIdx(to), RoadNodeIdx(from), node2, node1, Vec::new()));
            index += 2;
        };
        // Horizontal segments, from left to right and bottom to top
        create_road_segment(0, 1);
        create_road_segment(1, 2);
        create_road_segment(2, 3);

        create_road_segment(4, 5);
        create_road_segment(5, 6);
        create_road_segment(6, 7);

        create_road_segment(8, 9);
        create_road_segment(9, 10);
        create_road_segment(10, 11);
        
        create_road_segment(12, 13);
        create_road_segment(13, 14);
        create_road_segment(14, 15);

        // Vertical segments, from bottom to top and left to right
        create_road_segment(0, 4);
        create_road_segment(4, 8);
        create_road_segment(8, 12);

        create_road_segment(1, 5);
        create_road_segment(5, 9);
        create_road_segment(9, 13);

        create_road_segment(2, 6);
        create_road_segment(6, 10);
        create_road_segment(10, 14);

        create_road_segment(3, 7);
        create_road_segment(7, 11);
        create_road_segment(11, 15);
    }
}

impl RoadSegment {
    fn new(index: RoadSegmentIdx, from: RoadNodeIdx, to: RoadNodeIdx, from_node: &mut RoadNode, to_node: &RoadNode, mut visual_keypoints: Vec<RoadVisualKeypoint>) -> Self {
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
        from_node.road_segments.push(index);
        Self {
            from,
            to,
            signs: vec![
                // Sign { sign_type: SignType::SpeedLimit, value: 50. / 3.6, position: RoadPoint::new(index, 40.) },
                // Sign { sign_type: SignType::SpeedLimit, value: 30. / 3.6, position: RoadPoint::new(index, 85.) },
                // Sign { sign_type: SignType::EndSpeedLimit, value: 0., position: RoadPoint::new(index, 94.) },
            ],
            length,
            visual_keypoints,
        }
    }
}

impl Hash for RoadNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl Eq for RoadNode {}

impl RoadPoint {
    pub fn new(road_segment: RoadSegmentIdx, position: f32) -> Self {
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

impl Display for RoadPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.road_segment, self.position)
    }
}