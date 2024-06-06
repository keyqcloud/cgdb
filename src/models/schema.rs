use crate::models::context::Context;
use crate::database::{node::Node, edge::Edge};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Schema {
    pub contexts: HashMap<u32, Context>,
    pub nodes: HashMap<u32, Node>,
    pub edges: Vec<Edge>,
}

impl Schema {
    pub fn new() -> Self {
        Schema {
            contexts: HashMap::new(),
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_context(&mut self, context: Context) {
        self.contexts.insert(context.id, context);
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn get_context(&self, id: u32) -> Option<&Context> {
        self.contexts.get(&id)
    }

    pub fn get_node(&self, id: u32) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_edge(&self, from: u32, to: u32) -> Option<&Edge> {
        self.edges.iter().find(|&e| e.from == from && e.to == to)
    }
}
