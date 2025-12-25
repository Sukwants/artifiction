use crate::attribute::*;
use crate::common::{element, DamageResult, Element, MoonglareReaction, SkillType, ReactionType, TransformativeType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::damage_result::SimpleDamageResult;
use crate::damage::level_coefficient::LEVEL_MULTIPLIER;
use crate::damage::reaction::Reaction;
use crate::enemies::Enemy;

pub struct SimpleDamageBuilder {
    pub extra_critical_damage: f64,
    pub extra_critical_rate: f64,
    pub extra_bonus: f64,
    pub extra_damage: f64,
    pub extra_atk: f64,
    pub extra_def: f64,
    pub extra_hp: f64,

    pub extra_def_minus: f64,
    pub extra_res_minus: f64,
    pub extra_def_penetration: f64,

    pub ratio_atk: f64, // 技能倍率，（应该）不包含基础伤害加成
    pub ratio_def: f64,
    pub ratio_hp: f64,
    pub ratio_em: f64,
    pub base: f64,

    pub extra_enhance_melt: f64,
    pub extra_enhance_vaporize: f64,
    pub enhance_melt: f64,
    pub enhance_vaporize: f64,
    pub extra_em: f64,
}

impl DamageBuilder for SimpleDamageBuilder {
    type Result = SimpleDamageResult;
    type AttributeType = SimpleAttributeResult;

    fn new() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    fn add_em_ratio(&mut self, _key: &str, value: f64) {
        self.ratio_em += value;
    }

    fn add_atk_ratio(&mut self, _key: &str, value: f64) {
        self.ratio_atk += value
    }

    fn add_def_ratio(&mut self, _key: &str, value: f64) {
        self.ratio_def += value
    }

    fn add_hp_ratio(&mut self, _key: &str, value: f64) {
        self.ratio_hp += value
    }

    fn add_base(&mut self, key: &str, value: f64) {
        self.base += value
    }

    fn add_extra_em(&mut self, _key: &str, value: f64) {
        self.extra_em += value;
    }

    fn add_extra_atk(&mut self, _key: &str, value: f64) {
        self.extra_atk += value
    }

    fn add_extra_def(&mut self, _key: &str, value: f64) {
        self.extra_def += value
    }

    fn add_extra_hp(&mut self, _key: &str, value: f64) {
        self.extra_hp += value
    }

    fn add_extra_damage(&mut self, _key: &str, value: f64) {
        self.extra_damage += value
    }

    fn add_extra_critical(&mut self, _key: &str, value: f64) {
        self.extra_critical_rate += value
    }

    fn add_extra_critical_damage(&mut self, _key: &str, value: f64) {
        self.extra_critical_damage += value
    }

    fn add_extra_bonus(&mut self, _key: &str, value: f64) {
        self.extra_bonus += value
    }

    fn add_extra_enhance_melt(&mut self, _key: &str, value: f64) {
        self.extra_enhance_melt += value
    }

    fn add_extra_enhance_vaporize(&mut self, _key: &str, value: f64) {
        self.extra_enhance_vaporize += value
    }

    fn add_extra_def_minus(&mut self, _key: &str, value: f64) {
        self.extra_def_minus += value
    }

    fn add_extra_def_penetration(&mut self, _key: &str, value: f64) {
        self.extra_def_penetration += value;
    }

    fn add_extra_res_minus(&mut self, _key: &str, value: f64) {
        self.extra_res_minus += value
    }

    fn damage(&self, attribute: &Self::AttributeType, enemy: &Enemy, element: Element, skill: SkillType, character_level: usize, fumo: Option<Element>) -> Self::Result {
        let atk = attribute.get_atk() + self.extra_atk;
        let def = attribute.get_def() + self.extra_def;
        let hp = attribute.get_hp() + self.extra_hp;
        let em = self.extra_em + attribute.get_em_all();

        let element = if skill == SkillType::NormalAttack || skill == SkillType::ChargedAttack || skill.is_plunging() {
            if let Some(x) = fumo {
                x
            } else {
                element
            }
        } else {
            element
        };

        let get_damage = |reaction: Option<ReactionType>| -> DamageResult {
            let get_attribute_type = |variable: AttributeVariableType| -> AttributeType {
                AttributeType::Invisible(InvisibleAttributeType::new(
                    variable,
                    Some(element),
                    Some(skill),
                    reaction,
                ))
            };

            let base_damage
                = (attribute.get_def_ratio(element, skill) + self.ratio_def) * def
                + (attribute.get_hp_ratio(element, skill) + self.ratio_hp) * hp
                + (attribute.get_atk_ratio(element, skill) + self.ratio_atk) * atk
                + em * self.ratio_em
                + attribute.get_extra_damage(element, skill)
                + self.extra_damage
                + attribute.get_result_t(get_attribute_type(AttributeVariableType::BaseDamage));

            let bonus
                = attribute.get_bonus(element, skill)
                + self.extra_bonus
                + attribute.get_result_t(get_attribute_type(AttributeVariableType::Bonus));

            let critical_rate
                = attribute.get_critical_rate(element, skill)
                + self.extra_critical_rate
                + attribute.get_result_t(get_attribute_type(AttributeVariableType::CriticalRate));
            let critical_rate = critical_rate.clamp(0.0, 1.0);

            let critical_damage
                = attribute.get_critical_damage(element, skill)
                + self.extra_critical_damage
                + attribute.get_result_t(get_attribute_type(AttributeVariableType::CriticalDamage));
            
            let defensive_ratio = {
                let def_minus = self.extra_def_minus + attribute.get_enemy_def_minus(element, skill)
                    + attribute.get_result_t(get_attribute_type(AttributeVariableType::DefMinus));
                let def_penetration = self.extra_def_penetration + attribute.get_value(AttributeName::DefPenetration)
                    + attribute.get_result_t(get_attribute_type(AttributeVariableType::DefPenetration));
                enemy.get_defensive_ratio(character_level, def_minus, def_penetration)
            };
            let resistance_ratio = {
                let res_minus = self.extra_res_minus + attribute.get_enemy_res_minus(element, skill)
                    + attribute.get_result_t(get_attribute_type(AttributeVariableType::ResMinus));
                enemy.get_resistance_ratio(element, res_minus)
            };

            let reaction_enhance = match reaction {
                Some(ReactionType::Melt) => Reaction::amp(em) + attribute.get_result(AttributeName::EnhanceMelt) + self.extra_enhance_melt,
                Some(ReactionType::Vaporize) => Reaction::amp(em) + attribute.get_result(AttributeName::EnhanceVaporize) + self.extra_enhance_vaporize,
                Some(ReactionType::Spread) => Reaction::catalyze(em) + attribute.get_result(AttributeName::EnhanceSpread),
                Some(ReactionType::Aggravate) => Reaction::catalyze(em) + attribute.get_result(AttributeName::EnhanceAggravate),
                _ => 0.0
            } + attribute.get_result_t(get_attribute_type(AttributeVariableType::ReactionEnhance));

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
                Some(ReactionType::Spread) => 1.25,
                Some(ReactionType::Aggravate) => 1.15,
                _ => 1.0
            };

            match reaction {
                None => DamageResult {
                    expectation: base_damage * (1.0 + bonus) * (1.0 + critical_rate * critical_damage),
                    critical: base_damage * (1.0 + bonus) * (1.0 + critical_damage),
                    non_critical: base_damage * (1.0 + bonus),
                } * (defensive_ratio * resistance_ratio),
                Some(ReactionType::Melt) | Some(ReactionType::Vaporize) => DamageResult {
                    expectation: base_damage * (1.0 + bonus) * (1.0 + critical_rate * critical_damage),
                    critical: base_damage * (1.0 + bonus) * (1.0 + critical_damage),
                    non_critical: base_damage * (1.0 + bonus),
                } * (defensive_ratio * resistance_ratio) * reaction_coefficient * (1.0 + reaction_enhance),
                Some(ReactionType::Spread) | Some(ReactionType::Aggravate) => {
                    let reaction_base_damage = base_damage + LEVEL_MULTIPLIER[character_level - 1] * reaction_coefficient * (1.0 + reaction_enhance);
                    DamageResult {
                        critical: reaction_base_damage * (1.0 + bonus) * (1.0 + critical_damage),
                        non_critical: reaction_base_damage * (1.0 + bonus),
                        expectation: reaction_base_damage * (1.0 + bonus) * (1.0 + critical_damage * critical_rate),
                    } * (defensive_ratio * resistance_ratio)
                },
                _ => panic!()
            }
        };

        SimpleDamageResult::new(
            get_damage(None),
            if element == Element::Pyro || element == Element::Cryo {
                Some(get_damage(Some(ReactionType::Melt)))
            } else { None },
            if element == Element::Pyro || element == Element::Hydro {
                Some(get_damage(Some(ReactionType::Vaporize)))
            } else { None },
            if element == Element::Dendro {
                Some(get_damage(Some(ReactionType::Spread)))
            } else { None },
            if element == Element::Electro {
                Some(get_damage(Some(ReactionType::Aggravate)))
            } else { None },
        )
    }

    fn transformative(&self, attribute: &Self::AttributeType, enemy: &Enemy, transformative_type: TransformativeType, character_level: usize) -> Self::Result {
        let em = self.extra_em + attribute.get_em_all();

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

        let critical_rate = self.extra_critical_rate
            + attribute.get_result_t(get_attribute_type(AttributeVariableType::CriticalRate));
        let critical_rate = critical_rate.clamp(0.0, 1.0);

        let critical_damage = self.extra_critical_damage
            + attribute.get_result_t(get_attribute_type(AttributeVariableType::CriticalDamage));

        let resistance_ratio = if transformative_type != TransformativeType::Crystallize {
            let res_minus = self.extra_res_minus + attribute.get_enemy_res_minus(element.unwrap(), SkillType::NoneType)
                + attribute.get_result_t(get_attribute_type(AttributeVariableType::ResMinus));
            enemy.get_resistance_ratio(element.unwrap(), res_minus)
        } else { 1.0 };

        let enhance = Reaction::transformative(em) + match transformative_type {
            TransformativeType::SwirlCryo => attribute.get_value(AttributeName::EnhanceSwirlCryo) + attribute.get_value(AttributeName::EnhanceSwirlBase),
            TransformativeType::SwirlPyro => attribute.get_value(AttributeName::EnhanceSwirlPyro) + attribute.get_value(AttributeName::EnhanceSwirlBase),
            TransformativeType::SwirlHydro => attribute.get_value(AttributeName::EnhanceSwirlHydro) + attribute.get_value(AttributeName::EnhanceSwirlBase),
            TransformativeType::SwirlElectro => attribute.get_value(AttributeName::EnhanceSwirlElectro) + attribute.get_value(AttributeName::EnhanceSwirlBase),
            TransformativeType::Superconduct => attribute.get_value(AttributeName::EnhanceSuperconduct),
            TransformativeType::Overload => attribute.get_value(AttributeName::EnhanceOverload),
            TransformativeType::Burning => attribute.get_value(AttributeName::EnhanceBurning),
            TransformativeType::ElectroCharged => attribute.get_value(AttributeName::EnhanceElectroCharged),
            TransformativeType::Shatter => attribute.get_value(AttributeName::EnhanceShatter),
            TransformativeType::Bloom => attribute.get_value(AttributeName::EnhanceBloom),
            TransformativeType::Burgeon => attribute.get_value(AttributeName::EnhanceBurgeon),
            TransformativeType::Hyperbloom => attribute.get_value(AttributeName::EnhanceHyperbloom),
            TransformativeType::Crystallize => 0.0,
        } + attribute.get_result_t(get_attribute_type(AttributeVariableType::ReactionEnhance));
        
        let extra_increase = attribute.get_result(AttributeName::extra_increase_name_by_reaction(reaction).unwrap_or(AttributeName::NULL))
            + attribute.get_result_t(get_attribute_type(AttributeVariableType::ReactionExtra));

        let damage = {
            let dmg = base_damage * (1.0 + enhance) + extra_increase;
            DamageResult {
                critical: dmg * (1.0 + critical_damage),
                non_critical: dmg,
                expectation: dmg * (1.0 + critical_damage * critical_rate),
            } * resistance_ratio
        };

        SimpleDamageResult::new_normal(damage)
    }

    fn moonglare(&self, attribute: &Self::AttributeType, enemy: &Enemy, element: Element, lunar_type: MoonglareReaction, skill: SkillType, character_level: usize, fumo: Option<Element>) -> Self::Result {
        let atk = attribute.get_atk() + self.extra_atk;
        let def = attribute.get_def() + self.extra_def;
        let hp = attribute.get_hp() + self.extra_hp;
        let em = self.extra_em + attribute.get_em_all();

        let reaction = ReactionType::get_reaction_from_lunar_type(lunar_type).unwrap();
        
        let get_attribute_type = |variable: AttributeVariableType| -> AttributeType {
            AttributeType::Invisible(InvisibleAttributeType::new(
                variable,
                Some(element),
                Some(skill),
                Some(reaction),
            ))
        };

        let base_damage // without em bonus
            = atk * self.ratio_atk
            + def * self.ratio_def
            + hp * self.ratio_hp
            + em * self.ratio_em
            // + attribute.get_extra_damage(element, skill)
            + self.extra_damage // This line is unused for now
            + attribute.get_result_t(get_attribute_type(AttributeVariableType::BaseDamage));

        let critical_rate
            = attribute.get_critical_rate(element, skill)
            + self.extra_critical_rate
            + attribute.get_value(AttributeName::critical_rate_name_by_moonglare_reaction(lunar_type).unwrap())
            + attribute.get_result_t(get_attribute_type(AttributeVariableType::CriticalRate));
        let critical_rate = critical_rate.clamp(0.0, 1.0);

        let critical_damage
            = attribute.get_critical_damage(element, skill)
            + self.extra_critical_damage
            + attribute.get_value(AttributeName::critical_damage_name_by_moonglare_reaction(lunar_type).unwrap())
            + attribute.get_result_t(get_attribute_type(AttributeVariableType::CriticalDamage));

        let resistance_ratio = {
            let res_minus = self.extra_res_minus + attribute.get_enemy_res_minus(element, skill)
                + attribute.get_result_t(get_attribute_type(AttributeVariableType::ResMinus));
            enemy.get_resistance_ratio(element, res_minus)
        };

        let enhance = Reaction::moonglare(em) + attribute.get_value(AttributeName::EnhanceMoonglare) + match lunar_type {
            MoonglareReaction::LunarChargedReaction | MoonglareReaction::LunarCharged => attribute.get_value(AttributeName::EnhanceLunarCharged),
            MoonglareReaction::LunarBloom => attribute.get_value(AttributeName::EnhanceLunarBloom),
            _ => 0.0
        } + attribute.get_result_t(get_attribute_type(AttributeVariableType::ReactionEnhance));
        
        let extra_increase = match lunar_type {
            MoonglareReaction::LunarChargedReaction | MoonglareReaction::LunarCharged => attribute.get_value(AttributeName::ExtraIncreaseLunarCharged),
            MoonglareReaction::LunarBloom => attribute.get_value(AttributeName::ExtraIncreaseLunarBloom),
            _ => 0.0
        } + attribute.get_result_t(get_attribute_type(AttributeVariableType::ReactionExtra));

        let increase = match lunar_type {
            MoonglareReaction::LunarChargedReaction | MoonglareReaction::LunarCharged => attribute.get_value(AttributeName::IncreaseLunarCharged),
            MoonglareReaction::LunarBloom => attribute.get_value(AttributeName::IncreaseLunarBloom),
            _ => 0.0
        } + attribute.get_result_t(get_attribute_type(AttributeVariableType::MoonglareBase));

        let elevate = match lunar_type {
            MoonglareReaction::LunarChargedReaction | MoonglareReaction::LunarCharged => attribute.get_value(AttributeName::ElevateLunarCharged),
            MoonglareReaction::LunarBloom => attribute.get_value(AttributeName::ElevateLunarBloom),
            _ => 0.0
        } + attribute.get_result_t(get_attribute_type(AttributeVariableType::MoonglareElevate));

        let reaction_base = LEVEL_MULTIPLIER[character_level - 1];
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
                expectation: dmg * (1.0 + critical_damage * critical_rate),
            } * resistance_ratio * (1.0 + elevate)
        };

        SimpleDamageResult::new_normal(damage)
    }

    fn heal(&self, attribute: &Self::AttributeType) -> Self::Result {
        let atk = attribute.get_atk() + self.extra_atk;
        let def = attribute.get_def() + self.extra_def;
        let hp = attribute.get_hp() + self.extra_hp;

        let get_attribute_type = |variable: AttributeVariableType| -> AttributeType {
            AttributeType::Invisible(InvisibleAttributeType::new(
                variable,
                None,
                None,
                None,
            ))
        };

        let base = self.ratio_def * def + self.ratio_hp * hp + self.ratio_atk * atk + self.base + self.extra_damage;

        let healing_bonus = attribute.get_value(AttributeName::HealingBonus)
            + attribute.get_result_t(get_attribute_type(AttributeVariableType::HealingBonus));
        let incoming_healing_bonus = attribute.get_value(AttributeName::IncomingHealingBonus)
            + attribute.get_result_t(get_attribute_type(AttributeVariableType::IncomingHealingBonus));
        let healing_critical = self.extra_critical_rate.clamp(0.0, 1.0)
            + attribute.get_result_t(get_attribute_type(AttributeVariableType::HealingCriticalRate));
        let healing_critical_bonus = self.extra_critical_damage
            + attribute.get_result_t(get_attribute_type(AttributeVariableType::HealingCriticalDamage));
        let heal_value = base * (1.0 + healing_bonus) * (1.0 + incoming_healing_bonus);

        let damage = {
            DamageResult {
                critical: heal_value * (1.0 + healing_critical_bonus),
                non_critical: heal_value,
                expectation: heal_value * (1.0 + healing_critical * healing_critical_bonus),
            }
        };

        SimpleDamageResult::new_normal(damage)
    }

    fn shield(&self, attribute: &Self::AttributeType, _element: Element) -> Self::Result {
        let atk = attribute.get_atk() + self.extra_atk;
        let def = attribute.get_def() + self.extra_def;
        let hp = attribute.get_hp() + self.extra_hp;

        let get_attribute_type = |variable: AttributeVariableType| -> AttributeType {
            AttributeType::Invisible(InvisibleAttributeType::new(
                variable,
                None,
                None,
                None,
            ))
        };

        let base = self.ratio_def * def + self.ratio_hp * hp + self.ratio_atk * atk + self.base + self.extra_damage;

        let shield_strength = attribute.get_value(AttributeName::ShieldStrength)
            + attribute.get_result_t(get_attribute_type(AttributeVariableType::ShieldStrength));
        let shield_value = base * (1.0 + shield_strength);

        let damage = {
            DamageResult {
                critical: shield_value,
                non_critical: shield_value,
                expectation: shield_value,
            }
        };

        SimpleDamageResult::new_normal(damage)
    }

    fn none(&self) -> Self::Result {
        SimpleDamageResult::new_normal(DamageResult::default())
    }
}

impl SimpleDamageBuilder {
    pub fn new(ratio_atk: f64, ratio_def: f64, ratio_hp: f64) -> SimpleDamageBuilder {
        SimpleDamageBuilder {
            extra_critical_damage: 0.0,
            extra_critical_rate: 0.0,
            extra_bonus: 0.0,
            extra_damage: 0.0,
            extra_atk: 0.0,
            extra_def: 0.0,
            extra_hp: 0.0,

            extra_def_minus: 0.0,
            extra_def_penetration: 0.0,
            extra_res_minus: 0.0,

            ratio_atk,
            ratio_hp,
            ratio_def,
            ratio_em: 0.0,
            base: 0.0,

            extra_enhance_melt: 0.0,
            extra_enhance_vaporize: 0.0,
            enhance_melt: 0.0,
            enhance_vaporize: 0.0,
            extra_em: 0.0
        }
    }
}
