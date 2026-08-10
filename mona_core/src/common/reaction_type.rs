use serde::{Serialize, Deserialize};
use crate::attribute::*;
use crate::common::{Element, SkillType};
use crate::damage::level_coefficient::{LEVEL_MULTIPLIER, CRYSTALLIZE_BASE};

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone)]
pub enum TransformativeType {
    SwirlCryo,
    SwirlPyro,
    SwirlHydro,
    SwirlElectro,
    Overload,
    ElectroCharged,
    Shatter,
    Superconduct,
    Bloom,
    Burgeon,
    Hyperbloom,
    Burning,
    Crystallize,
}

impl TransformativeType {
    pub fn get_element(&self) -> Option<Element> {
        match *self {
            TransformativeType::SwirlCryo => Some(Element::Cryo),
            TransformativeType::SwirlHydro => Some(Element::Hydro),
            TransformativeType::SwirlElectro => Some(Element::Electro),
            TransformativeType::SwirlPyro => Some(Element::Pyro),
            TransformativeType::Superconduct => Some(Element::Cryo),
            TransformativeType::Overload => Some(Element::Pyro),
            TransformativeType::Burning => Some(Element::Pyro),
            TransformativeType::ElectroCharged => Some(Element::Electro),
            TransformativeType::Shatter => Some(Element::Physical),
            TransformativeType::Bloom | TransformativeType::Burgeon | TransformativeType::Hyperbloom => Some(Element::Dendro),
            TransformativeType::Crystallize => None,
        }
    }

    pub fn get_reaction_base(&self, character_level: usize) -> f64 {
        match *self {
            TransformativeType::Crystallize => CRYSTALLIZE_BASE[character_level - 1],
            _ => LEVEL_MULTIPLIER[character_level - 1],
        }
    }

    pub fn get_reaction_coefficient(&self) -> f64 {
        match *self {
            TransformativeType::SwirlCryo | TransformativeType::SwirlPyro | TransformativeType::SwirlHydro | TransformativeType::SwirlElectro => 0.6,
            TransformativeType::Superconduct => 1.5,
            TransformativeType::Overload => 2.75,
            TransformativeType::Burning => 0.25,
            TransformativeType::ElectroCharged => 2.0,
            TransformativeType::Shatter => 3.0,
            TransformativeType::Bloom => 2.0,
            TransformativeType::Burgeon => 3.0,
            TransformativeType::Hyperbloom => 3.0,
            TransformativeType::Crystallize => 1.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone)]
pub enum ElevativeReaction {
    LunarChargedReaction,       // 月感电
    LunarCharged,               // 广义月感电
    LunarBloom,
    LunarCrystallizeReaction,   // 月结晶
    LunarCrystallize,           // 广义月结晶
    StellarConductCryo,         // 视为星超导反应伤害的冰元素伤害
    StellarConductElectro,      // 视为星超导反应伤害的雷元素伤害
    StellarSwirlReactionAnemo,  // 星扩散反应风伤
    StellarSwirlReactionCryo,   // 星扩散反应冰伤
    StellarSwirlAnemo,          // 视为星扩散伤害的风元素伤害
    StellarSwirlCryo,           // 视为星扩散伤害的冰元素伤害
}

impl ElevativeReaction {
    pub fn get_element(&self) -> Option<Element> {
        match *self {
            ElevativeReaction::LunarChargedReaction | ElevativeReaction::LunarCharged => Some(Element::Electro),
            ElevativeReaction::LunarBloom => Some(Element::Dendro),
            ElevativeReaction::LunarCrystallizeReaction | ElevativeReaction::LunarCrystallize => Some(Element::Geo),
            ElevativeReaction::StellarSwirlReactionAnemo | ElevativeReaction::StellarSwirlAnemo => Some(Element::Anemo),
            ElevativeReaction::StellarSwirlReactionCryo | ElevativeReaction::StellarSwirlCryo => Some(Element::Cryo),
            _ => None
        }
    }

    pub fn get_reaction_base(&self, character_level: usize) -> f64 {
        match *self {
            ElevativeReaction::LunarChargedReaction | ElevativeReaction::LunarCrystallizeReaction
            | ElevativeReaction::StellarSwirlReactionAnemo | ElevativeReaction::StellarSwirlReactionCryo => LEVEL_MULTIPLIER[character_level - 1],
            _ => 0.0,
        }
    }

