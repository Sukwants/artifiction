use core::panic;
use std::collections::HashMap;
use num_traits::Inv;

use crate::attribute::*;
use crate::buffs::buffs::base_dmg;
use crate::common::{DamageResult, Element, TransformativeType, MoonglareReaction, ReactionType, SkillType};
use crate::damage::damage_analysis::{DamageAnalysis, DamageAnalysisWithPossibleReaction, EventAnalysis, HealAnalysis, MoonglareDamageAnalysis, ShieldAnalysis, TransformativeDamageAnalysis};
use crate::enemies::Enemy;
use crate::common::EntryType;
use crate::damage::damage_builder::{DamageBuilder};
use crate::damage::level_coefficient::{LEVEL_MULTIPLIER, CRYSTALLIZE_BASE};
use crate::damage::reaction::Reaction;
use crate::damage::SimpleDamageBuilder;
use crate::weapon::weapons::emerald_orb;

#[derive(Default)]
pub struct ComplicatedDamageBuilder {
    pub extra_critical_damage: EntryType,
    pub extra_critical_rate: EntryType,
    pub extra_bonus: EntryType,
    pub extra_damage: EntryType,
    pub extra_atk: EntryType,
    pub extra_def: EntryType,
    pub extra_hp: EntryType,
    pub extra_healing_bonus: EntryType,
    
    pub extra_enhance_melt: EntryType,
    pub extra_enhance_vaporize: EntryType,
    pub extra_em: EntryType,
    
    pub extra_def_minus: EntryType,
    pub extra_def_penetration: EntryType,
    pub extra_res_minus: EntryType,
    
    pub ratio_atk: EntryType,
    pub ratio_def: EntryType,
    pub ratio_hp: EntryType,
    pub ratio_em: EntryType,
    pub base: EntryType,
}

impl DamageBuilder for ComplicatedDamageBuilder {
    type Result = EventAnalysis;
    type AttributeType = ComplicatedAttributeResult;

    fn new() -> Self {
        ComplicatedDamageBuilder::default()
    }

    fn add_em_ratio(&mut self, key: &str, value: f64) {
        if value > 0.0 {
            *self.ratio_em.0.entry(String::from(key)).or_insert(0.0) += value;
        }
    }
    
    fn add_atk_ratio(&mut self, key: &str, value: f64) {
        if value > 0.0 {
            *self.ratio_atk.0.entry(String::from(key)).or_insert(0.0) += value;
        }
    }

    fn add_def_ratio(&mut self, key: &str, value: f64) {
        if value > 0.0 {
            *self.ratio_def.0.entry(String::from(key)).or_insert(0.0) += value;
        }
    }

    fn add_hp_ratio(&mut self, key: &str, value: f64) {
        if value > 0.0 {
            *self.ratio_hp.0.entry(String::from(key)).or_insert(0.0) += value;
        }
    }

