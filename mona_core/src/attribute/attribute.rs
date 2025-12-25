use std::sync::Arc;

use super::attribute_name::{AttributeName, InvisibleAttributeType};
use crate::attribute::complicated_attribute_graph::ComplicatedAttributeGraph;
use crate::attribute::simple_attribute_graph2::SimpleAttributeGraph2;
use crate::attribute::typing::{EdgeFunctionBwd, EdgeFunctionFwd};
use crate::attribute::{self, AttributeGraphResult, AttributeResult, AttributeResultWithCharacter};
use crate::character::team_status::{CharacterSelector, CharacterStatus};
use crate::character::CharacterStaticData;
use crate::common::{Element, SkillType};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum AttributeType {
    Panel(AttributeName),
    Invisible(InvisibleAttributeType),
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct AttributeNode {
    pub character_id: usize,
    pub attribute_type: AttributeType,
}

impl AttributeNode {
    pub fn new(character_id: usize, attribute_type: AttributeType) -> Self {
        AttributeNode {
            character_id,
            attribute_type,
        }
    }

    pub fn new_panel(character_id: usize, attribute_type: AttributeName) -> Self {
        AttributeNode {
            character_id,
            attribute_type: AttributeType::Panel(attribute_type),
        }
    }

    pub fn new_invisible(character_id: usize, attribute_type: InvisibleAttributeType) -> Self {
        AttributeNode {
            character_id,
            attribute_type: AttributeType::Invisible(attribute_type),
        }
    }

    pub fn get_parents(&self) -> Vec<AttributeNode> {
        match self.attribute_type {
            AttributeType::Panel(attribute_type) => {
                vec![*self]
            }
            AttributeType::Invisible(attribute_type) => {
                let mut temp = Vec::new();
                for element in if attribute_type.element == None {
                    vec![None]
                } else {
                    vec![attribute_type.element, None]
                } {
                    for skill in if attribute_type.skill == None {
                        vec![None]
                    } else {
                        vec![attribute_type.skill, None]
                    } {
                        for reaction in if attribute_type.reaction == None {
                            vec![None]
                        } else {
                            vec![attribute_type.reaction, None]
                        } {
                            temp.push(AttributeNode::new_invisible(
                                self.character_id,
                                InvisibleAttributeType::new(
                                    attribute_type.attribute_variable_type,
                                    element,
                                    skill,
                                    reaction,
                                ),
                            ));
                        }
                    }
                }

                temp
            }
        }
    }
}

pub type EdgeFunction = Arc<dyn Fn(f64, f64) -> f64>;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum EdgePriority {
    Static,
    Base,
    Common,
    Special,
    Last,
}

pub trait AttributeGraph {
    type EdgeHandle: Copy;
    type ResultType: AttributeGraphResult;

    fn set_value_to_internal(&mut self, name: AttributeNode, key: &str, value: f64);

    fn set_value_by_internal(&mut self, name: AttributeNode, key: &str, value: f64);

    // fn add_edge(&mut self, from: AttributeName, to: AttributeName, edge: EdgeFunction, priority: usize, key: &str) -> Self::EdgeHandle;

    fn add_edge(
        &mut self,
        from1: AttributeNode,
        from2: AttributeNode,
        to: AttributeNode,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    ) -> Self::EdgeHandle;

    fn remove_edge(&mut self, handle: Self::EdgeHandle);

    fn new_with_characters(characters: Vec<CharacterStatus>) -> Self;

    fn get_characters(&self) -> &Vec<CharacterStatus>;

    fn solve(&self) -> Self::ResultType;
}

pub trait Attribute {
    type GraphTy: AttributeGraph;
    type ResultType: AttributeResult;

    fn new(attribute: Self::GraphTy, character_id: usize) -> Self;

    fn set_value_to_internal(&mut self, name: AttributeNode, key: &str, value: f64);

    fn set_value_by_internal(&mut self, name: AttributeNode, key: &str, value: f64);

    fn add_edge(
        &mut self,
        from1: AttributeNode,
        from2: AttributeNode,
        to: AttributeNode,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    ) -> <Self::GraphTy as AttributeGraph>::EdgeHandle;

    fn remove_edge(&mut self, handle: <Self::GraphTy as AttributeGraph>::EdgeHandle);

    fn get_character_id(&self) -> &usize;

    fn get_characters(&self) -> &Vec<CharacterStatus>;

    fn solve(&self) -> Self::ResultType;
}

pub struct AttributeWithCharacter<GraphTy: AttributeGraph> {
    pub attribute: GraphTy,
    pub character_id: usize,
}

pub type SimpleAttribute = AttributeWithCharacter<SimpleAttributeGraph2>;
pub type ComplicatedAttribute = AttributeWithCharacter<ComplicatedAttributeGraph>;

impl<GraphTy: AttributeGraph> Attribute for AttributeWithCharacter<GraphTy> {
    type GraphTy = GraphTy;
    type ResultType = AttributeResultWithCharacter<<Self::GraphTy as AttributeGraph>::ResultType>;

    fn new(attribute: GraphTy, character_id: usize) -> Self {
        AttributeWithCharacter {
            attribute,
            character_id,
        }
    }

    fn set_value_by_internal(&mut self, name: AttributeNode, key: &str, value: f64) {
        self.attribute.set_value_by_internal(name, key, value);
    }

    fn set_value_to_internal(&mut self, name: AttributeNode, key: &str, value: f64) {
        self.attribute.set_value_to_internal(name, key, value);
    }

    fn add_edge(
        &mut self,
        from1: AttributeNode,
        from2: AttributeNode,
        to: AttributeNode,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    ) -> GraphTy::EdgeHandle {
        self.attribute
            .add_edge(from1, from2, to, func, key, priority)
    }

    fn remove_edge(&mut self, handle: GraphTy::EdgeHandle) {
        self.attribute.remove_edge(handle);
    }

    fn get_character_id(&self) -> &usize {
        &self.character_id
    }

    fn get_characters(&self) -> &Vec<CharacterStatus> {
        self.attribute.get_characters()
    }

    fn solve(&self) -> Self::ResultType {
        AttributeResultWithCharacter {
            result: self.attribute.solve(),
            character_id: self.character_id,
        }
    }
}

pub trait AttributeCommon<T: Attribute> {
    fn new_with_base_edge(characters: Vec<CharacterStatus>) -> T::GraphTy;

    fn set_value_by(&mut self, name: AttributeName, key: &str, value: f64);

    fn set_value_to(&mut self, name: AttributeName, key: &str, value: f64);

    fn set_value_by_t(&mut self, ty: AttributeType, key: &str, value: f64);

    fn set_value_to_t(&mut self, ty: AttributeType, key: &str, value: f64);

    fn set_value_by_s(
        &mut self,
        character_selector: CharacterSelector,
        ty: AttributeType,
        key: &str,
        value: f64,
    );

    fn set_value_to_s(
        &mut self,
        character_selector: CharacterSelector,
        ty: AttributeType,
        key: &str,
        value: f64,
    );

    fn add_edge1(
        &mut self,
        from: AttributeName,
        to: AttributeName,
        fwd: EdgeFunctionFwd,
        bwd: EdgeFunctionBwd,
        key: &str,
    );

    fn add_edge2(
        &mut self,
        from1: AttributeName,
        from2: AttributeName,
        to: AttributeName,
        fwd: EdgeFunctionFwd,
        bwd: EdgeFunctionBwd,
        key: &str,
    );

    fn add_edge_n1(
        &mut self,
        from: AttributeName,
        to: AttributeName,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    );

    fn add_edge_n2(
        &mut self,
        from1: AttributeName,
        from2: AttributeName,
        to: AttributeName,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    );

    fn add_edge_t1(
        &mut self,
        from: AttributeType,
        to: AttributeType,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    );

    fn add_edge_t2(
        &mut self,
        from1: AttributeType,
        from2: AttributeType,
        to: AttributeType,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    );

    fn add_edge_s1to1(
        &mut self,
        character_selector: CharacterSelector,
        from: AttributeType,
        to: AttributeType,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    );

    fn add_edge_s2to1(
        &mut self,
        character_selector: CharacterSelector,
        from1: AttributeType,
        from2: AttributeType,
        to: AttributeType,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    );

    fn add_edge_s1ton(
        &mut self,
        character_selector: CharacterSelector,
        from: AttributeType,
        to: AttributeType,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    );

    fn add_edge_s2ton(
        &mut self,
        character_selector: CharacterSelector,
        from1: AttributeType,
        from2: AttributeType,
        to: AttributeType,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    );

    fn add_atk_percentage(&mut self, key: &str, value: f64);

    fn add_atk_percentage_base(&mut self, key: &str, value: f64);

    fn add_def_percentage(&mut self, key: &str, value: f64);

    fn add_def_percentage_base(&mut self, key: &str, value: f64);

    fn add_hp_percentage(&mut self, key: &str, value: f64);

    fn add_hp_percentage_base(&mut self, key: &str, value: f64);

    fn add_elemental_bonus(&mut self, key: &str, value: f64);
}

impl<T: Attribute> AttributeCommon<T> for T {
    fn new_with_base_edge(characters: Vec<CharacterStatus>) -> T::GraphTy {
        let mut temp = T::GraphTy::new_with_characters(characters.clone());

        for character in characters {
            temp.add_edge(
                AttributeNode::new_panel(character.character_id, AttributeName::ATKBase),
                AttributeNode::new_panel(character.character_id, AttributeName::ATKBase),
                AttributeNode::new_panel(character.character_id, AttributeName::ATK),
                Arc::new(|x1, _x2| x1),
                "atk_base",
                EdgePriority::Static,
            );
            temp.add_edge(
                AttributeNode::new_panel(character.character_id, AttributeName::ATKPercentage),
                AttributeNode::new_panel(character.character_id, AttributeName::ATKPercentage),
                AttributeNode::new_panel(character.character_id, AttributeName::ATK),
                Arc::new(|x1, _x2| x1),
                "atk_percentage",
                EdgePriority::Static,
            );
            temp.add_edge(
                AttributeNode::new_panel(character.character_id, AttributeName::ATKFixed),
                AttributeNode::new_panel(character.character_id, AttributeName::ATKFixed),
                AttributeNode::new_panel(character.character_id, AttributeName::ATK),
                Arc::new(|x1, _x2| x1),
                "atk_fixed",
                EdgePriority::Static,
            );

            temp.add_edge(
                AttributeNode::new_panel(character.character_id, AttributeName::HPBase),
                AttributeNode::new_panel(character.character_id, AttributeName::HPBase),
                AttributeNode::new_panel(character.character_id, AttributeName::HP),
                Arc::new(|x1, _x2| x1),
                "hp_base",
                EdgePriority::Static,
            );
            temp.add_edge(
                AttributeNode::new_panel(character.character_id, AttributeName::HPPercentage),
                AttributeNode::new_panel(character.character_id, AttributeName::HPPercentage),
                AttributeNode::new_panel(character.character_id, AttributeName::HP),
                Arc::new(|x1, _x2| x1),
                "hp_percentage",
                EdgePriority::Static,
            );
            temp.add_edge(
                AttributeNode::new_panel(character.character_id, AttributeName::HPFixed),
                AttributeNode::new_panel(character.character_id, AttributeName::HPFixed),
                AttributeNode::new_panel(character.character_id, AttributeName::HP),
                Arc::new(|x1, _x2| x1),
                "hp_fixed",
                EdgePriority::Static,
            );

            temp.add_edge(
                AttributeNode::new_panel(character.character_id, AttributeName::DEFBase),
                AttributeNode::new_panel(character.character_id, AttributeName::DEFBase),
                AttributeNode::new_panel(character.character_id, AttributeName::DEF),
                Arc::new(|x1, _x2| x1),
                "def_base",
                EdgePriority::Static,
            );
            temp.add_edge(
                AttributeNode::new_panel(character.character_id, AttributeName::DEFPercentage),
                AttributeNode::new_panel(character.character_id, AttributeName::DEFPercentage),
                AttributeNode::new_panel(character.character_id, AttributeName::DEF),
                Arc::new(|x1, _x2| x1),
                "def_percentage",
                EdgePriority::Static,
            );
            temp.add_edge(
                AttributeNode::new_panel(character.character_id, AttributeName::DEFFixed),
                AttributeNode::new_panel(character.character_id, AttributeName::DEFFixed),
                AttributeNode::new_panel(character.character_id, AttributeName::DEF),
                Arc::new(|x1, _x2| x1),
                "def_fixed",
                EdgePriority::Static,
            );

            temp.set_value_by_internal(
                AttributeNode::new_panel(character.character_id, AttributeName::CriticalBase),
                "初始值",
                0.05,
            );
            temp.set_value_by_internal(
                AttributeNode::new_panel(character.character_id, AttributeName::CriticalDamageBase),
                "初始值",
                0.5,
            );
            temp.set_value_by_internal(
                AttributeNode::new_panel(character.character_id, AttributeName::Recharge),
                "初始值",
                1.0,
            );
        }

        temp
    }

    fn set_value_by(&mut self, name: AttributeName, key: &str, value: f64) {
        self.set_value_by_internal(
            AttributeNode::new_panel(*self.get_character_id(), name),
            key,
            value,
        );
    }

    fn set_value_to(&mut self, name: AttributeName, key: &str, value: f64) {
        self.set_value_to_internal(
            AttributeNode::new_panel(*self.get_character_id(), name),
            key,
            value,
        );
    }

    fn set_value_by_t(&mut self, ty: AttributeType, key: &str, value: f64) {
        self.set_value_by_internal(AttributeNode::new(*self.get_character_id(), ty), key, value);
    }

    fn set_value_to_t(&mut self, ty: AttributeType, key: &str, value: f64) {
        self.set_value_to_internal(AttributeNode::new(*self.get_character_id(), ty), key, value);
    }

    fn set_value_by_s(
        &mut self,
        character_selector: CharacterSelector,
        ty: AttributeType,
        key: &str,
        value: f64,
    ) {
        let list = character_selector.get_matched_list(self.get_characters());
        for id in list.iter() {
            self.set_value_by_internal(AttributeNode::new(*id, ty), key, value);
        }
    }

    fn set_value_to_s(
        &mut self,
        character_selector: CharacterSelector,
        ty: AttributeType,
        key: &str,
        value: f64,
    ) {
        let list = character_selector.get_matched_list(self.get_characters());
        for id in list.iter() {
            self.set_value_to_internal(AttributeNode::new(*id, ty), key, value);
        }
    }

    fn add_edge1(
        &mut self,
        from: AttributeName,
        to: AttributeName,
        fwd: EdgeFunctionFwd,
        bwd: EdgeFunctionBwd,
        key: &str,
    ) {
        T::add_edge_n1(self, from, to, Arc::from(fwd), key, EdgePriority::Common);
    }

    fn add_edge2(
        &mut self,
        from1: AttributeName,
        from2: AttributeName,
        to: AttributeName,
        fwd: EdgeFunctionFwd,
        bwd: EdgeFunctionBwd,
        key: &str,
    ) {
        T::add_edge_n2(
            self,
            from1,
            from2,
            to,
            Arc::from(fwd),
            key,
            EdgePriority::Common,
        );
    }

    fn add_edge_n1(
        &mut self,
        from: AttributeName,
        to: AttributeName,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    ) {
        self.add_edge(
            AttributeNode::new_panel(*self.get_character_id(), from),
            AttributeNode::new_panel(*self.get_character_id(), from),
            AttributeNode::new_panel(*self.get_character_id(), to),
            func,
            key,
            priority,
        );
    }

    fn add_edge_n2(
        &mut self,
        from1: AttributeName,
        from2: AttributeName,
        to: AttributeName,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    ) {
        self.add_edge(
            AttributeNode::new_panel(*self.get_character_id(), from1),
            AttributeNode::new_panel(*self.get_character_id(), from2),
            AttributeNode::new_panel(*self.get_character_id(), to),
            func,
            key,
            priority,
        );
    }

    fn add_edge_t1(
        &mut self,
        from: AttributeType,
        to: AttributeType,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    ) {
        self.add_edge(
            AttributeNode::new(*self.get_character_id(), from),
            AttributeNode::new(*self.get_character_id(), from),
            AttributeNode::new(*self.get_character_id(), to),
            func,
            key,
            priority,
        );
    }

    fn add_edge_t2(
        &mut self,
        from1: AttributeType,
        from2: AttributeType,
        to: AttributeType,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    ) {
        self.add_edge(
            AttributeNode::new(*self.get_character_id(), from1),
            AttributeNode::new(*self.get_character_id(), from2),
            AttributeNode::new(*self.get_character_id(), to),
            func,
            key,
            priority,
        );
    }

    fn add_edge_s1to1(
        &mut self,
        character_selector: CharacterSelector,
        from: AttributeType,
        to: AttributeType,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    ) {
        let list = character_selector.get_matched_list(self.get_characters());
        for id in list.iter() {
            self.add_edge(
                AttributeNode::new(*self.get_character_id(), from),
                AttributeNode::new(*self.get_character_id(), from),
                AttributeNode::new(*id, to),
                func.clone(),
                key,
                priority,
            );
        }
    }

    fn add_edge_s2to1(
        &mut self,
        character_selector: CharacterSelector,
        from1: AttributeType,
        from2: AttributeType,
        to: AttributeType,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    ) {
        let list = character_selector.get_matched_list(self.get_characters());
        for id in list.iter() {
            self.add_edge(
                AttributeNode::new(*self.get_character_id(), from1),
                AttributeNode::new(*self.get_character_id(), from2),
                AttributeNode::new(*id, to),
                func.clone(),
                key,
                priority,
            );
        }
    }

    fn add_edge_s1ton(
        &mut self,
        character_selector: CharacterSelector,
        from: AttributeType,
        to: AttributeType,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    ) {
        let list = character_selector.get_matched_list(self.get_characters());
        for id in list.iter() {
            self.add_edge(
                AttributeNode::new(*id, from),
                AttributeNode::new(*id, from),
                AttributeNode::new(*id, to),
                func.clone(),
                key,
                priority,
            );
        }
    }

    fn add_edge_s2ton(
        &mut self,
        character_selector: CharacterSelector,
        from1: AttributeType,
        from2: AttributeType,
        to: AttributeType,
        func: EdgeFunction,
        key: &str,
        priority: EdgePriority,
    ) {
        let list = character_selector.get_matched_list(self.get_characters());
        for id in list.iter() {
            self.add_edge(
                AttributeNode::new(*id, from1),
                AttributeNode::new(*id, from2),
                AttributeNode::new(*id, to),
                func.clone(),
                key,
                priority,
            );
        }
    }

    fn add_atk_percentage(&mut self, key: &str, value: f64) {
        self.add_edge_n1(
            AttributeName::ATKBase,
            AttributeName::ATKPercentage,
            Arc::new(move |x, _| x * value),
            key,
            EdgePriority::Common,
        );
    }

    fn add_atk_percentage_base(&mut self, key: &str, value: f64) {
        self.add_edge_n1(
            AttributeName::ATKBase,
            AttributeName::ATKPercentage,
            Arc::new(move |x, _| x * value),
            key,
            EdgePriority::Base,
        );
    }

    fn add_def_percentage(&mut self, key: &str, value: f64) {
        self.add_edge_n1(
            AttributeName::DEFBase,
            AttributeName::DEFPercentage,
            Arc::new(move |x, _| x * value),
            key,
            EdgePriority::Common,
        );
    }

    fn add_def_percentage_base(&mut self, key: &str, value: f64) {
        self.add_edge_n1(
            AttributeName::DEFBase,
            AttributeName::DEFPercentage,
            Arc::new(move |x, _| x * value),
            key,
            EdgePriority::Base,
        );
    }

    fn add_hp_percentage(&mut self, key: &str, value: f64) {
        self.add_edge_n1(
            AttributeName::HPBase,
            AttributeName::HPPercentage,
            Arc::new(move |x, _| x * value),
            key,
            EdgePriority::Common,
        );
    }

    fn add_hp_percentage_base(&mut self, key: &str, value: f64) {
        self.add_edge_n1(
            AttributeName::HPBase,
            AttributeName::HPPercentage,
            Arc::new(move |x, _| x * value),
            key,
            EdgePriority::Base,
        );
    }

    fn add_elemental_bonus(&mut self, key: &str, value: f64) {
        self.set_value_by(AttributeName::BonusElectro, key, value);
        self.set_value_by(AttributeName::BonusPyro, key, value);
        self.set_value_by(AttributeName::BonusHydro, key, value);
        self.set_value_by(AttributeName::BonusAnemo, key, value);
        self.set_value_by(AttributeName::BonusCryo, key, value);
        self.set_value_by(AttributeName::BonusGeo, key, value);
        self.set_value_by(AttributeName::BonusDendro, key, value);
    }
}
