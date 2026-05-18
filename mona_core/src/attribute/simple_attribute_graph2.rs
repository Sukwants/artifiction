use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use rand::Rng;
use crate::attribute::SimpleAttribute;
use crate::character::team_status::CharacterStatus;
use crate::common::{Element, EntryType, SkillType};
use crate::damage::SimpleDamageBuilder;

use super::attribute::{AttributeGraph, AttributeNode, EdgeFunction, EdgePriority};
use super::AttributeGraphResult;

#[derive(Clone)]
pub struct Edge {
    pub from1: AttributeNode,
    pub from2: AttributeNode,
    pub to: AttributeNode,
    pub key: String,
    pub func: EdgeFunction,
    pub priority: EdgePriority,
    pub id: usize,
}

#[derive(Clone)]
pub struct Node {
    pub values: HashMap<String, f64>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            values: HashMap::new(),
        }
    }
}

impl Node {
    pub fn new() -> Self {
        Node::default()
    }

    pub fn sum(&self) -> f64 {
        self.values.values().sum::<f64>()
    }

    pub fn set_value_by(&mut self, key: &str, value: f64) {
        *self.values.entry(String::from(key)).or_insert(0.0) += value;
    }

    pub fn set_value_to(&mut self, key: &str, value: f64) {
        *self.values.entry(String::from(key)).or_insert(0.0) = value;
    }
}

#[derive(Clone)]
pub struct SimpleAttributeGraphResult {
    pub map: HashMap<AttributeNode, Node>,
    pub characters: Vec<CharacterStatus>,
}

impl SimpleAttributeGraphResult {
    pub fn new(characters: Vec<CharacterStatus>) -> Self {
        SimpleAttributeGraphResult {
            map: HashMap::new(),
            characters,
        }
    }

    pub fn get_attribute_mut(&mut self, node: AttributeNode) -> &mut Node {
        self.map.entry(node).or_default()
    }

    pub fn get_attribute_value(&self, node: AttributeNode) -> f64 {
        let mut temp = 0.0;

        for pa in node.get_parents() {
            temp += self.map.get(&pa).map(Node::sum).unwrap_or(0.0);
        }

        temp
    }
}

impl AttributeGraphResult for SimpleAttributeGraphResult {
    type ResultType = f64;
    
    fn get_attribute_value(&self, node: AttributeNode) -> f64 {
        self.get_attribute_value(node)
    }

    fn get_attribute(&self, node: AttributeNode) -> f64 {
        self.get_attribute_value(node)
    }

    fn get_attribute_merge(&self, nodes: &[AttributeNode]) -> f64 {
        let mut temp = 0.0;
        for node in nodes.iter() {
            temp += self.get_attribute_value(*node);
        }
        temp
    }

    fn get_characters(&self) -> &Vec<CharacterStatus> {
        &self.characters
    }
}

#[derive(Clone)]
pub struct SimpleAttributeGraph2 {
    pub nodes: SimpleAttributeGraphResult,
    pub edges: Vec<Edge>,
}

impl AttributeGraph for SimpleAttributeGraph2 {
    type EdgeHandle = usize;
    type ResultType = SimpleAttributeGraphResult;

    fn set_value_to_internal(&mut self, node: AttributeNode, key: &str, value: f64) {
        self.nodes.get_attribute_mut(node).set_value_to(key, value);
    }

    fn set_value_by_internal(&mut self, node: AttributeNode, key: &str, value: f64) {
        self.nodes.get_attribute_mut(node).set_value_by(key, value);
    }

    fn add_edge(
        &mut self,
        from1: AttributeNode,
        from2: AttributeNode,
        to: AttributeNode,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    ) -> Self::EdgeHandle {
        let mut rng = rand::thread_rng();
        let id: usize = rng.gen();
        let edge = Edge {
            from1,
            from2,
            to,
            key: String::from(key),
            func,
            priority,
            id,
        };

        self.edges.push(edge);
        id
    }

    fn remove_edge(&mut self, handle: Self::EdgeHandle) {
        let mut index = 0;
        for (i, edge) in self.edges.iter().enumerate() {
            if edge.id == handle {
                index = i;
                break;
            }
        }

        self.edges.remove(index);
    }

    fn new_with_characters(characters: Vec<CharacterStatus>) -> Self {
        SimpleAttributeGraph2 {
            nodes: SimpleAttributeGraphResult::new(characters),
            edges: Vec::new(),
        }
    }

    fn get_characters(&self) -> &Vec<CharacterStatus> {
        &self.nodes.characters
    }

    fn solve(&self) -> Self::ResultType {
        self.solve()
    }
}

impl SimpleAttributeGraph2 {
    pub fn solve(&self) -> SimpleAttributeGraphResult {
        let mut result = self.nodes.clone();
        let mut temp = self.nodes.clone();

        let solve_edge = |
            edge: &Edge,
            nodes_old: &SimpleAttributeGraphResult,
            nodes_new: &mut SimpleAttributeGraphResult,
            c: f64,
        | {
            let from1_value = nodes_old.get_attribute_value(edge.from1);
            let from2_value = nodes_old.get_attribute_value(edge.from2);
            let value = (edge.func)(from1_value, from2_value) * c;
            nodes_new.get_attribute_mut(edge.to).set_value_by(&edge.key, value);
        };

        let mut edge_lists = BTreeMap::new();
        let mut edge_static = Vec::new();
        for edge in self.edges.iter() {
            if edge.priority == EdgePriority::Static {
                edge_static.push(edge);
                continue;
            }
            edge_lists.entry(edge.priority as usize).or_insert(Vec::new()).push(edge);
        }

        for edge in edge_static.iter() {
            solve_edge(edge, &result, &mut temp, 1.0);
        }
        result = temp.clone();

        for list in edge_lists.values() {
            for edge in list.iter() {
                solve_edge(edge, &result, &mut temp, 1.0);
            }
            for edge in edge_static.iter() {
                solve_edge(edge, &result, &mut temp, -1.0);
            }
            result = temp.clone();
            for edge in edge_static.iter() {
                solve_edge(edge, &result, &mut temp, 1.0);
            }
            result = temp.clone();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attribute::complicated_attribute_graph::ComplicatedAttributeGraph;
    use crate::attribute::AttributeName;

    #[test]
    fn set_value_to_parity_with_complicated_graph() {
        let node = AttributeNode::new_panel(0, AttributeName::ATKPercentage);
        let mut simple = SimpleAttributeGraph2::new_with_characters(Vec::new());
        let mut complicated = ComplicatedAttributeGraph::new_with_characters(Vec::new());

        simple.set_value_by_internal(node, "a", 0.2);
        simple.set_value_by_internal(node, "b", 0.3);
        simple.set_value_to_internal(node, "a", 0.5);

        complicated.set_value_by_internal(node, "a", 0.2);
        complicated.set_value_by_internal(node, "b", 0.3);
        complicated.set_value_to_internal(node, "a", 0.5);

        assert_eq!(simple.nodes.get_attribute_value(node), complicated.nodes.get_attribute_value(node));
    }
}