    pub fn get_reaction_coefficient(&self) -> f64 {
        match *self {
            ElevativeReaction::LunarChargedReaction => 1.8,
            ElevativeReaction::LunarCharged => 3.0,
            ElevativeReaction::LunarBloom => 1.0,
            ElevativeReaction::LunarCrystallizeReaction => 0.96,
            ElevativeReaction::LunarCrystallize => 1.6,
            ElevativeReaction::StellarConductCryo => 1.0,
            ElevativeReaction::StellarConductElectro => 1.0,
            ElevativeReaction::StellarSwirlReactionAnemo => 0.75,
            ElevativeReaction::StellarSwirlReactionCryo => 2.0,
            ElevativeReaction::StellarSwirlAnemo => 1.0,
            ElevativeReaction::StellarSwirlCryo => 1.0,
        }
    }

    pub const STELLAR_CONDUCT_EXTRA_COEFFICIENT: [f64; 13] = [0.0, 0.45, 0.5, 0.54, 0.6, 0.64, 0.71, 0.75, 0.79, 0.85, 0.89, 0.95, 1.0];
    pub fn apply_stellar_conduct_extra_coefficient<A: Attribute>(attribute: &mut A, application_count: usize) {
        let coefficient = ElevativeReaction::STELLAR_CONDUCT_EXTRA_COEFFICIENT[application_count];
        attribute.set_value_to_t(
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                AttributeVariableType::ElevativeCoefficient,
                ReactionType::StellarConduct,
            )),
            "星超导附着次数",
            coefficient,
        );
    }

    // 星扩散反应冰伤的基础倍率随风涡系数的不同而变化：
    // - 风涡系数为 1~2 时，基础倍率为 2；
    // - 风涡系数为 3~6 时，基础倍率为 3。
    // 这里存储的是在基准基础倍率 2.0 之上的额外基础倍率（1~2 为 0.0，3~6 为 0.5），
    // 以与 `STELLAR_CONDUCT_EXTRA_COEFFICIENT` 的写法保持一致，
    // 数组下标对应风涡系数 0~6。
    pub const STELLAR_SWIRL_CRYO_EXTRA_COEFFICIENT: [f64; 7] = [0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5];
    pub fn apply_stellar_swirl_cryo_coefficient<A: Attribute>(attribute: &mut A, vortex_count: usize) {
        let coefficient = ElevativeReaction::STELLAR_SWIRL_CRYO_EXTRA_COEFFICIENT[vortex_count.min(6)];
        attribute.set_value_to_t(
            AttributeType::Invisible(InvisibleAttributeType::new(
                AttributeVariableType::ElevativeCoefficient,
                Some(Element::Cryo),
                Some(SkillType::Elevative),
                Some(ReactionType::StellarSwirl),
            )),
            "星扩散风涡系数",
            coefficient,
        );
    }
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone)]
pub enum ReactionType {
    Melt,
    Vaporize,
    Spread,
    Aggravate,
    CryoSwirl,
    PyroSwirl,
    HydroSwirl,
    ElectroSwirl,
    Superconduct,
    Overload,
    Burning,
    ElectroCharged,
    Shatter,
    Bloom,
    Burgeon,
    Hyperbloom,
    Crystallize,
    LunarCharged,
    LunarBloom,
    LunarCrystallize,
    StellarConduct,
    StellarSwirl,
}

impl ReactionType {
    pub fn get_lunar_reaction_list() -> Vec<ReactionType> {
        vec![
            ReactionType::LunarCharged,
            ReactionType::LunarBloom,
            ReactionType::LunarCrystallize,
        ]
    }
    
