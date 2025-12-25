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
pub struct SimpleAttributeGraphResult {
    pub map: HashMap<AttributeNode, f64>,
}

impl SimpleAttributeGraphResult {
    pub fn new() -> Self {
        SimpleAttributeGraphResult {
            map: HashMap::new(),
        }
    }

    pub fn get_attribute_mut(&mut self, node: AttributeNode) -> &mut f64 {
        self.map.entry(node).or_insert(0.0)
    }

    pub fn get_attribute(&self, node: AttributeNode) -> f64 {
        *self.map.get(&node).unwrap_or(&0.0)
    }

    pub fn get_attribute_value(&self, node: AttributeNode) -> f64 {
        let mut temp = 0.0;

        for pa in node.get_parents() {
            temp += self.get_attribute(pa);
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
        self.get_attribute(node)
    }

    fn get_attribute_merge(&self, nodes: &[AttributeNode]) -> f64 {
        let mut temp = 0.0;
        for node in nodes.iter() {
            temp += self.get_attribute(*node);
        }
        temp
    }
}

#[derive(Clone)]
pub struct SimpleAttributeGraph2 {
    pub nodes: SimpleAttributeGraphResult,
    pub fixed: HashMap<AttributeNode, f64>,
    pub edges: Vec<Edge>,
    pub characters: Vec<CharacterStatus>,
}

impl AttributeGraph for SimpleAttributeGraph2 {
    type EdgeHandle = usize;
    type ResultType = SimpleAttributeGraphResult;

    fn set_value_to_internal(&mut self, node: AttributeNode, key: &str, value: f64) {
        self.fixed.insert(node, value);
    }

    fn set_value_by_internal(&mut self, node: AttributeNode, key: &str, value: f64) {
        *self.nodes.get_attribute_mut(node) += value;
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
            nodes: SimpleAttributeGraphResult::new(),
            fixed: HashMap::new(),
            edges: Vec::new(),
            characters,
        }
    }

    fn get_characters(&self) -> &Vec<CharacterStatus> {
        &self.characters
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
            *nodes_new.get_attribute_mut(edge.to) += value;
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
            for edge in edge_static.iter() {
                solve_edge(edge, &result, &mut temp, -1.0);
            }
            result = temp.clone();
            for edge in edge_static.iter() {
                solve_edge(edge, &result, &mut temp, 1.0);
            }
            result = temp.clone();
        }

        for (node, value) in self.fixed.iter() {
            *result.get_attribute_mut(*node) = *value;
        }

        result
    }
}