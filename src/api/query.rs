use crate::database::node::Node;
use crate::database::edge::Edge;
use std::collections::HashMap;

pub struct Database {
    nodes: HashMap<u32, Node>,
    edges: Vec<Edge>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn get_node(&self, id: u32) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_edge(&self, from: u32, to: u32) -> Option<&Edge> {
        self.edges.iter().find(|&e| e.from == from && e.to == to)
    }

    pub fn query_nodes_by_property(&self, key: &str, value: &str) -> Vec<&Node> {
        self.nodes.values()
            .filter(|&node| node.get_property(key).map_or(false, |v| v == value))
            .collect()
    }

    pub fn query_edges_by_property(&self, key: &str, value: &str) -> Vec<&Edge> {
        self.edges.iter()
            .filter(|&edge| edge.get_property(key).map_or(false, |v| v == value))
            .collect()
    }
}