    pub fn get_reaction_from_transformative_type(transformative_type: TransformativeType) -> ReactionType {
        match transformative_type {
            TransformativeType::SwirlCryo => ReactionType::CryoSwirl,
            TransformativeType::SwirlPyro => ReactionType::PyroSwirl,
            TransformativeType::SwirlHydro => ReactionType::HydroSwirl,
            TransformativeType::SwirlElectro => ReactionType::ElectroSwirl,
            TransformativeType::Superconduct => ReactionType::Superconduct,
            TransformativeType::Overload => ReactionType::Overload,
            TransformativeType::Burning => ReactionType::Burning,
            TransformativeType::ElectroCharged => ReactionType::ElectroCharged,
            TransformativeType::Shatter => ReactionType::Shatter,
            TransformativeType::Bloom => ReactionType::Bloom,
            TransformativeType::Burgeon => ReactionType::Burgeon,
            TransformativeType::Hyperbloom => ReactionType::Hyperbloom,
            TransformativeType::Crystallize => ReactionType::Crystallize,
        }
    }
    pub fn get_reaction_from_elevative_type(elevative_type: ElevativeReaction) -> ReactionType {
        match elevative_type {
            ElevativeReaction::LunarChargedReaction | ElevativeReaction::LunarCharged => ReactionType::LunarCharged,
            ElevativeReaction::LunarBloom => ReactionType::LunarBloom,
            ElevativeReaction::LunarCrystallizeReaction | ElevativeReaction::LunarCrystallize => ReactionType::LunarCrystallize,
            ElevativeReaction::StellarConductCryo | ElevativeReaction::StellarConductElectro => ReactionType::StellarConduct,
            ElevativeReaction::StellarSwirlReactionAnemo | ElevativeReaction::StellarSwirlReactionCryo
            | ElevativeReaction::StellarSwirlAnemo | ElevativeReaction::StellarSwirlCryo => ReactionType::StellarSwirl,
        }
    }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone)]
pub enum AmplifyingType {
    Melt(Element),
    Vaporize(Element)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stellar_swirl_reaction() {
        // 星扩散反应风伤：基础倍率 0.75，反应基伤取等级系数，风元素
        assert_eq!(ElevativeReaction::StellarSwirlReactionAnemo.get_reaction_coefficient(), 0.75);
        assert_eq!(ElevativeReaction::StellarSwirlReactionAnemo.get_reaction_base(90), LEVEL_MULTIPLIER[89]);
        assert_eq!(ElevativeReaction::StellarSwirlReactionAnemo.get_element(), Some(Element::Anemo));

        // 星扩散反应冰伤：基础倍率 2.0（随风涡系数变化，见额外基础倍率），反应基伤取等级系数，冰元素
        assert_eq!(ElevativeReaction::StellarSwirlReactionCryo.get_reaction_coefficient(), 2.0);
        assert_eq!(ElevativeReaction::StellarSwirlReactionCryo.get_reaction_base(90), LEVEL_MULTIPLIER[89]);
        assert_eq!(ElevativeReaction::StellarSwirlReactionCryo.get_element(), Some(Element::Cryo));

        // 视为星扩散伤害的风伤/冰伤：基础倍率 1.0，无反应基伤
        assert_eq!(ElevativeReaction::StellarSwirlAnemo.get_reaction_coefficient(), 1.0);
        assert_eq!(ElevativeReaction::StellarSwirlAnemo.get_reaction_base(90), 0.0);
        assert_eq!(ElevativeReaction::StellarSwirlAnemo.get_element(), Some(Element::Anemo));
        assert_eq!(ElevativeReaction::StellarSwirlCryo.get_reaction_coefficient(), 1.0);
        assert_eq!(ElevativeReaction::StellarSwirlCryo.get_reaction_base(90), 0.0);
        assert_eq!(ElevativeReaction::StellarSwirlCryo.get_element(), Some(Element::Cryo));

        // 风涡系数 0~6 时的额外基础倍率：1~2 为 0.0（总倍率 2.0），3~6 为 0.5（总倍率 3.0）
        let extra = ElevativeReaction::STELLAR_SWIRL_CRYO_EXTRA_COEFFICIENT;
        assert_eq!(extra[0], 0.0);
        assert_eq!(extra[1], 0.0);
        assert_eq!(extra[2], 0.0);
        assert_eq!(extra[3], 0.5);
        assert_eq!(extra[4], 0.5);
        assert_eq!(extra[5], 0.5);
        assert_eq!(extra[6], 0.5);

        // 反应类型映射
        assert_eq!(
            ReactionType::get_reaction_from_elevative_type(ElevativeReaction::StellarSwirlReactionAnemo),
            ReactionType::StellarSwirl,
        );
        assert_eq!(
            ReactionType::get_reaction_from_elevative_type(ElevativeReaction::StellarSwirlReactionCryo),
            ReactionType::StellarSwirl,
        );
        assert_eq!(
            ReactionType::get_reaction_from_elevative_type(ElevativeReaction::StellarSwirlAnemo),
            ReactionType::StellarSwirl,
        );
        assert_eq!(
            ReactionType::get_reaction_from_elevative_type(ElevativeReaction::StellarSwirlCryo),
            ReactionType::StellarSwirl,
        );
    }
}
