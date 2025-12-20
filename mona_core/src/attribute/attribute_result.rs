use crate::attribute::{ComplicatedAttributeGraphResult, SimpleAttributeGraphResult};
use crate::common::{Element, SkillType};
use crate::damage::damage_builder::DamageBuilder;
use super::{AttributeName, AttributeType, AttributeNode};
use crate::character::team_status::CharacterStatus;

pub trait AttributeGraphResult {
    type ResultType;

    fn get_attribute_value(&self, node: AttributeNode) -> f64;

    fn get_attribute(&self, node: AttributeNode) -> Self::ResultType;

    fn get_attribute_merge(&self, nodes: &[AttributeNode]) -> Self::ResultType;
}

pub struct AttributeResultWithCharacter<ResultTy: AttributeGraphResult> {
    pub result: ResultTy,
    pub character_id: usize,
}

pub type SimpleAttributeResult = AttributeResultWithCharacter<SimpleAttributeGraphResult>;
pub type ComplicatedAttributeResult = AttributeResultWithCharacter<ComplicatedAttributeGraphResult>;

pub trait AttributeResult {
    type ResultType;
    
    fn get_value(&self, name: AttributeName) -> f64;

    fn get_result(&self, name: AttributeName) -> Self::ResultType;

    fn get_result_t(&self, ty: AttributeType) -> Self::ResultType;

    fn get_result_merge(&self, names: &[AttributeName]) -> Self::ResultType;

    fn get_em_all(&self) -> f64 {
        self.get_value(AttributeName::ElementalMastery) + self.get_value(AttributeName::ElementalMasteryExtra)
    }

    fn get_atk(&self) -> f64 {
        self.get_value(AttributeName::ATK)
    }

    fn get_hp(&self) -> f64 {
        self.get_value(AttributeName::HP)
    }

    fn get_def(&self) -> f64 {
        self.get_value(AttributeName::DEF)
    }

    fn get_atk_ratio(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::atk_ratio_name_by_element(element);
        let key2 = AttributeName::atk_ratio_name_by_skill_type(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        self.get_value(AttributeName::ATKRatioBase)
            + self.get_value(key1) + value2
    }

    fn get_def_ratio(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::def_ratio_name_by_element(element);
        let key2 = AttributeName::def_ratio_name_by_skill_type(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        self.get_value(AttributeName::DEFRatioBase)
            + self.get_value(key1) + value2
    }

    fn get_hp_ratio(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::hp_ratio_name_by_element(element);
        let key2 = AttributeName::hp_ratio_name_by_skill_type(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        self.get_value(AttributeName::HPRatioBase)
            + self.get_value(key1) + value2
    }

    fn get_extra_damage(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::extra_dmg_name_by_element(element);
        let key2 = AttributeName::extra_dmg_name_by_skill_type(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        let value3 = if skill == SkillType::PlungingAttackOnGround {
            self.get_value(AttributeName::ExtraDmgPlungingAttackLowHigh)
        } else {
            0.0
        };

        self.get_value(AttributeName::ExtraDmgBase)
            + self.get_value(key1) + value2 + value3
    }

    fn get_bonus(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::bonus_name_by_element(element);
        let key2 = AttributeName::bonus_name_by_skill_type(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        let mut temp = self.get_value(AttributeName::BonusBase)
            + self.get_value(key1) + value2;
        // todo refactor
        if element != Element::Physical && skill == SkillType::NormalAttack {
            temp += self.get_value(AttributeName::BonusNormalAndElemental);
        }
        temp
    }

    fn get_critical_rate(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::critical_rate_name_by_element(element);
        let key2 = AttributeName::critical_rate_name_by_skill_type(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        self.get_value(AttributeName::CriticalBase) + self.get_value(AttributeName::CriticalAttacking)
            + self.get_value(key1) + value2
    }

    fn get_critical_damage(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::critical_damage_name_by_element(element);
        let key2 = AttributeName::critical_damage_name_by_skill_name(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        self.get_value(AttributeName::CriticalDamageBase)
            + self.get_value(key1) + value2
    }

    fn get_enemy_res_minus(&self, element: Element, _skill: SkillType) -> f64 {
        self.get_value(AttributeName::ResMinusBase)
            + self.get_value(AttributeName::res_minus_name_by_element(element))
    }

    fn get_enemy_def_minus(&self, _element: Element, _skill: SkillType) -> f64 {
        self.get_value(AttributeName::DefMinus)
    }
}

impl<ResultTy: AttributeGraphResult> AttributeResult for AttributeResultWithCharacter<ResultTy> {
    type ResultType = ResultTy::ResultType;

    fn get_value(&self, name: AttributeName) -> f64 {
        self.result.get_attribute_value(AttributeNode::new_panel(self.character_id, name))
    }

    fn get_result(&self, name: AttributeName) -> Self::ResultType {
        self.result.get_attribute(AttributeNode::new_panel(self.character_id, name))
    }

    fn get_result_t(&self, ty: AttributeType) -> Self::ResultType {
        self.result.get_attribute(AttributeNode::new(self.character_id, ty))
    }

    fn get_result_merge(&self, names: &[AttributeName]) -> Self::ResultType {
        let nodes: Vec<AttributeNode> = names.iter().map(|&name| {
            AttributeNode::new_panel(self.character_id, name)
        }).collect();
        self.result.get_attribute_merge(nodes.as_slice())
    }
}