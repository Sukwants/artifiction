use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use rand::Rng;
use crate::character::team_status::CharacterStatus;
use crate::common::{Element, EntryType, SkillType};
use crate::damage::ComplicatedDamageBuilder;

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

impl Node {
    pub fn new() -> Self {
        Node {
            values: HashMap::new(),
        }
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

    pub fn composition(&self) -> HashMap<String, f64> {
        let mut temp: HashMap<String, f64> = HashMap::new();
        for (k, v) in self.values.iter() {
            *temp.entry(k.clone()).or_insert(0.0) += *v;
        }

        temp
    }
}

#[derive(Clone)]
pub struct ComplicatedAttributeGraphResult {
    pub map: HashMap<AttributeNode, Node>,
}

impl ComplicatedAttributeGraphResult {
    pub fn new() -> Self {
        ComplicatedAttributeGraphResult {
            map: HashMap::new(),
        }
    }

    pub fn get_attribute_mut(&mut self, node: AttributeNode) -> &mut Node {
        self.map.entry(node).or_insert(Node::new())
    }

    pub fn get_attribute(&self, node: AttributeNode) -> Node {
        self.map.get(&node).unwrap_or(&Node::new()).clone()
    }

    pub fn get_attribute_value(&self, node: AttributeNode) -> f64 {
        let mut temp = 0.0;

        for pa in node.get_parents() {
            temp += self.get_attribute(pa).sum();
        }

        temp
    }

    pub fn get_attribute_composition(&self, node: AttributeNode) -> EntryType {
        let mut temp = EntryType::new();

        for pa in node.get_parents() {
            temp.merge(&EntryType(self.get_attribute(pa).composition()));
        }

        temp
    }

    pub fn get_composition_merge(&self, nodes: &[AttributeNode]) -> EntryType {
        let mut temp = EntryType::new();
        for node in nodes.iter() {
            let comp = self.get_attribute_composition(*node);
            temp.merge(&comp);
        }

        temp
    }
}

impl AttributeGraphResult for ComplicatedAttributeGraphResult {
    type ResultType = EntryType;
    
    fn get_attribute_value(&self, node: AttributeNode) -> f64 {
        self.get_attribute_value(node)
    }

    fn get_attribute(&self, node: AttributeNode) -> EntryType {
        self.get_attribute_composition(node)
    }

    fn get_attribute_merge(&self, nodes: &[AttributeNode]) -> Self::ResultType {
        self.get_composition_merge(nodes)
    }
}

#[derive(Clone)]
pub struct ComplicatedAttributeGraph {
    pub nodes: ComplicatedAttributeGraphResult,
    pub fixed: HashMap<AttributeNode, (String, f64)>,
    pub edges: Vec<Edge>,
    pub characters: Vec<CharacterStatus>,
}

impl AttributeGraph for ComplicatedAttributeGraph {
    type EdgeHandle = usize;
    type ResultType = ComplicatedAttributeGraphResult;

    fn set_value_to_internal(&mut self, node: AttributeNode, key: &str, value: f64) {
        self.fixed.insert(node, (String::from(key), value));
    }

    fn set_value_by_internal(&mut self, node: AttributeNode, key: &str, value: f64) {
        (*self.nodes.get_attribute_mut(node)).set_value_by(key, value);
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
        ComplicatedAttributeGraph {
            nodes: ComplicatedAttributeGraphResult::new(),
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

impl ComplicatedAttributeGraph {
    pub fn solve(&self) -> ComplicatedAttributeGraphResult {
        let mut result = self.nodes.clone();
        let mut temp = self.nodes.clone();

        use crate::utils;
        utils::log!("solve");

        let mut edge_lists = BTreeMap::new();
        for edge in self.edges.iter() {
            edge_lists.entry(edge.priority as usize).or_insert(Vec::new()).push(edge);
        }
        for list in edge_lists.values() {
            utils::log!("list {:?}", result.get_attribute_value(AttributeNode::new_panel(0, crate::attribute::AttributeName::ATK)));
            for edge in list.iter() {
                let from1_value = result.get_attribute_value(edge.from1);
                let from2_value = result.get_attribute_value(edge.from2);
                let value = (edge.func)(from1_value, from2_value);
                utils::log!("{:?} {:?} {:?} {:?}", edge.key, edge.priority, from1_value, value);
                temp.get_attribute_mut(edge.to).set_value_by(&edge.key, value);
            }

            result = temp.clone();
        }

        for (node, (key, value)) in self.fixed.iter() {
            result.get_attribute_mut(*node).set_value_to(key, *value);
        }

        result
    }
}