use crate::attribute::{Attribute, AttributeResult};
use crate::common::{DamageResult, Element, MoonglareReaction, SkillType};
use crate::common::reaction_type::TransformativeType;
use crate::damage::{DamageAnalysis, DamageBuilderResult};
use crate::enemies::Enemy;

pub trait DamageBuilder {
    type Result: DamageBuilderResult;
    type AttributeType: AttributeResult;

    fn new() -> Self;

    fn add_atk_ratio(&mut self, key: &str, value: f64);

    fn add_def_ratio(&mut self, key: &str, value: f64);

    fn add_hp_ratio(&mut self, key: &str, value: f64);

    fn add_em_ratio(&mut self, key: &str, value: f64);

    fn add_base(&mut self, key: &str, value: f64); // 用于添加治疗量或护盾量的常数值

    // 以下方法由于误差问题已弃用，请调用 Attribute 中的方法以获得更加准确的结果
    // the following methods are deprecated due to accuracy issues, please use the methods in Attribute for more accurate results
    fn add_extra_em(&mut self, key: &str, value: f64);

    fn add_extra_atk(&mut self, key: &str, value: f64);

    fn add_extra_def(&mut self, key: &str, value: f64);

    fn add_extra_hp(&mut self, key: &str, value: f64);

    fn add_extra_damage(&mut self, key: &str, value: f64);

    fn add_extra_critical(&mut self, key: &str, value: f64);

    fn add_extra_critical_damage(&mut self, key: &str, value: f64);

    fn add_extra_bonus(&mut self, key: &str, value: f64);

    fn add_extra_enhance_melt(&mut self, key: &str, value: f64);

    fn add_extra_enhance_vaporize(&mut self, key: &str, value: f64);

    fn add_extra_def_minus(&mut self, key: &str, value: f64);

    fn add_extra_def_penetration(&mut self, key: &str, value: f64);

    fn add_extra_res_minus(&mut self, key: &str, value: f64);

    fn damage(
        &self,
        attribute: &Self::AttributeType,
        enemy: &Enemy,
        element: Element,
        skill_type: SkillType,
        character_level: usize,
        fumo: Option<Element>
    ) -> Self::Result;

    fn transformative(
        &self,
        attribute: &Self::AttributeType,
        enemy: &Enemy,
        transformative_type: TransformativeType,
        character_level: usize,
    ) -> Self::Result;

    fn moonglare(
        &self,
        attribute: &Self::AttributeType,
        enemy: &Enemy,
        element: Element,
        lunar_type: MoonglareReaction,
        skill_type: SkillType,
        character_level: usize,
        fumo: Option<Element>
    ) -> Self::Result;

    fn heal(&self, attribute: &Self::AttributeType) -> Self::Result;

    fn shield(&self, attribute: &Self::AttributeType, element: Element) -> Self::Result;

    fn none(&self) -> Self::Result;
}