    fn add_base(&mut self, key: &str, value: f64) {
        *self.base.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_em(&mut self, key: &str, value: f64) {
        *self.extra_em.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_atk(&mut self, key: &str, value: f64) {
        *self.extra_atk.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_def(&mut self, key: &str, value: f64) {
        *self.extra_def.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_hp(&mut self, key: &str, value: f64) {
        *self.extra_hp.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_damage(&mut self, key: &str, value: f64) {
        *self.extra_damage.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_critical(&mut self, key: &str, value: f64) {
        *self.extra_critical_rate.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_critical_damage(&mut self, key: &str, value: f64) {
        *self.extra_critical_damage.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_bonus(&mut self, key: &str, value: f64) {
        *self.extra_bonus.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_enhance_melt(&mut self, key: &str, value: f64) {
        *self.extra_enhance_melt.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_enhance_vaporize(&mut self, key: &str, value: f64) {
        *self.extra_enhance_vaporize.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_def_minus(&mut self, key: &str, value: f64) {
        *self.extra_def_minus.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_def_penetration(&mut self, key: &str, value: f64) {
        *self.extra_def_penetration.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_res_minus(&mut self, key: &str, value: f64) {
        *self.extra_res_minus.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn damage(
        &self,
        attribute: &Self::AttributeType,
        enemy: &Enemy,
        element: Element,
        skill: SkillType,
        character_level: usize,
        fumo: Option<Element>
    ) -> Self::Result {
        let element = if skill == SkillType::NormalAttack || skill == SkillType::ChargedAttack || skill.is_plunging() {
            if let Some(x) = fumo {
                x
            } else {
                element
            }
        } else {
            element
        };

        let get_damage = |reaction: Option<ReactionType>| -> DamageAnalysis {
            let get_attribute_type = |variable: AttributeVariableType| -> AttributeType {
                AttributeType::Invisible(InvisibleAttributeType::new(
                    variable,
                    Some(element),
                    Some(skill),
                    reaction,
                ))
            };

            let atk_comp = self.get_atk_composition(attribute);
            let atk = atk_comp.sum();
            let atk_ratio_comp = self.get_atk_ratio_composition(attribute, element, skill);
            let atk_ratio = atk_ratio_comp.sum();

            let def_comp = self.get_def_composition(attribute);
            let def = def_comp.sum();
            let def_ratio_comp = self.get_def_ratio_composition(attribute, element, skill);
            let def_ratio = def_ratio_comp.sum();

            let hp_comp = self.get_hp_composition(attribute);
            let hp = hp_comp.sum();
            let hp_ratio_comp = self.get_hp_ratio_composition(attribute, element, skill);
            let hp_ratio = hp_ratio_comp.sum();

            let em_comp = self.get_em_composition(attribute);
            let em = em_comp.sum();
            let em_ratio_comp = self.get_em_ratio_composition(attribute, element, skill);
            let em_ratio = em_ratio_comp.sum();

            let extra_damage_comp = self.get_extra_damage_composition(attribute, element, skill)
                .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::BaseDamage)));
            let extra_damage = extra_damage_comp.sum();

            let base_damage = atk * atk_ratio + def * def_ratio + hp * hp_ratio + em * em_ratio + extra_damage;

            let bonus_comp = self.get_bonus_composition(attribute, element, skill)
                .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::Bonus)));
            let bonus = bonus_comp.sum();

            let critical_comp = self.get_critical_composition(attribute, element, skill)
                .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::CriticalRate)));
            let critical = critical_comp.sum().clamp(0.0, 1.0);

            let critical_damage_comp = self.get_critical_damage_composition(attribute, element, skill)
                .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::CriticalDamage)));
            let critical_damage = critical_damage_comp.sum();

            let def_minus_comp = self.get_def_minus_composition(attribute)
                .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::DefMinus)));
            let def_minus = def_minus_comp.sum();
            let def_penetration_comp = self.get_def_penetration_composition(attribute)
                .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::DefPenetration)));
            let def_penetration = def_penetration_comp.sum();
            let res_minus_comp = self.get_res_minus_composition(attribute, element)
                .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::ResMinus)));
            let res_minus = res_minus_comp.sum();
            let defensive_ratio = enemy.get_defensive_ratio(character_level, def_minus, def_penetration);
            let resistance_ratio = enemy.get_resistance_ratio(element, res_minus);

            let reaction_enhance_comp = match reaction {
                Some(ReactionType::Melt) => self.get_enhance_melt_composition(attribute),
                Some(ReactionType::Vaporize) => self.get_enhance_vaporize_composition(attribute),
                Some(ReactionType::Spread) => self.get_enhance_spread_composition(attribute),
                Some(ReactionType::Aggravate) => self.get_enhance_aggravate_composition(attribute),
                _ => EntryType::new()
            }.merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::ReactionEnhance)));
            let reaction_enhance = reaction_enhance_comp.sum();

            let reaction_base = match reaction {
                Some(ReactionType::Spread) => LEVEL_MULTIPLIER[character_level - 1] * 1.25,
                Some(ReactionType::Aggravate) => LEVEL_MULTIPLIER[character_level - 1] * 1.15,
                _ => 0.0
            };

            let reaction_coefficient = match reaction {
                Some(ReactionType::Melt) => match element {
                    Element::Pyro => 2.0,
                    Element::Cryo => 1.5,
                    _ => panic!()
                },
                Some(ReactionType::Vaporize) => match element {
                    Element::Hydro => 2.0,
                    Element::Pyro => 1.5,
                    _ => panic!()
                },
                _ => 1.0
            };

            let damage = match reaction {
                None => DamageResult {
                    expectation: base_damage * (1.0 + bonus) * (1.0 + critical * critical_damage),
                    critical: base_damage * (1.0 + bonus) * (1.0 + critical_damage),
                    non_critical: base_damage * (1.0 + bonus),
                } * (defensive_ratio * resistance_ratio),
                Some(ReactionType::Melt) | Some(ReactionType::Vaporize) => DamageResult {
                    expectation: base_damage * (1.0 + bonus) * (1.0 + critical * critical_damage),
                    critical: base_damage * (1.0 + bonus) * (1.0 + critical_damage),
                    non_critical: base_damage * (1.0 + bonus),
                } * (defensive_ratio * resistance_ratio) * reaction_coefficient * (1.0 + reaction_enhance),
                Some(ReactionType::Spread) | Some(ReactionType::Aggravate) => {
                    let reaction_base_damage = base_damage + reaction_base * (1.0 + reaction_enhance);
                    DamageResult {
                        critical: reaction_base_damage * (1.0 + bonus) * (1.0 + critical_damage),
                        non_critical: reaction_base_damage * (1.0 + bonus),
                        expectation: reaction_base_damage * (1.0 + bonus) * (1.0 + critical_damage * critical),
                    } * (defensive_ratio * resistance_ratio)
                },
                _ => panic!()
            };

            DamageAnalysis {
                element: element,
                reaction_type: reaction,
                
                atk: atk_comp.0,
                atk_ratio: atk_ratio_comp.0,
                hp: hp_comp.0,
                hp_ratio: hp_ratio_comp.0,
                def: def_comp.0,
                def_ratio: def_ratio_comp.0,
                em: em_comp.0,
                em_ratio: em_ratio_comp.0,
                reaction_base: reaction_base,
                reaction_coefficient: reaction_coefficient,

                base_damage: extra_damage_comp.0,
                bonus: bonus_comp.0,
                reaction_enhance: reaction_enhance_comp.0,
                critical_rate: critical_comp.0,
                critical_damage: critical_damage_comp.0,
                res_minus: res_minus_comp.0,
                def_minus: def_minus_comp.0,
                def_penetration: def_penetration_comp.0,

                result: damage,
            }
        };

        EventAnalysis::Damage(DamageAnalysisWithPossibleReaction{
            normal: get_damage(None),
            melt: if element == Element::Pyro || element == Element::Cryo {
                Some(get_damage(Some(ReactionType::Melt)))
            } else { None },
            vaporize: if element == Element::Pyro || element == Element::Hydro {
                Some(get_damage(Some(ReactionType::Vaporize)))
            } else { None },
            spread: if element == Element::Dendro {
                Some(get_damage(Some(ReactionType::Spread)))
            } else { None },
            aggravate: if element == Element::Electro {
                Some(get_damage(Some(ReactionType::Aggravate)))
            } else { None },
        })
    }
    
    fn transformative(
        &self,
        attribute: &Self::AttributeType,
        enemy: &Enemy,
        transformative_type: TransformativeType,
        character_level: usize,
    ) -> Self::Result {
        let element = transformative_type.get_element();
        let reaction = ReactionType::get_reaction_from_transformative_type(transformative_type);
        
        let get_attribute_type = |variable: AttributeVariableType| -> AttributeType {
            AttributeType::Invisible(InvisibleAttributeType::new(
                variable,
                None,
                None,
                Some(reaction),
            ))
        };
        
        let reaction_base = transformative_type.get_reaction_base(character_level);
        let reaction_coefficient = transformative_type.get_reaction_coefficient();

        let base_damage = reaction_base * reaction_coefficient;

        let enhance_comp = self.get_enhance_transformative_composition(attribute, transformative_type)
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::ReactionEnhance)));
        let enhance = enhance_comp.sum();

        let extra_increase_comp = self.get_extra_increase_reaction_composition(attribute, reaction)
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::ReactionExtra)));
        let extra_increase = extra_increase_comp.sum();

        let critical_comp = self.extra_critical_rate
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::CriticalRate)));
        let critical = critical_comp.sum().clamp(0.0, 1.0);

        let critical_damage_comp = self.extra_critical_damage
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::CriticalDamage)));
        let critical_damage = critical_damage_comp.sum();

        let res_minus_comp = if transformative_type != TransformativeType::Crystallize {
            self.get_res_minus_composition(attribute, element.unwrap())
                .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::ResMinus)))
        } else { EntryType::new() };
        let res_minus = res_minus_comp.sum();
        let resistance_ratio = if transformative_type != TransformativeType::Crystallize {
            enemy.get_resistance_ratio(element.unwrap(), res_minus)
        } else { 1.0 };

        let damage = {
            let dmg = base_damage * (1.0 + enhance) + extra_increase;
            DamageResult {
                critical: dmg * (1.0 + critical_damage),
                non_critical: dmg,
                expectation: dmg * (1.0 + critical_damage * critical),
            } * resistance_ratio
        };

        EventAnalysis::TransformativeDamage(TransformativeDamageAnalysis {
            element: element,
            transformative_type: transformative_type,
            
            reaction_base: reaction_base,
            reaction_coefficient: reaction_coefficient,

            reaction_enhance: enhance_comp.0,
            reaction_extra: extra_increase_comp.0,
            critical_rate: critical_comp.0,
            critical_damage: critical_damage_comp.0,
            res_minus: res_minus_comp.0,

            result: damage,
        })
    }

    fn moonglare(
        &self,
        attribute: &Self::AttributeType,
        enemy: &Enemy,
        element: Element,
        lunar_type: MoonglareReaction,
        skill: SkillType,
        character_level: usize,
        fumo: Option<Element>
    ) -> Self::Result {
        let reaction = ReactionType::get_reaction_from_lunar_type(lunar_type).unwrap();
        
        let get_attribute_type = |variable: AttributeVariableType| -> AttributeType {
            AttributeType::Invisible(InvisibleAttributeType::new(
                variable,
                Some(element),
                Some(skill),
                Some(reaction),
            ))
        };

        let atk_comp = self.get_atk_composition(attribute);
        let atk = atk_comp.sum();
        let atk_ratio_comp = self.get_atk_ratio_composition(attribute, element, skill);
        let atk_ratio = atk_ratio_comp.sum();

        let def_comp = self.get_def_composition(attribute);
        let def = def_comp.sum();
        let def_ratio_comp = self.get_def_ratio_composition(attribute, element, skill);
        let def_ratio = def_ratio_comp.sum();

        let hp_comp = self.get_hp_composition(attribute);
        let hp = hp_comp.sum();
        let hp_ratio_comp = self.get_hp_ratio_composition(attribute, element, skill);
        let hp_ratio = hp_ratio_comp.sum();

        let em_comp = self.get_em_composition(attribute);
        let em = em_comp.sum();
        let em_ratio_comp = self.get_em_ratio_composition(attribute, element, skill);
        let em_ratio = em_ratio_comp.sum();

        let base_damage = atk * atk_ratio + def * def_ratio + hp * hp_ratio + em * em_ratio;

        let critical_comp = self.get_critical_composition(attribute, element, skill)
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::CriticalRate)));
        let critical = critical_comp.sum().clamp(0.0, 1.0);

        let critical_damage_comp = self.get_critical_damage_composition(attribute, element, skill)
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::CriticalDamage)));
        let critical_damage = critical_damage_comp.sum();

        let res_minus_comp = self.get_res_minus_composition(attribute, element)
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::ResMinus)));
        let res_minus = res_minus_comp.sum();
        let resistance_ratio = enemy.get_resistance_ratio(element, res_minus);

        let enhance_comp = self.get_enhance_moonglare_composition(attribute, lunar_type)
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::ReactionEnhance)));
        let enhance = enhance_comp.sum();

        let extra_increase_comp = self.get_extra_increase_reaction_composition(attribute, reaction)
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::ReactionExtra)));
        let extra_increase = extra_increase_comp.sum();

        let increase_comp = self.get_increase_moonglare_composition(attribute, lunar_type)
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::MoonglareBase)));
        let increase = increase_comp.sum();

        let elevate_comp = self.get_elevate_moonglare_composition(attribute, lunar_type)
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::MoonglareElevate)));
        let elevate = elevate_comp.sum();

        let reaction_base = match lunar_type {
            MoonglareReaction::LunarChargedReaction => LEVEL_MULTIPLIER[character_level - 1],
            _ => 0.0,
        };
        let reaction_coefficient = lunar_type.get_reaction_coefficient();

        let damage = {
            let dmg = match lunar_type {
                MoonglareReaction::LunarChargedReaction => reaction_base,
                MoonglareReaction::LunarCharged | MoonglareReaction::LunarBloom => base_damage,
                _ => panic!()
            } * reaction_coefficient * (1.0 + enhance) * (1.0 + increase) + extra_increase;
            DamageResult {
                critical: dmg * (1.0 + critical_damage),
                non_critical: dmg,
                expectation: dmg * (1.0 + critical_damage * critical),
            } * resistance_ratio * (1.0 + elevate)
        };

        EventAnalysis::MoonglareDamage(MoonglareDamageAnalysis {
            element: element,
            lunar_type: lunar_type,

            atk: atk_comp.0,
            atk_ratio: atk_ratio_comp.0,
            hp: hp_comp.0,
            hp_ratio: hp_ratio_comp.0,
            def: def_comp.0,
            def_ratio: def_ratio_comp.0,
            em: em_comp.0,
            em_ratio: em_ratio_comp.0,
            reaction_base: reaction_base,
            reaction_coefficient: reaction_coefficient,

            reaction_enhance: enhance_comp.0,
            reaction_extra: extra_increase_comp.0,
            critical_rate: critical_comp.0,
            critical_damage: critical_damage_comp.0,
            res_minus: res_minus_comp.0,
            moonglare_base: increase_comp.0,
            moonglare_elevate: elevate_comp.0,

            result: damage
        })
    }

    fn heal(&self, attribute: &Self::AttributeType) -> Self::Result {
        let get_attribute_type = |variable: AttributeVariableType| -> AttributeType {
            AttributeType::Invisible(InvisibleAttributeType::new(
                variable,
                None,
                None,
                None,
            ))
        };

        let atk_comp = self.get_atk_composition(attribute);
        let atk = atk_comp.sum();
        let def_comp = self.get_def_composition(attribute);
        let def = def_comp.sum();
        let hp_comp = self.get_hp_composition(attribute);
        let hp = hp_comp.sum();
        let em_comp = self.get_em_composition(attribute);
        let em = em_comp.sum();

        let healing_bonus_comp = self.get_healing_bonus_composition(attribute)
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::HealingBonus)));
        let healing_bonus = healing_bonus_comp.sum();
        let incoming_healing_bonus_comp= self.get_incoming_healing_bonus_composition(attribute)
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::IncomingHealingBonus)));
        let incoming_healing_bonus = incoming_healing_bonus_comp.sum();
        let healing_critical_comp = &self.extra_critical_rate
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::HealingCriticalRate)));
        let healing_critical  = healing_critical_comp.sum().clamp(0.0, 1.0);
        let healing_critical_damage_comp = &self.extra_critical_damage
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::HealingCriticalDamage)));
        let healing_critical_damage = healing_critical_damage_comp.sum();

        let base = atk * self.ratio_atk.sum() + hp * self.ratio_hp.sum() + def * self.ratio_def.sum() + em * self.ratio_em.sum() + self.base.sum() + self.extra_damage.sum();

        let heal_value = base * (1.0 + healing_bonus) * (1.0 + incoming_healing_bonus);
        let heal = DamageResult {
            expectation: heal_value * (1.0 + healing_critical * healing_critical_damage),
            critical: heal_value * (1.0 + healing_critical_damage),
            non_critical: heal_value,
        };

        EventAnalysis::Heal(HealAnalysis {
            atk: atk_comp.0,
            atk_ratio: self.ratio_atk.0.clone(),
            hp: hp_comp.0,
            hp_ratio: self.ratio_hp.0.clone(),
            def: def_comp.0,
            def_ratio: self.ratio_def.0.clone(),
            em: em_comp.0,
            em_ratio: self.ratio_em.0.clone(),
            base: self.base.merge_with(&self.extra_damage).0.clone(),

            healing_bonus: healing_bonus_comp.0,
            incoming_healing_bonus: incoming_healing_bonus_comp.0,
            critical_rate: healing_critical_comp.0.clone(),
            critical_damage: healing_critical_damage_comp.0.clone(),

            result: heal,
        })
    }

    fn shield(&self, attribute: &Self::AttributeType, element: Element) -> Self::Result {
        let get_attribute_type = |variable: AttributeVariableType| -> AttributeType {
            AttributeType::Invisible(InvisibleAttributeType::new(
                variable,
                None,
                None,
                None,
            ))
        };

        let atk_comp = self.get_atk_composition(attribute);
        let atk = atk_comp.sum();
        let def_comp = self.get_def_composition(attribute);
        let def = def_comp.sum();
        let hp_comp = self.get_hp_composition(attribute);
        let hp = hp_comp.sum();
        let em_comp = self.get_em_composition(attribute);
        let em = em_comp.sum();

        let shield_strength_comp = self.get_shield_strength_composition(attribute)
            .merge_with(&attribute.get_result_t(get_attribute_type(AttributeVariableType::ShieldStrength)));
        let shield_strength = shield_strength_comp.sum();

        let base = atk * self.ratio_atk.sum() + hp * self.ratio_hp.sum() + def * self.ratio_def.sum() + em * self.ratio_em.sum() + self.base.sum() + self.extra_damage.sum();

        let shield_value = base * (1.0 + shield_strength);
        let shield = DamageResult {
            expectation: shield_value,
            critical: 0.0,
            non_critical: 0.0,
        };

        EventAnalysis::Shield(ShieldAnalysis {
            element: element,
            
            atk: atk_comp.0,
            atk_ratio: self.ratio_atk.0.clone(),
            hp: hp_comp.0,
            hp_ratio: self.ratio_hp.0.clone(),
            def: def_comp.0,
            def_ratio: self.ratio_def.0.clone(),
            em: em_comp.0,
            em_ratio: self.ratio_em.0.clone(),
            base: self.base.merge_with(&self.extra_damage).0.clone(),

            shield_strength: shield_strength_comp.0,

            result: shield,
        })
    }

    fn none(&self) -> Self::Result {
        EventAnalysis::None
    }
}

