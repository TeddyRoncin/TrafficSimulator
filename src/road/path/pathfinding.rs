use std::{cmp::Ordering, collections::HashMap};
use sortedlist_rs::SortedList;
use crate::road::{RoadNode, RoadNodeIdx, RoadPoint, Roads};


#[derive(Clone, Copy, Debug)]
struct Node {
    road_node: RoadNodeIdx,
    distance_from_start: f32,
    distance_to_end: f32,
    parent_road_node: RoadNodeIdx,
    opened: bool,
}

#[derive(PartialEq)]
struct OpenedNode {
    road_node: RoadNodeIdx,
    cost: f32,
}

impl Node {
    fn cost(&self) -> f32 { self.distance_from_start + self.distance_to_end }
}

impl Eq for OpenedNode {}

impl PartialOrd for OpenedNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for OpenedNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost < other.cost {
            Ordering::Less
        } else if self.cost > other.cost {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl OpenedNode {
    fn from(node: &Node) -> Self { Self { road_node: node.road_node, cost: node.cost() } }
}

fn get_distance(point1: &RoadNode, point2: &RoadNode) -> f32 { ((point1.x - point2.x).powi(2) + (point1.y - point2.y).powi(2)).sqrt() }

pub fn pathfind(start: &RoadPoint, end: &RoadPoint, roads: &Roads) -> Option<Vec<RoadNodeIdx>> {
    let start_road_node_index = roads.segments[start.road_segment].to;
    let end_road_node_index = roads.segments[end.road_segment].from;
    let mut nodes: HashMap<RoadNodeIdx, Node> = HashMap::new();
    let mut opened_nodes: SortedList<OpenedNode> = SortedList::new();
    
    // Insert start node
    let start_node = Node {
        road_node: start_road_node_index,
        distance_from_start: 0.,
        distance_to_end: get_distance(&roads.nodes[start_road_node_index], &roads.nodes[start_road_node_index]),
        parent_road_node: RoadNodeIdx(0),
        opened: true,
    };
    opened_nodes.insert(OpenedNode { road_node: start_road_node_index, cost: start_node.cost() });
    nodes.insert(start_road_node_index, start_node);

    let mut found = false;

    while !found && opened_nodes.len() != 0 {
        // Get the best opened node
        let mut node = *nodes.get(&opened_nodes.remove(0).road_node).unwrap();
        node.opened = false;
        nodes.insert(node.road_node, node);
        if node.road_node == end_road_node_index {
            found = true;
            continue;
        }
        // List all the neighbours of that node
        for road_segment_idx in &roads.nodes[node.road_node].road_segments {
            let segment = &roads.segments[*road_segment_idx];
            let neighbour_road_node_index = segment.to;
            let neighbour_road_node = &roads.nodes[neighbour_road_node_index];
            let old_neighbour_node = nodes.get(&segment.to);
            let old_neighbour_node_opened = old_neighbour_node.and_then(|node| Some(node.opened));
            if old_neighbour_node_opened == Some(false) {
                // If the neighbour is already closed, skip it
                continue;
            }
            let new_neighbour_node = Node {
                road_node: neighbour_road_node_index,
                distance_from_start: node.distance_from_start + segment.length,
                distance_to_end: get_distance(&roads.nodes[node.road_node], neighbour_road_node),
                parent_road_node: node.road_node,
                opened: true,
            };
            if old_neighbour_node_opened == None {
                // Create a new value in opened_nodes and in nodes
                opened_nodes.insert(OpenedNode::from(&new_neighbour_node));
                nodes.insert(neighbour_road_node_index, new_neighbour_node);
            } else if old_neighbour_node.unwrap().cost() > new_neighbour_node.cost() {
                // Remove the old value from opened_nodes, and create a new one. In nodes, update the value.
                opened_nodes.remove(opened_nodes.binary_search(&OpenedNode::from(&old_neighbour_node.unwrap())).unwrap());
                opened_nodes.insert(OpenedNode::from(&new_neighbour_node));
                nodes.insert(neighbour_road_node_index, new_neighbour_node);
            }
        }
    }

    if !found {
        return None;
    }

    // We build the path backwards, and then reverse it
    let mut reversed_path: Vec<RoadNodeIdx> = Vec::new();
    reversed_path.push(end_road_node_index);

    let mut current_node: Node = nodes[&end_road_node_index];
    while current_node.road_node != start_road_node_index {
        current_node = nodes[&current_node.parent_road_node];
        reversed_path.push(current_node.road_node);
    }
    reversed_path.reverse();

    Some(reversed_path)
}