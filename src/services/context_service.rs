use crate::models::context::Context;
use crate::models::schema::Schema;
use crate::database::node::Node; // Import Node
use crate::database::edge::Edge; // Import Edge
use std::collections::HashMap;

pub struct ContextService {
    pub schema: Schema,
}

impl ContextService {
    pub fn new() -> Self {
        ContextService {
            schema: Schema::new(),
        }
    }

    pub fn add_context(&mut self, context: Context) {
        self.schema.add_context(context);
    }

    pub fn get_context(&self, id: u32) -> Option<&Context> {
        self.schema.get_context(id)
    }

    pub fn add_node_to_context(&mut self, context_id: u32, node_id: u32, label: &str) {
        if let Some(context) = self.schema.get_context(context_id) {
            let node = Node::new(node_id, label);
            self.schema.add_node(node);
        }
    }

    pub fn add_edge_to_context(&mut self, context_id: u32, from: u32, to: u32, label: &str) {
        if self.schema.get_context(context_id).is_some() {
            let edge = Edge::new(from, to, label);
            self.schema.add_edge(edge);
        }
    }
}