type AttributeTy = ComplicatedAttributeResult;

impl ComplicatedDamageBuilder {
    fn get_def_minus_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::DefMinus);
        comp.merge(&self.extra_def_minus);
        comp
    }

    fn get_def_penetration_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::DefPenetration);
        comp.merge(&self.extra_def_penetration);
        comp
    }

    fn get_res_minus_composition(&self, attribute: &AttributeTy, element: Element) -> EntryType {
        let mut comp = attribute.get_result_merge(&vec![
            AttributeName::ResMinusBase,
            AttributeName::res_minus_name_by_element(element)
        ]);
        comp.merge(&self.extra_res_minus);
        comp
    }

    fn get_extra_damage_composition(&self, attribute: &AttributeTy, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::ExtraDmgBase,
            AttributeName::extra_dmg_name_by_element(element),
        ];
        if let Some(name) = AttributeName::extra_dmg_name_by_skill_type(skill) {
            names.push(name);
        }
        if skill == SkillType::PlungingAttackOnGround {
            names.push(AttributeName::ExtraDmgPlungingAttackLowHigh);
        }
        let mut comp = attribute.get_result_merge(&names);
        comp.merge(&self.extra_damage);
        comp
    }

    fn get_healing_bonus_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::HealingBonus);
        comp.merge(&self.extra_healing_bonus);
        comp
    }

    fn get_incoming_healing_bonus_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::IncomingHealingBonus);
        comp
    }

    fn get_shield_strength_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::ShieldStrength);
        comp
    }

    fn get_enhance_melt_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::EnhanceMelt);
        comp.merge(&self.extra_enhance_melt);
        let em = self.extra_em.sum() + attribute.get_em_all();
        if em > 0.0 {
            comp.add_value("精通", Reaction::amp(em));
        }
        comp
    }

    fn get_enhance_vaporize_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::EnhanceVaporize);
        comp.merge(&self.extra_enhance_vaporize);
        let em = self.extra_em.sum() + attribute.get_em_all();
        if em > 0.0 {
            comp.add_value("精通", Reaction::amp(em));
        }
        comp
    }

    fn get_enhance_spread_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::EnhanceSpread);
        let em = &self.extra_em.sum() + attribute.get_em_all();
        if em > 0.0 {
            comp.add_value("精通", Reaction::catalyze(em));
        }
        comp
    }

    fn get_enhance_aggravate_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::EnhanceAggravate);
        let em = &self.extra_em.sum() + attribute.get_em_all();
        if em > 0.0 {
            comp.add_value("精通", Reaction::catalyze(em));
        }
        comp
    }

    fn get_enhance_transformative_composition(&self, attribute: &AttributeTy, transformative_type: TransformativeType) -> EntryType {
        let mut comp = match transformative_type {
            TransformativeType::SwirlCryo => attribute.get_result(AttributeName::EnhanceSwirlCryo).merge_with(&attribute.get_result(AttributeName::EnhanceSwirlBase)),
            TransformativeType::SwirlPyro => attribute.get_result(AttributeName::EnhanceSwirlPyro).merge_with(&attribute.get_result(AttributeName::EnhanceSwirlBase)),
            TransformativeType::SwirlHydro => attribute.get_result(AttributeName::EnhanceSwirlHydro).merge_with(&attribute.get_result(AttributeName::EnhanceSwirlBase)),
            TransformativeType::SwirlElectro => attribute.get_result(AttributeName::EnhanceSwirlElectro).merge_with(&attribute.get_result(AttributeName::EnhanceSwirlBase)),
            TransformativeType::Superconduct => attribute.get_result(AttributeName::EnhanceSuperconduct),
            TransformativeType::Overload => attribute.get_result(AttributeName::EnhanceOverload),
            TransformativeType::Burning => attribute.get_result(AttributeName::EnhanceBurning),
            TransformativeType::ElectroCharged => attribute.get_result(AttributeName::EnhanceElectroCharged),
            TransformativeType::Shatter => attribute.get_result(AttributeName::EnhanceShatter),
            TransformativeType::Bloom => attribute.get_result(AttributeName::EnhanceBloom),
            TransformativeType::Burgeon => attribute.get_result(AttributeName::EnhanceBurgeon),
            TransformativeType::Hyperbloom => attribute.get_result(AttributeName::EnhanceHyperbloom),
            TransformativeType::Crystallize => EntryType::new(),
        };
        let em = &self.extra_em.sum() + attribute.get_em_all();
        if em > 0.0 {
            comp.add_value("精通", Reaction::transformative(em));
        }
        comp
    }

    fn get_enhance_moonglare_composition(&self, attribute: &AttributeTy, lunar_type: MoonglareReaction) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::enhance_name_by_moonglare_reaction(lunar_type).unwrap_or(AttributeName::NULL));
        comp.merge(&attribute.get_result(AttributeName::EnhanceMoonglare));
        let em = &self.extra_em.sum() + attribute.get_em_all();
        if em > 0.0 {
            comp.add_value("精通", Reaction::moonglare(em));
        }
        comp
    }

    fn get_extra_increase_reaction_composition(&self, attribute: &AttributeTy, reaction_type: ReactionType) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::extra_increase_name_by_reaction(reaction_type).unwrap_or(AttributeName::NULL));
        comp
    }

    fn get_increase_moonglare_composition(&self, attribute: &AttributeTy, lunar_type: MoonglareReaction) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::increase_name_by_moonglare_reaction(lunar_type).unwrap_or(AttributeName::NULL));
        comp
    }

    fn get_elevate_moonglare_composition(&self, attribute: &AttributeTy, lunar_type: MoonglareReaction) -> EntryType {
        let mut comp = attribute.get_result(AttributeName::elevate_name_by_moonglare_reaction(lunar_type).unwrap_or(AttributeName::NULL));
        comp
    }

    fn get_critical_damage_composition(&self, attribute: &AttributeTy, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::CriticalDamageBase,
            AttributeName::critical_damage_name_by_element(element),
        ];
        if let Some(name) = AttributeName::critical_damage_name_by_skill_name(skill) {
            names.push(name);
        }
        let mut comp = attribute.get_result_merge(&names);
        comp.merge(&self.extra_critical_damage);
        comp
    }

    fn get_critical_damage_moonglare_composition(&self, attribute: &AttributeTy, element: Element, lunar_type: MoonglareReaction, skill: SkillType) -> EntryType {
        let mut comp = self.get_critical_damage_composition(attribute, element, skill);
        comp.merge(&attribute.get_result(AttributeName::critical_damage_name_by_moonglare_reaction(lunar_type).unwrap()));
        comp
    }

    fn get_critical_composition(&self, attribute: &AttributeTy, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::CriticalBase,
            AttributeName::critical_rate_name_by_element(element),
        ];
        if let Some(name) = AttributeName::critical_rate_name_by_skill_type(skill) {
            names.push(name);
        }
        let mut comp = attribute.get_result_merge(&names);
        comp.merge(&self.extra_critical_rate);

        comp
    }

    fn get_critical_moonglare_composition(&self, attribute: &AttributeTy, element: Element, lunar_type: MoonglareReaction, skill: SkillType) -> EntryType {
        let mut comp = self.get_critical_composition(attribute, element, skill);
        comp.merge(&attribute.get_result(AttributeName::critical_rate_name_by_moonglare_reaction(lunar_type).unwrap()));

        comp
    }

    fn get_bonus_composition(&self, attribute: &AttributeTy, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::BonusBase,
            AttributeName::bonus_name_by_element(element),
        ];
        if let Some(name) = AttributeName::bonus_name_by_skill_type(skill) {
            names.push(name);
        }
        let mut comp = attribute.get_result_merge(&names);
        if element != Element::Physical && skill == SkillType::NormalAttack {
            // todo refactor
            comp.merge(&attribute.get_result(AttributeName::BonusNormalAndElemental));
        }
        comp.merge(&self.extra_bonus);
        comp
    }

    fn get_atk_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut atk_comp =
            attribute.get_result_merge(&vec![AttributeName::ATKBase, AttributeName::ATKPercentage, AttributeName::ATKFixed]);
        atk_comp.merge(&self.extra_atk);

        atk_comp
    }

    fn get_atk_ratio_composition(&self, attribute: &AttributeTy, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::ATKRatioBase,
            AttributeName::atk_ratio_name_by_element(element),
        ];
        if let Some(name) = AttributeName::atk_ratio_name_by_skill_type(skill) {
            names.push(name)
        }
        let mut atk_ratio_comp = attribute.get_result_merge(&names);
        atk_ratio_comp.merge(&self.ratio_atk);

        atk_ratio_comp
    }

    fn get_def_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut def_comp = attribute.get_result_merge(&vec![
            AttributeName::DEFBase,
            AttributeName::DEFPercentage,
            AttributeName::DEFFixed
        ]);
        def_comp.merge(&self.extra_def);

        def_comp
    }

    fn get_def_ratio_composition(&self, attribute: &AttributeTy, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::DEFRatioBase,
            AttributeName::def_ratio_name_by_element(element),
        ];
        if let Some(name) = AttributeName::def_ratio_name_by_skill_type(skill) {
            names.push(name);
        }
        let mut def_ratio_comp = attribute.get_result_merge(&names);
        def_ratio_comp.merge(&self.ratio_def);

        def_ratio_comp
    }

    fn get_hp_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut hp_comp = attribute.get_result_merge(&vec![
            AttributeName::HPBase,
            AttributeName::HPPercentage,
            AttributeName::HPFixed
        ]);
        hp_comp.merge(&self.extra_hp);

        hp_comp
    }

    fn get_em_composition(&self, attribute: &AttributeTy) -> EntryType {
        let mut em_comp = attribute.get_result_merge(&vec![
            AttributeName::ElementalMastery,
            AttributeName::ElementalMasteryExtra,
        ]);
        em_comp.merge(&self.extra_em);
        em_comp
    }

    fn get_hp_ratio_composition(&self, attribute: &AttributeTy, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::HPRatioBase,
            AttributeName::hp_ratio_name_by_element(element),
        ];
        if let Some(name) = AttributeName::hp_ratio_name_by_skill_type(skill) {
            names.push(name)
        }
        let mut hp_ratio_comp = attribute.get_result_merge(&names);
        hp_ratio_comp.merge(&self.ratio_hp);

        hp_ratio_comp
    }

    fn get_em_ratio_composition(&self, attribute: &AttributeTy, element: Element, skill: SkillType) -> EntryType {
        let mut em_ratio_comp = attribute.get_result_merge(&vec![
            // todo
        ]);
        em_ratio_comp.merge(&self.ratio_em);
        em_ratio_comp
    }
}