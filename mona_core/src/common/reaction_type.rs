use serde::{Serialize, Deserialize};
use crate::common::Element;
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
}

impl ElevativeReaction {
    pub fn get_element(&self) -> Option<Element> {
        match *self {
            ElevativeReaction::LunarChargedReaction | ElevativeReaction::LunarCharged => Some(Element::Electro),
            ElevativeReaction::LunarBloom => Some(Element::Dendro),
            ElevativeReaction::LunarCrystallizeReaction | ElevativeReaction::LunarCrystallize => Some(Element::Geo),
            _ => None
        }
    }

    pub fn get_reaction_base(&self, character_level: usize) -> f64 {
        match *self {
            ElevativeReaction::LunarChargedReaction | ElevativeReaction::LunarCrystallizeReaction => LEVEL_MULTIPLIER[character_level - 1],
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
        }
    }

    pub const STELLAR_CONDUCT_EXTRA_COEFFICIENT: [f64; 13] = [0.0, 0.45, 0.5, 0.54, 0.6, 0.64, 0.71, 0.75, 0.79, 0.85, 0.89, 0.95, 1.0];
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
        }
    }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone)]
pub enum AmplifyingType {
    Melt(Element),
    Vaporize(Element)
}
