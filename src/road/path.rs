pub mod pathfinding;
// pub use pathfinding;

use crate::{generate_custom_vec, road::{RoadNodeIdx, RoadPoint, RoadSegmentIdx, Roads}};

generate_custom_vec!(RoadNodeIdx, NodePathIdx);

#[derive(Debug)]
pub struct Path {
    road_nodes: Vec<RoadNodeIdx>,
}

impl Path {
    pub fn new() -> Self { Self { road_nodes: Vec::new() }}

    pub fn move_by(&mut self, position: &mut RoadPoint, mut amount: f32, roads: &Roads) {
        while amount > 0. {
            let segment_length = roads.segments[position.road_segment].length;
            let amount_on_segment = amount.min(segment_length - position.position);
            position.position += amount_on_segment;
            amount -= amount_on_segment;
            if amount > 0. {
                let current_node_path_index = self.get_node_path_idx(roads.segments[position.road_segment].to).expect("Node not found in path");
                position.road_segment = self.get_segment_following_node_path(current_node_path_index, roads);
                position.position = 0.;
            }
        }
    }

    pub fn total_distance(&self, roads: &Roads) -> f32 {
        if self.road_nodes.len() < 2 { // Having only 1 RoadNode makes no sense
            return 0.;
        }
        let mut distance = 0.;
        for node_path_idx in 0..(self.road_nodes.len() - 1) {
            let segment_idx = self.get_segment_following_node_path(NodePathIdx(node_path_idx), roads);
            distance += roads.segments[*segment_idx].length;
        }

        distance
    }

    pub fn distance_left(&self, node_point: &RoadPoint, roads: &Roads) -> f32 {
        let mut distance_left = roads.segments[node_point.road_segment].length - node_point.position;
        for node_path_idx in self.get_node_path_idx(roads.segments[node_point.road_segment].to).expect("Node not found in path").into()..(self.road_nodes.len() - 1) {
            let segment_idx = self.get_segment_following_node_path(NodePathIdx(node_path_idx), roads);
            distance_left += roads.segments[segment_idx].length;
        }

        distance_left
    }

    pub fn append(&mut self, mut path: Vec<RoadNodeIdx>) {
        self.road_nodes.append(&mut path);
    }

    pub fn forget_before(&mut self, position: &RoadPoint, roads: &Roads) {
        let delete_up_to_idx = self.get_node_path_idx(roads.segments[position.road_segment].from);
        if delete_up_to_idx.is_none() {
            return;
        }
        for _ in 0..delete_up_to_idx.unwrap().into() {
            self.road_nodes.remove(0);
        }
    }

    fn get_node_path_idx(&self, node: RoadNodeIdx) -> Option<NodePathIdx> {
        self.road_nodes.iter().position(|elt| elt == &node).and_then(|i| Some(NodePathIdx(i)))
    }

    fn get_segment_following_node_path(&self, node: NodePathIdx, roads: &Roads) -> RoadSegmentIdx {
        for segment in &roads.nodes[self.road_nodes[node]].road_segments {
            if roads.segments[*segment].to == self.road_nodes[node + NodePathIdx(1)] {
                return *segment;
            }
        }
        panic!("Could not find a RoadSegment going from RoadNodeIdx({}) to RoadNodeIdx({})", node, node + NodePathIdx(1));
    }
}