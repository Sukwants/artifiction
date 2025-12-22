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
pub enum MoonglareReaction {
    LunarChargedReaction,   // 月感电
    LunarCharged,           // 广义月感电
    LunarBloom,
    None,
}

impl MoonglareReaction {
    pub fn get_element(&self) -> Option<Element> {
        match *self {
            MoonglareReaction::LunarChargedReaction | MoonglareReaction::LunarCharged => Some(Element::Electro),
            MoonglareReaction::LunarBloom => Some(Element::Dendro),
            _ => None
        }
    }

    pub fn get_reaction_coefficient(&self) -> f64 {
        match *self {
            MoonglareReaction::LunarChargedReaction => 1.8,
            MoonglareReaction::LunarCharged => 3.0,
            MoonglareReaction::LunarBloom => 1.0,
            _ => panic!()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone)]
pub enum ReactionType {
    Melt,
    Vaporize,
    Spread,
    Aggravate,
    Swirl,
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
}

impl ReactionType {
    pub fn get_reaction_from_transformative_type(transformative_type: TransformativeType) -> ReactionType {
        match transformative_type {
            TransformativeType::SwirlCryo | TransformativeType::SwirlPyro | TransformativeType::SwirlHydro | TransformativeType::SwirlElectro => ReactionType::Swirl,
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
    pub fn get_reaction_from_lunar_type(lunar_type: MoonglareReaction) -> Option<ReactionType> {
        match lunar_type {
            MoonglareReaction::LunarChargedReaction | MoonglareReaction::LunarCharged => Some(ReactionType::LunarCharged),
            MoonglareReaction::LunarBloom => Some(ReactionType::LunarBloom),
            MoonglareReaction::None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone)]
pub enum AmplifyingType {
    Melt(Element),
    Vaporize(Element)
}
